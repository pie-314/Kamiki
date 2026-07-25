use crate::error::Error;
use crate::event::NetworkEvent;
use crossbeam_channel::{Receiver, Sender, bounded};
use log::{error, info};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct CollectorConfig {
    pub interface: String,
    pub object_path: String,
    pub channel_capacity: usize,
    pub poll_timeout_ms: u64,
}

impl Default for CollectorConfig {
    fn default() -> Self {
        Self {
            interface: "eth0".into(),
            object_path: "kamiki-ebpf/out/xdp_prober.bpf.o".into(),
            channel_capacity: 8192,
            poll_timeout_ms: 100,
        }
    }
}

pub struct Collector {
    config: CollectorConfig,
    shutdown: Arc<AtomicBool>,
    tx: Sender<NetworkEvent>,
    rx: Receiver<NetworkEvent>,
}

impl Collector {
    pub fn new(config: CollectorConfig) -> Self {
        let (tx, rx) = bounded(config.channel_capacity);
        Self {
            config,
            shutdown: Arc::new(AtomicBool::new(false)),
            tx,
            rx,
        }
    }

    pub fn events(&self) -> Receiver<NetworkEvent> {
        self.rx.clone()
    }

    pub fn spawn(self) -> Result<CollectorHandle, Error> {
        let shutdown = Arc::clone(&self.shutdown);
        let shutdown_thread = Arc::clone(&self.shutdown);
        let tx = self.tx.clone();
        let config = self.config;

        let thread = std::thread::Builder::new()
            .name("kamiki-collector".into())
            .spawn(move || {
                if let Err(e) = run_collector_loop(&config, &tx, &shutdown_thread) {
                    error!("collector: {e}");
                } else {
                    info!("collector stopped cleanly");
                }
            })
            .map_err(Error::Io)?;

        Ok(CollectorHandle {
            thread: Some(thread),
            shutdown,
        })
    }
}

pub struct CollectorHandle {
    thread: Option<std::thread::JoinHandle<()>>,
    shutdown: Arc<AtomicBool>,
}

impl CollectorHandle {
    pub fn stop(mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

impl Drop for CollectorHandle {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
    }
}

#[cfg_attr(not(target_os = "linux"), allow(unused_variables))]
fn run_collector_loop(
    config: &CollectorConfig,
    tx: &Sender<NetworkEvent>,
    shutdown: &AtomicBool,
) -> Result<(), Error> {
    #[cfg(not(target_os = "linux"))]
    return Err(Error::UnsupportedPlatform);

    #[cfg(target_os = "linux")]
    {
        use crate::event::RawPktEvent;
        use libbpf_rs::{MapCore, ObjectBuilder, RingBufferBuilder};
        use std::ffi::OsStr;
        use std::mem::size_of;
        use std::time::Duration;

        if !std::path::Path::new(&config.object_path).exists() {
            return Err(Error::ObjectNotFound(config.object_path.clone()));
        }

        // Silence libbpf's default stderr so it doesn't corrupt the TUI
        libbpf_rs::set_print(None);

        let ifindex = get_ifindex(&config.interface)?;

        // 1. Open and load the eBPF object - runs the BPF verifier
        let open_obj = ObjectBuilder::default().open_file(&config.object_path)?;
        let obj = open_obj.load()?;

        // 2. Attach XDP to the interface.
        //    _link must stay alive for the duration - dropping it detaches XDP.
        let _link = {
            let prog = obj
                .progs_mut()
                .find(|p| p.name() == OsStr::new("xdp_prober"))
                .ok_or_else(|| Error::Other("program 'xdp_prober' not found in .bpf.o".into()))?;
            prog.attach_xdp(ifindex as i32)?
        };

        info!("XDP attached to {} (ifindex {})", config.interface, ifindex);

        // 3. Find the ring buffer map and wire up the reader
        let map = obj
            .maps()
            .find(|m| m.name() == OsStr::new("ringbuf"))
            .ok_or_else(|| Error::Other("map 'ringbuf' not found in .bpf.o".into()))?;

        let tx_clone = tx.clone();
        let mut rb_builder = RingBufferBuilder::new();
        rb_builder.add(&map, move |data: &[u8]| -> i32 {
            if data.len() < size_of::<RawPktEvent>() {
                return 0;
            }
            // SAFETY: kernel wrote sizeof(pkt_event) bytes; repr(C) matches exactly
            let raw: RawPktEvent =
                unsafe { std::ptr::read_unaligned(data.as_ptr() as *const RawPktEvent) };
            let _ = tx_clone.try_send(raw.decode());
            0
        })?;
        let rb = rb_builder.build()?;

        // 4. Poll until shutdown
        info!("ring buffer polling started");
        while !shutdown.load(Ordering::Relaxed) {
            if let Err(e) = rb.poll(Duration::from_millis(config.poll_timeout_ms)) {
                log::debug!("poll: {e}");
            }
        }

        // _link drops here → XDP detached automatically
        info!("XDP detached from {}", config.interface);
        Ok(())
    }
}

/// Read the interface index from sysfs — avoids a libc dependency.
#[cfg(target_os = "linux")]
fn get_ifindex(interface: &str) -> Result<u32, Error> {
    let path = format!("/sys/class/net/{}/ifindex", interface);
    std::fs::read_to_string(&path)
        .map_err(|_| Error::InterfaceNotFound(interface.to_string()))?
        .trim()
        .parse::<u32>()
        .map_err(|e| Error::Other(format!("bad ifindex in {path}: {e}")))
}
