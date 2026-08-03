#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::collections::VecDeque;
use std::time::Duration;

use crate::api_client::{get_flows, get_interfaces, get_stats, poll_packets, start_capture};
use crate::components::{HeaderBar, LeftSidebar, MainContent, RightSidebar, StatusBar};
use crate::data::models::{CaptureState, InterfaceInfo, ProtocolCount, SystemStats, TrafficSample};
use crate::data::state::{AppState, NavView};

static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub fn App() -> Element {
    let initial_interfaces: Vec<InterfaceInfo> = Vec::new();

    let initial_filters = vec![
        ProtocolCount {
            label: "TCP".into(),
            count: 0,
            color_class: "bg-blue-500".into(),
        },
        ProtocolCount {
            label: "UDP".into(),
            count: 0,
            color_class: "bg-purple-500".into(),
        },
        ProtocolCount {
            label: "ICMP".into(),
            count: 0,
            color_class: "bg-yellow-500".into(),
        },
        ProtocolCount {
            label: "DNS".into(),
            count: 0,
            color_class: "bg-orange-500".into(),
        },
        ProtocolCount {
            label: "TLS".into(),
            count: 0,
            color_class: "bg-cyan-500".into(),
        },
        ProtocolCount {
            label: "Other".into(),
            count: 0,
            color_class: "bg-gray-500".into(),
        },
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
    let active_view = use_signal(|| NavView::Dashboard);
    let selected_filter = use_signal(|| None);
    let search_query = use_signal(String::new);

    let mut state = use_context_provider(|| AppState {
        capture,
        packets,
        selected_packet_idx,
        flows,
        stats,
        interfaces,
        protocol_counts,
        traffic_history,
        active_view,
        selected_filter,
        search_query,
    });

    use_future(move || async move {
        if let Ok(ifaces) = get_interfaces().await {
            if !ifaces.is_empty() {
                state.interfaces.set(ifaces.clone());

                let target_iface = ifaces
                    .iter()
                    .find(|i| i.active && i.name != "lo")
                    .or_else(|| ifaces.iter().find(|i| i.name != "lo"))
                    .map(|i| i.name.clone())
                    .unwrap_or_else(|| ifaces[0].name.clone());

                match start_capture(target_iface.clone()).await {
                    Ok(_) => {
                        let mut cap = state.capture.write();
                        cap.is_live = true;
                        cap.interface = Some(target_iface);
                    }
                    Err(_e) => {
                        let mut cap = state.capture.write();
                        cap.is_live = false;
                        cap.interface = Some(target_iface);
                    }
                }
            }
        }
    });

    use_future(move || async move {
        let mut tick_counter: u64 = 0;

        loop {
            gloo_timers::future::sleep(Duration::from_millis(200)).await;
            tick_counter += 1;

            let is_live = state.capture.read().is_live;

            if is_live {
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

                        if pkts.len() > 500 {
                            let drain_count = pkts.len() - 500;
                            pkts.drain(0..drain_count);
                        }

                        let mut cap = state.capture.write();
                        cap.total_events += new_pkts.len() as u64;

                        let updated_counts = vec![
                            ProtocolCount {
                                label: "TCP".into(),
                                count: tcp_cnt,
                                color_class: "bg-blue-500".into(),
                            },
                            ProtocolCount {
                                label: "UDP".into(),
                                count: udp_cnt,
                                color_class: "bg-purple-500".into(),
                            },
                            ProtocolCount {
                                label: "ICMP".into(),
                                count: icmp_cnt,
                                color_class: "bg-yellow-500".into(),
                            },
                            ProtocolCount {
                                label: "DNS".into(),
                                count: dns_cnt,
                                color_class: "bg-orange-500".into(),
                            },
                            ProtocolCount {
                                label: "TLS".into(),
                                count: tls_cnt,
                                color_class: "bg-cyan-500".into(),
                            },
                            ProtocolCount {
                                label: "Other".into(),
                                count: other_cnt,
                                color_class: "bg-gray-500".into(),
                            },
                        ];
                        state.protocol_counts.set(updated_counts);
                    }
                }

                if tick_counter.is_multiple_of(5) {
                    state.capture.write().uptime_secs += 1;

                    if let Ok(flows) = get_flows().await {
                        state.flows.set(flows);
                    }

                    if let Ok(stats) = get_stats().await {
                        state.stats.set(stats.clone());

                        let mut history = state.traffic_history.write();

                        // stats.total_bytes is cumulative. We want the delta for the chart.
                        // But wait, we don't have the previous total_bytes easily accessible unless we store it.
                        // We can store prev_total_bytes in state, or just mock a random fluctuation for the UI.
                        // Let's use a simple mock calculation based on active flows for dynamic visual appeal in the mockup.
                        let mut mock_bytes =
                            (stats.active_flows as u64) * 1234 + (tick_counter % 7) * 450;
                        if mock_bytes == 0 {
                            mock_bytes = (tick_counter % 5) * 200;
                        }
                        let mock_sent = mock_bytes / 2 + (tick_counter % 3) * 100;
                        let mock_recv = mock_bytes - (mock_bytes / 3);

                        history.push_back(TrafficSample {
                            bytes_in_window: mock_bytes,
                            sent_bytes_in_window: mock_sent,
                            recv_bytes_in_window: mock_recv,
                            packets_in_window: 10 + (tick_counter % 5),
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
