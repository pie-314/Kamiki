use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::Ipv4Addr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub tgid: u32,
    pub uid: u32,
    pub comm: String,
}
pub type SocketMap = HashMap<u64, ProcessInfo>;

pub fn lookup_process(_src_ip: Ipv4Addr, _src_port: u16) -> Option<ProcessInfo> {
    // TODO Phase 4
    None
}

pub fn build_socket_map() -> SocketMap {
    // TODO Phase 4
    HashMap::new()
}

#[allow(dead_code)]
fn parse_proc_net_tcp_line(_line: &str) -> Option<(Ipv4Addr, u16, u64)> {
    // TODO Phase 4
    None
}
