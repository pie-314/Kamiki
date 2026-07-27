#![allow(unused_imports)]

use dioxus::prelude::*;
use crate::data::models::{FlowData, InterfaceInfo, PacketEvent, SystemStats};

#[cfg(not(target_arch = "wasm32"))]
mod engine_state {
    use kamiki_core::{Kamiki, collector::CollectorConfig};
    use std::sync::{Arc, Mutex, OnceLock};

    static ENGINE: OnceLock<Arc<Mutex<Option<Kamiki>>>> = OnceLock::new();
    static ACTIVE_IFACE: OnceLock<Arc<Mutex<String>>> = OnceLock::new();

    pub fn get_engine() -> &'static Arc<Mutex<Option<Kamiki>>> {
        ENGINE.get_or_init(|| Arc::new(Mutex::new(None)))
    }

    pub fn get_iface() -> &'static Arc<Mutex<String>> {
        ACTIVE_IFACE.get_or_init(|| {
            let iface = std::env::var("KAMIKI_INTERFACE").unwrap_or_else(|_| "eth0".into());
            Arc::new(Mutex::new(iface))
        })
    }
}

/// Start packet capture engine on the specified network interface.
#[server]
pub async fn start_capture(interface: String) -> Result<String, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use engine_state::{get_engine, get_iface};
        use kamiki_core::{Kamiki, collector::CollectorConfig};

        let mut engine_lock = get_engine().lock().unwrap();

        // Stop existing engine if running
        if let Some(old_engine) = engine_lock.take() {
            old_engine.stop();
        }

        let config = CollectorConfig {
            interface: interface.clone(),
            object_path: "kamiki-ebpf/out/xdp_prober.bpf.o".into(),
            ..Default::default()
        };

        match Kamiki::start(config) {
            Ok(new_engine) => {
                *engine_lock = Some(new_engine);
                *get_iface().lock().unwrap() = interface.clone();
                Ok(format!("Started eBPF capture on {}", interface))
            }
            Err(err) => Err(ServerFnError::new(format!("Failed to start Kamiki engine: {err}"))),
        }
    }

    #[cfg(target_arch = "wasm32")]
    Ok("Capturing started".into())
}

/// Stop the running packet capture engine.
#[server]
pub async fn stop_capture() -> Result<(), ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use engine_state::get_engine;
        let mut engine_lock = get_engine().lock().unwrap();
        if let Some(engine) = engine_lock.take() {
            engine.stop();
        }
    }
    Ok(())
}

/// Poll live decoded network events from the eBPF event channel.
#[server]
pub async fn poll_packets(limit: usize) -> Result<Vec<PacketEvent>, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use engine_state::get_engine;
        let engine_lock = get_engine().lock().unwrap();
        let mut batch = Vec::new();

        if let Some(ref kamiki) = *engine_lock {
            while let Ok(event) = kamiki.events.try_recv() {
                let process_name = event
                    .process
                    .as_ref()
                    .map(|p| p.comm.clone())
                    .unwrap_or_else(|| "—".into());
                let pid = event.process.as_ref().map(|p| p.pid).unwrap_or(0);

                batch.push(PacketEvent {
                    src_ip: event.src_ip.to_string(),
                    dst_ip: event.dst_ip.to_string(),
                    src_port: event.src_port,
                    dst_port: event.dst_port,
                    protocol: event.protocol.to_string(),
                    pkt_len: event.pkt_len,
                    timestamp: format!("{:?}", event.timestamp),
                    process_name,
                    pid,
                });

                if batch.len() >= limit {
                    break;
                }
            }
        }

        Ok(batch)
    }

    #[cfg(target_arch = "wasm32")]
    Ok(Vec::new())
}

/// Fetch active 5-tuple network flows tracked by Kamiki's FlowTable.
#[server]
pub async fn get_flows() -> Result<Vec<FlowData>, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use engine_state::get_engine;
        let engine_lock = get_engine().lock().unwrap();
        let mut flows_list = Vec::new();

        if let Some(ref kamiki) = *engine_lock {
            for entry in kamiki.flows.iter() {
                let val = entry.value();
                flows_list.push(FlowData {
                    src_ip: val.key.src_ip.to_string(),
                    dst_ip: val.key.dst_ip.to_string(),
                    src_port: val.key.src_port,
                    dst_port: val.key.dst_port,
                    protocol: val.key.protocol.to_string(),
                    packets: val.packets,
                    bytes: val.bytes,
                });
            }
        }

        Ok(flows_list)
    }

    #[cfg(target_arch = "wasm32")]
    Ok(Vec::new())
}

/// Fetch system-level capture statistics and kernel telemetry.
#[server]
pub async fn get_stats() -> Result<SystemStats, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use engine_state::{get_engine, get_iface};
        let engine_lock = get_engine().lock().unwrap();

        let mut total_pkts = 0u64;
        let mut total_bytes = 0u64;
        let mut active_flows = 0u32;

        if let Some(ref kamiki) = *engine_lock {
            active_flows = kamiki.flows.len() as u32;
            for entry in kamiki.flows.iter() {
                total_pkts += entry.value().packets;
                total_bytes += entry.value().bytes;
            }
        }

        let interface = get_iface().lock().unwrap().clone();
        let kernel = std::fs::read_to_string("/proc/version")
            .unwrap_or_else(|_| "Linux".into())
            .split_whitespace()
            .take(3)
            .collect::<Vec<_>>()
            .join(" ");

        Ok(SystemStats {
            total_pkts,
            total_bytes,
            active_flows,
            interface,
            kernel,
        })
    }

    #[cfg(target_arch = "wasm32")]
    Ok(SystemStats::default())
}

/// Enumerate network interfaces present in `/sys/class/net/`.
#[server]
pub async fn get_interfaces() -> Result<Vec<InterfaceInfo>, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::fs;
        use std::path::Path;

        let mut ifaces = Vec::new();
        if let Ok(entries) = fs::read_dir("/sys/class/net/") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let operstate_path = format!("/sys/class/net/{}/operstate", name);
                let active = fs::read_to_string(&operstate_path)
                    .map(|s| s.trim() == "up" || s.trim() == "unknown")
                    .unwrap_or(false);

                let speed_path = format!("/sys/class/net/{}/speed", name);
                let speed = fs::read_to_string(&speed_path)
                    .ok()
                    .and_then(|s| s.trim().parse::<i32>().ok())
                    .map(|mbps| if mbps >= 1000 { format!("{} Gbps", mbps / 1000) } else { format!("{} Mbps", mbps) })
                    .unwrap_or_else(|| "—".into());

                ifaces.push(InterfaceInfo { name, speed, active });
            }
        }

        // Fallback default list if /sys/class/net is unreadable
        if ifaces.is_empty() {
            ifaces = vec![
                InterfaceInfo { name: "eth0".into(), speed: "10 Gbps".into(), active: true },
                InterfaceInfo { name: "wlan0".into(), speed: "866 Mbps".into(), active: true },
                InterfaceInfo { name: "lo".into(), speed: "—".into(), active: false },
            ];
        }

        Ok(ifaces)
    }

    #[cfg(target_arch = "wasm32")]
    Ok(Vec::new())
}
