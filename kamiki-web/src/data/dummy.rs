#![allow(dead_code)]

// Dummy Data Structures & Mock Data for Kamiki UI

#[derive(Clone, Debug, PartialEq)]
pub struct ProcessInfo {
    pub name: &'static str,
    pub pid: u32,
    pub connections: u32,
    pub sent: &'static str,
    pub received: &'static str,
    pub total: &'static str,
    pub top_remote: &'static str,
    pub icon_color: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PacketEvent {
    pub time: &'static str,
    pub process: &'static str,
    pub pid: u32,
    pub source: &'static str,
    pub destination: &'static str,
    pub protocol: &'static str,
    pub info: &'static str,
    pub size: u32,
    pub is_selected: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilterCount {
    pub label: &'static str,
    pub count: u32,
    pub color_class: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceInfo {
    pub name: &'static str,
    pub speed: &'static str,
    pub active: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PacketDetail {
    pub timestamp: &'static str,
    pub process: &'static str,
    pub pid: u32,
    pub source: &'static str,
    pub destination: &'static str,
    pub protocol: &'static str,
    pub length: u32,
    pub interface: &'static str,
    pub direction: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CaptureState {
    pub uptime: &'static str,
    pub events: u32,
    pub dropped: u32,
    pub is_live: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SystemMetrics {
    pub packets: u32,
    pub bytes: &'static str,
    pub flows: u32,
    pub connections: u32,
    pub cpu: &'static str,
    pub memory: &'static str,
    pub interface: &'static str,
    pub kernel: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HexLine {
    pub offset: &'static str,
    pub hex: &'static str,
    pub ascii: &'static str,
}

// Data Provider Functions

pub fn get_processes() -> Vec<ProcessInfo> {
    vec![
        ProcessInfo {
            name: "firefox",
            pid: 8421,
            connections: 45,
            sent: "1.4 MB",
            received: "8.7 MB",
            total: "10.1 MB",
            top_remote: "142.250.74.206:443",
            icon_color: "text-orange-500",
        },
        ProcessInfo {
            name: "discord",
            pid: 9122,
            connections: 26,
            sent: "532 KB",
            received: "310 KB",
            total: "842 KB",
            top_remote: "162.159.130.232:443",
            icon_color: "text-indigo-400",
        },
        ProcessInfo {
            name: "ssh",
            pid: 2231,
            connections: 3,
            sent: "12 KB",
            received: "12 KB",
            total: "24 KB",
            top_remote: "192.168.1.10:22",
            icon_color: "text-gray-400",
        },
        ProcessInfo {
            name: "spotify",
            pid: 7721,
            connections: 8,
            sent: "210 KB",
            received: "320 KB",
            total: "530 KB",
            top_remote: "35.186.224.24:443",
            icon_color: "text-green-500",
        },
        ProcessInfo {
            name: "curl",
            pid: 9912,
            connections: 1,
            sent: "4 KB",
            received: "4 KB",
            total: "8 KB",
            top_remote: "142.250.189.78:443",
            icon_color: "text-blue-400",
        },
        ProcessInfo {
            name: "systemd-resolve",
            pid: 1452,
            connections: 2,
            sent: "2 KB",
            received: "6 KB",
            total: "8 KB",
            top_remote: "1.1.1.1:53",
            icon_color: "text-cyan-400",
        },
    ]
}

pub fn get_packets() -> Vec<PacketEvent> {
    vec![
        PacketEvent {
            time: "14:35:21.123456",
            process: "firefox",
            pid: 8421,
            source: "192.168.1.4:49231",
            destination: "142.250.74.206:443",
            protocol: "TCP",
            info: "TLSv1.3 Application Data",
            size: 1448,
            is_selected: true,
        },
        PacketEvent {
            time: "14:35:21.123789",
            process: "firefox",
            pid: 8421,
            source: "142.250.74.206:443",
            destination: "192.168.1.4:49231",
            protocol: "TCP",
            info: "TLSv1.3 Application Data",
            size: 1448,
            is_selected: false,
        },
        PacketEvent {
            time: "14:35:21.124001",
            process: "discord",
            pid: 9122,
            source: "192.168.1.4:53712",
            destination: "162.159.130.232:443",
            protocol: "TCP",
            info: "TLSv1.3 Application Data",
            size: 1240,
            is_selected: false,
        },
        PacketEvent {
            time: "14:35:21.124300",
            process: "ssh",
            pid: 2231,
            source: "192.168.1.4:54222",
            destination: "192.168.1.10:22",
            protocol: "TCP",
            info: "SSH Protocol Data",
            size: 512,
            is_selected: false,
        },
        PacketEvent {
            time: "14:35:21.124567",
            process: "systemd-resolve",
            pid: 1452,
            source: "192.168.1.4:53",
            destination: "1.1.1.1:53",
            protocol: "UDP",
            info: "Standard query 0x1a2b A example.com",
            size: 78,
            is_selected: false,
        },
        PacketEvent {
            time: "14:35:21.124890",
            process: "systemd-resolve",
            pid: 1452,
            source: "1.1.1.1:53",
            destination: "192.168.1.4:53",
            protocol: "UDP",
            info: "Standard query response 0x1a2b A ...",
            size: 110,
            is_selected: false,
        },
        PacketEvent {
            time: "14:35:21.125200",
            process: "firefox",
            pid: 8421,
            source: "192.168.1.4:49231",
            destination: "142.250.74.206:443",
            protocol: "TCP",
            info: "ACK",
            size: 66,
            is_selected: false,
        },
        PacketEvent {
            time: "14:35:21.125500",
            process: "discord",
            pid: 9122,
            source: "192.168.1.4:53712",
            destination: "162.159.130.232:443",
            protocol: "TCP",
            info: "ACK",
            size: 66,
            is_selected: false,
        },
        PacketEvent {
            time: "14:35:21.125800",
            process: "spotify",
            pid: 7721,
            source: "192.168.1.4:59821",
            destination: "35.186.224.24:443",
            protocol: "TCP",
            info: "TLSv1.3 Application Data",
            size: 1448,
            is_selected: false,
        },
        PacketEvent {
            time: "14:35:21.126100",
            process: "curl",
            pid: 9912,
            source: "192.168.1.4:60211",
            destination: "142.250.189.78:443",
            protocol: "TCP",
            info: "TLSv1.3 Client Hello",
            size: 517,
            is_selected: false,
        },
    ]
}

pub fn get_filters() -> Vec<FilterCount> {
    vec![
        FilterCount { label: "TCP", count: 1032, color_class: "bg-blue-500" },
        FilterCount { label: "UDP", count: 132, color_class: "bg-purple-500" },
        FilterCount { label: "ICMP", count: 12, color_class: "bg-yellow-500" },
        FilterCount { label: "DNS", count: 45, color_class: "bg-orange-500" },
        FilterCount { label: "TLS", count: 312, color_class: "bg-cyan-500" },
        FilterCount { label: "Other", count: 8, color_class: "bg-gray-500" },
    ]
}

pub fn get_interfaces() -> Vec<InterfaceInfo> {
    vec![
        InterfaceInfo { name: "eth0", speed: "10 Gbps", active: true },
        InterfaceInfo { name: "wlan0", speed: "866 Mbps", active: true },
        InterfaceInfo { name: "lo", speed: "—", active: false },
    ]
}

pub fn get_selected_packet_detail() -> PacketDetail {
    PacketDetail {
        timestamp: "14:35:21.123456",
        process: "firefox",
        pid: 8421,
        source: "192.168.1.4:49231",
        destination: "142.250.74.206:443",
        protocol: "TCP",
        length: 1448,
        interface: "eth0",
        direction: "Outgoing",
    }
}

pub fn get_capture_state() -> CaptureState {
    CaptureState {
        uptime: "00:01:37",
        events: 12548,
        dropped: 0,
        is_live: true,
    }
}

pub fn get_system_metrics() -> SystemMetrics {
    SystemMetrics {
        packets: 12548,
        bytes: "24.8 MB",
        flows: 312,
        connections: 87,
        cpu: "3.2%",
        memory: "142 MB",
        interface: "eth0",
        kernel: "6.8.0-arch1-1",
    }
}

pub fn get_hex_dump() -> Vec<HexLine> {
    vec![
        HexLine { offset: "0000", hex: "45 00 05 dc a1 b2 40 00", ascii: "E.....@." },
        HexLine { offset: "0010", hex: "40 06 8b 1c c0 a8 01 04", ascii: "@......." },
        HexLine { offset: "0020", hex: "8e fa 4a ce c0 8f 07 bb", ascii: "..J....." },
        HexLine { offset: "0030", hex: "be 3f 3e 4e 50 18 01 f7", ascii: ".?>NP..." },
        HexLine { offset: "0040", hex: "1a 2b 00 06 17 03 03 00", ascii: ".+......" },
        HexLine { offset: "0050", hex: "5a 00 00 00 56 03 03 00", ascii: "Z...V..." },
        HexLine { offset: "0060", hex: "9f 5d 7f 88 3a 91 6c 3e", ascii: ".]..:.l>" },
        HexLine { offset: "0070", hex: "4f 8c c3 71 d5 9e 88 3b", ascii: "O..q...;" },
    ]
}
