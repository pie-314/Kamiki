use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::time::SystemTime;

/// Raw event layout as written by the eBPF program into the ring buffer.
///
/// IMPORTANT: this repr(C) struct must stay byte-for-byte in sync with
/// `struct pkt_event` in kamiki-ebpf/src/xdp_prober.bpf.c.
/// Field order, sizes, and alignment must match exactly.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RawPktEvent {
    pub src_ip: u32,
    pub dst_ip: u32,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: u8,  // IPPROTO_TCP=6, IPPROTO_UDP=17, IPPROTO_ICMP=1
    pub _pad: [u8; 3], // explicit padding to match C struct alignment
    pub pkt_len: u32,
}

/// Decoded, host-endian network event ready for use in userspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEvent {
    pub src_ip: Ipv4Addr,
    pub dst_ip: Ipv4Addr,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: Protocol,
    pub pkt_len: u32,
    pub timestamp: SystemTime,

    pub process: Option<crate::process::ProcessInfo>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
    Unknown(u8),
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Tcp => write!(f, "TCP"),
            Protocol::Udp => write!(f, "UDP"),
            Protocol::Icmp => write!(f, "ICMP"),
            Protocol::Unknown(n) => write!(f, "PROTO({n})"),
        }
    }
}

impl From<u8> for Protocol {
    fn from(v: u8) -> Self {
        match v {
            6 => Protocol::Tcp,
            17 => Protocol::Udp,
            1 => Protocol::Icmp,
            n => Protocol::Unknown(n),
        }
    }
}

impl RawPktEvent {
    pub fn decode(self) -> NetworkEvent {
        NetworkEvent {
            src_ip: Ipv4Addr::from(self.src_ip),
            dst_ip: Ipv4Addr::from(self.dst_ip),
            src_port: self.src_port,
            dst_port: self.dst_port,
            protocol: Protocol::from(self.protocol),
            pkt_len: self.pkt_len,
            timestamp: SystemTime::now(),
            process: None,
        }
    }
}

/// Compile-time guard: struct size must match the C definition exactly.
/// If this fails, sync RawPktEvent fields with struct pkt_event in xdp_prober.bpf.c.
const _SIZE_CHECK: () = assert!(std::mem::size_of::<RawPktEvent>() == 20);
