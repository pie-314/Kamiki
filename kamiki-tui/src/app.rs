use crossterm::event::{KeyCode, KeyEvent};
use kamiki_core::{
    Kamiki,
    event::{NetworkEvent, Protocol},
    filter::Filter,
    flow::{FlowEntry, FlowKey, update_flow},
};
use std::collections::VecDeque;

const MAX_PACKET_LOG: usize = 200;

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub total_pkts: u64,
    pub total_bytes: u64,
    pub tcp_pkts: u64,
    pub udp_pkts: u64,
    pub icmp_pkts: u64,
}

pub struct App {
    pub kamiki: Kamiki,
    pub should_quit: bool,
    pub filter: Filter,
    pub packet_log: VecDeque<NetworkEvent>,
    pub flow_snapshot: Vec<(FlowKey, FlowEntry)>,
    pub selected_row: usize,
    pub stats: Stats,
    pub interface: String,
    pub is_running: bool,
}

impl App {
    pub fn new(kamiki: Kamiki, interface: String) -> Self {
        Self {
            kamiki,
            should_quit: false,
            filter: Filter::default(),
            packet_log: VecDeque::with_capacity(MAX_PACKET_LOG),
            flow_snapshot: Vec::new(),
            selected_row: 0,
            stats: Stats::default(),
            interface,
            is_running: true,
        }
    }

    pub fn tick(&mut self) {
        let mut count = 0;
        while let Ok(event) = self.kamiki.events.try_recv() {
            if !self.filter.matches(&event) {
                continue;
            }

            self.stats.total_pkts += 1;
            self.stats.total_bytes += event.pkt_len as u64;
            match event.protocol {
                Protocol::Tcp => self.stats.tcp_pkts += 1,
                Protocol::Udp => self.stats.udp_pkts += 1,
                Protocol::Icmp => self.stats.icmp_pkts += 1,
                _ => {}
            }

            update_flow(&self.kamiki.flows, &event);

            if self.packet_log.len() >= MAX_PACKET_LOG {
                self.packet_log.pop_back();
            }
            self.packet_log.push_front(event);

            count += 1;
            if count >= 256 {
                break;
            }
        }

        self.flow_snapshot = self
            .kamiki
            .flows
            .iter()
            .map(|e| (e.key().clone(), e.value().clone()))
            .collect();
        self.flow_snapshot
            .sort_by_key(|b| std::cmp::Reverse(b.1.bytes));
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') if !self.packet_log.is_empty() => {
                self.selected_row =
                    (self.selected_row + 1).min(self.packet_log.len().saturating_sub(1));
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.selected_row = self.selected_row.saturating_sub(1);
            }
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }

    pub fn shutdown(self) {
        self.kamiki.stop();
    }
}
