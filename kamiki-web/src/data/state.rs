use crate::data::models::{
    CaptureState, FlowData, InterfaceInfo, PacketEvent, ProtocolCount, SystemStats, TrafficSample,
};
use dioxus::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavView {
    Dashboard,
    Connections,
    Packets,
    Processes,
    Events,
    Timeline,
}

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
    pub active_view: Signal<NavView>,
    pub selected_filter: Signal<Option<String>>,
    pub search_query: Signal<String>,
}

impl AppState {
    pub fn selected_packet(&self) -> Option<PacketEvent> {
        let idx = (*self.selected_packet_idx.read())?;
        let pkts = self.packets.read();
        pkts.get(idx).cloned()
    }
}
