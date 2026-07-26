use dioxus::prelude::*;
use std::collections::VecDeque;
use crate::data::models::{CaptureState, FlowData, InterfaceInfo, PacketEvent, ProtocolCount, SystemStats, TrafficSample};

#[derive(Clone, Copy)]
pub struct AppState {
    pub capture: Signal<CaptureState>,
    pub packets: Signal<Vec<PacketEvent>>,
    pub selected_packet_idx: Signal<Option<usize>>,
    pub flows: Signal<Vec<FlowData>>,
    pub stats: Signal<SystemStats>,
    pub interfaces: Signal<Vec<InterfaceInfo>>,
    pub protocol_counts: Signal<Vec<ProtocolCount>>,
    pub traffic_history: Signal<VecDeque<TrafficSample>>,
}

impl AppState {
    pub fn new() -> Self {
        let initial_interfaces = vec![
            InterfaceInfo { name: "eth0".into(), speed: "10 Gbps".into(), active: true },
            InterfaceInfo { name: "wlan0".into(), speed: "866 Mbps".into(), active: true },
            InterfaceInfo { name: "lo".into(), speed: "—".into(), active: false },
        ];

        let initial_filters = vec![
            ProtocolCount { label: "TCP".into(), count: 0, color_class: "bg-blue-500".into() },
            ProtocolCount { label: "UDP".into(), count: 0, color_class: "bg-purple-500".into() },
            ProtocolCount { label: "ICMP".into(), count: 0, color_class: "bg-yellow-500".into() },
            ProtocolCount { label: "DNS".into(), count: 0, color_class: "bg-orange-500".into() },
            ProtocolCount { label: "TLS".into(), count: 0, color_class: "bg-cyan-500".into() },
            ProtocolCount { label: "Other".into(), count: 0, color_class: "bg-gray-500".into() },
        ];

        let mut history = VecDeque::new();
        for _ in 0..60 {
            history.push_back(TrafficSample::default());
        }

        Self {
            capture: use_signal(CaptureState::default),
            packets: use_signal(Vec::new),
            selected_packet_idx: use_signal(|| None),
            flows: use_signal(Vec::new),
            stats: use_signal(SystemStats::default),
            interfaces: use_signal(|| initial_interfaces),
            protocol_counts: use_signal(|| initial_filters),
            traffic_history: use_signal(|| history),
        }
    }

    pub fn selected_packet(&self) -> Option<PacketEvent> {
        let idx = (*self.selected_packet_idx.read())?;
        let pkts = self.packets.read();
        pkts.get(idx).cloned()
    }
}
