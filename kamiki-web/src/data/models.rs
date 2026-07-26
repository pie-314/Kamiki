use serde::{Deserialize, Serialize};

/// Mirrors kamiki_core::event::NetworkEvent, serialized across the boundary
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PacketEvent {
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: String,
    pub pkt_len: u32,
    pub timestamp: String,
    pub process_name: String,
    pub pid: u32,
}

/// Aggregated flow entry from kamiki_core::flow::FlowTable
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FlowData {
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: String,
    pub packets: u64,
    pub bytes: u64,
}

/// System-level aggregate stats
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct SystemStats {
    pub total_pkts: u64,
    pub total_bytes: u64,
    pub active_flows: u32,
    pub interface: String,
    pub kernel: String,
}

/// Network interface info (read from /sys/class/net/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InterfaceInfo {
    pub name: String,
    pub speed: String,
    pub active: bool,
}

/// Live capture state tracked on the client
#[derive(Clone, Debug, PartialEq)]
pub struct CaptureState {
    pub interface: Option<String>,
    pub is_live: bool,
    pub uptime_secs: u64,
    pub total_events: u64,
    pub dropped: u64,
}

impl Default for CaptureState {
    fn default() -> Self {
        Self {
            interface: Some("eth0".into()),
            is_live: false,
            uptime_secs: 0,
            total_events: 0,
            dropped: 0,
        }
    }
}

impl CaptureState {
    pub fn formatted_uptime(&self) -> String {
        let hrs = self.uptime_secs / 3600;
        let mins = (self.uptime_secs % 3600) / 60;
        let secs = self.uptime_secs % 60;
        format!("{:02}:{:02}:{:02}", hrs, mins, secs)
    }
}

/// Rolling traffic sample for bandwidth and connection charts
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TrafficSample {
    pub bytes_in_window: u64,
    pub packets_in_window: u64,
    pub active_flows: u32,
}

/// Protocol count for sidebar filter badges
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProtocolCount {
    pub label: String,
    pub count: u32,
    pub color_class: String,
}
