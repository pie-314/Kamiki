#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(target_os = "linux")]
    #[error("eBPF error: {0}")]
    Ebpf(#[from] libbpf_rs::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("event channel closed unexpectedly")]
    ChannelClosed,

    #[error("interface not found: {0}")]
    InterfaceNotFound(String),

    #[error("eBPF object not found at path: {0}")]
    ObjectNotFound(String),

    #[error("eBPF capture is only supported on Linux")]
    UnsupportedPlatform,

    #[error("{0}")]
    Other(String),
}
