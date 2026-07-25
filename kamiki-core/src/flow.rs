use crate::event::{NetworkEvent, Protocol};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FlowKey {
    pub src_ip: Ipv4Addr,
    pub dst_ip: Ipv4Addr,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: Protocol,
}

impl FlowKey {
    pub fn normalized(mut self) -> Self {
        if (self.src_ip, self.src_port) > (self.dst_ip, self.dst_port) {
            std::mem::swap(&mut self.src_ip, &mut self.dst_ip);
            std::mem::swap(&mut self.src_port, &mut self.dst_port);
        }
        self
    }
}

#[derive(Debug, Clone)]
pub struct FlowEntry {
    pub key: FlowKey,
    pub first_seen: Instant,
    pub last_seen: Instant,
    pub packets: u64,
    pub bytes: u64,
}

impl FlowEntry {
    pub fn duration(&self) -> Duration {
        self.last_seen.duration_since(self.first_seen)
    }

    pub fn is_idle(&self, idle_timeout: Duration) -> bool {
        self.last_seen.elapsed() > idle_timeout
    }
}
pub type FlowTable = DashMap<FlowKey, FlowEntry>;

pub fn update_flow(table: &FlowTable, event: &NetworkEvent) {
    let key = FlowKey {
        src_ip: event.src_ip,
        dst_ip: event.dst_ip,
        src_port: event.src_port,
        dst_port: event.dst_port,
        protocol: event.protocol,
    }
    .normalized();

    let now = Instant::now();
    table
        .entry(key.clone())
        .and_modify(|e| {
            e.last_seen = now;
            e.packets += 1;
            e.bytes += event.pkt_len as u64;
        })
        .or_insert(FlowEntry {
            key,
            first_seen: now,
            last_seen: now,
            packets: 1,
            bytes: event.pkt_len as u64,
        });
}

pub fn evict_idle(table: &FlowTable, timeout: Duration) {
    table.retain(|_, v| !v.is_idle(timeout));
}
