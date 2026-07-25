use crate::event::{NetworkEvent, Protocol};
use std::net::Ipv4Addr;

#[derive(Debug, Clone, Default)]
pub struct Filter {
    pub protocol: Option<Protocol>,
    pub src_ip: Option<Ipv4Addr>,
    pub dst_ip: Option<Ipv4Addr>,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
}

impl Filter {
    pub fn matches(&self, event: &NetworkEvent) -> bool {
        if let Some(p) = self.protocol
            && event.protocol != p
        {
            return false;
        }
        if let Some(ip) = self.src_ip
            && event.src_ip != ip
        {
            return false;
        }
        if let Some(ip) = self.dst_ip
            && event.dst_ip != ip
        {
            return false;
        }
        if let Some(port) = self.src_port
            && event.src_port != port
        {
            return false;
        }
        if let Some(port) = self.dst_port
            && event.dst_port != port
        {
            return false;
        }
        if let Some(pid) = self.pid
            && event.process.as_ref().map(|p| p.pid) != Some(pid)
        {
            return false;
        }
        if let Some(ref name) = self.process_name {
            let matches = event
                .process
                .as_ref()
                .map(|p| p.comm.contains(name.as_str()))
                .unwrap_or(false);
            if !matches {
                return false;
            }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.protocol.is_none()
            && self.src_ip.is_none()
            && self.dst_ip.is_none()
            && self.src_port.is_none()
            && self.dst_port.is_none()
            && self.pid.is_none()
            && self.process_name.is_none()
    }
}
