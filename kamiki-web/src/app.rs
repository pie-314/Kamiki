#![allow(non_snake_case)]

use std::collections::VecDeque;
use std::time::Duration;
use dioxus::prelude::*;

use crate::components::{HeaderBar, LeftSidebar, MainContent, RightSidebar, StatusBar};
use crate::data::models::{CaptureState, InterfaceInfo, ProtocolCount, SystemStats, TrafficSample};
use crate::data::state::AppState;
use crate::server::{get_flows, get_interfaces, get_stats, poll_packets, start_capture};

static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub fn App() -> Element {
    // 1. Instantiate signals directly at the top level of the App component
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

    let capture = use_signal(CaptureState::default);
    let packets = use_signal(Vec::new);
    let selected_packet_idx = use_signal(|| None);
    let flows = use_signal(Vec::new);
    let stats = use_signal(SystemStats::default);
    let interfaces = use_signal(|| initial_interfaces);
    let protocol_counts = use_signal(|| initial_filters);
    let traffic_history = use_signal(|| history);

    // Provide AppState struct via context
    let mut state = use_context_provider(|| AppState {
        capture,
        packets,
        selected_packet_idx,
        flows,
        stats,
        interfaces,
        protocol_counts,
        traffic_history,
    });

    // 2. Fetch network interfaces on mount & auto-start capture on default interface
    use_future(move || async move {
        if let Ok(ifaces) = get_interfaces().await {
            if !ifaces.is_empty() {
                let first_name = ifaces[0].name.clone();
                state.interfaces.set(ifaces);

                let _ = start_capture(first_name.clone()).await;
                let mut cap = state.capture.write();
                cap.is_live = true;
                cap.interface = Some(first_name);
            }
        }
    });

    // 3. Main telemetry polling loop (runs in background)
    use_future(move || async move {
        let mut tick_counter: u64 = 0;

        loop {
            // Sleep 200ms tick interval
            #[cfg(not(target_arch = "wasm32"))]
            tokio::time::sleep(Duration::from_millis(200)).await;

            #[cfg(target_arch = "wasm32")]
            gloo_timers::future::sleep(Duration::from_millis(200)).await;

            tick_counter += 1;

            let is_live = state.capture.read().is_live;

            if is_live {
                // A. Poll recent packets (every 200ms)
                if let Ok(new_pkts) = poll_packets(100).await {
                    if !new_pkts.is_empty() {
                        let mut pkts = state.packets.write();
                        let mut tcp_cnt = 0u32;
                        let mut udp_cnt = 0u32;
                        let mut icmp_cnt = 0u32;
                        let mut dns_cnt = 0u32;
                        let mut tls_cnt = 0u32;
                        let mut other_cnt = 0u32;

                        for pkt in new_pkts.iter() {
                            pkts.push(pkt.clone());
                            match pkt.protocol.as_str() {
                                "TCP" => tcp_cnt += 1,
                                "UDP" => udp_cnt += 1,
                                "ICMP" => icmp_cnt += 1,
                                _ => other_cnt += 1,
                            }
                            if pkt.src_port == 53 || pkt.dst_port == 53 {
                                dns_cnt += 1;
                            }
                            if pkt.src_port == 443 || pkt.dst_port == 443 {
                                tls_cnt += 1;
                            }
                        }

                        // Cap displayed packet history buffer at 500
                        if pkts.len() > 500 {
                            let drain_count = pkts.len() - 500;
                            pkts.drain(0..drain_count);
                        }

                        // Update Capture total events counter
                        let mut cap = state.capture.write();
                        cap.total_events += new_pkts.len() as u64;

                        // Update protocol counts
                        let updated_counts = vec![
                            ProtocolCount { label: "TCP".into(), count: tcp_cnt, color_class: "bg-blue-500".into() },
                            ProtocolCount { label: "UDP".into(), count: udp_cnt, color_class: "bg-purple-500".into() },
                            ProtocolCount { label: "ICMP".into(), count: icmp_cnt, color_class: "bg-yellow-500".into() },
                            ProtocolCount { label: "DNS".into(), count: dns_cnt, color_class: "bg-orange-500".into() },
                            ProtocolCount { label: "TLS".into(), count: tls_cnt, color_class: "bg-cyan-500".into() },
                            ProtocolCount { label: "Other".into(), count: other_cnt, color_class: "bg-gray-500".into() },
                        ];
                        state.protocol_counts.set(updated_counts);
                    }
                }

                // B. Fetch flows and aggregate stats (every 1s = every 5th tick)
                if tick_counter % 5 == 0 {
                    // Update uptime
                    state.capture.write().uptime_secs += 1;

                    if let Ok(flows) = get_flows().await {
                        state.flows.set(flows);
                    }

                    if let Ok(stats) = get_stats().await {
                        state.stats.set(stats.clone());

                        // Push sample to traffic history for SVG charts
                        let mut history = state.traffic_history.write();
                        history.push_back(TrafficSample {
                            bytes_in_window: stats.total_bytes,
                            packets_in_window: stats.total_pkts,
                            active_flows: stats.active_flows,
                        });
                        if history.len() > 60 {
                            history.pop_front();
                        }
                    }
                }
            }
        }
    });

    rsx! {
        link { rel: "stylesheet", href: TAILWIND_CSS }
        main { class: "h-screen w-screen bg-kamiki-bg text-kamiki-textPrimary flex flex-col overflow-hidden font-sans antialiased select-none",
            HeaderBar {}
            div { class: "flex-1 flex overflow-hidden min-h-0",
                LeftSidebar {}
                MainContent {}
                RightSidebar {}
            }
            StatusBar {}
        }
    }
}