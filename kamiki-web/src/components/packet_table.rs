#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::data::state::AppState;
use crate::components::AppIcon;

pub fn PacketTable() -> Element {
    let mut state = use_context::<AppState>();
    let packets = state.packets.read();
    let selected_idx = *state.selected_packet_idx.read();
    let selected_filter = state.selected_filter.read().clone();
    let search_query = state.search_query.read().clone();

    let filtered_packets: Vec<(usize, &crate::data::models::PacketEvent)> = packets
        .iter()
        .enumerate()
        .filter(|(_, pkt)| {
            // 1. Sidebar protocol filter check
            if let Some(ref filter) = selected_filter {
                let f_upper = filter.to_uppercase();
                let matches_proto = if f_upper == "TLS" {
                    pkt.protocol == "TCP" && (pkt.src_port == 443 || pkt.dst_port == 443)
                } else if f_upper == "DNS" {
                    pkt.protocol == "UDP" && (pkt.src_port == 53 || pkt.dst_port == 53)
                } else if f_upper == "OTHER" {
                    pkt.protocol != "TCP" && pkt.protocol != "UDP" && pkt.protocol != "ICMP"
                } else {
                    pkt.protocol.eq_ignore_ascii_case(filter)
                };
                if !matches_proto { return false; }
            }

            // 2. Header Search Query filter check
            if !search_query.trim().is_empty() {
                let q = search_query.trim().to_lowercase();
                let clean_q = q.replace("\"", "");

                if clean_q.contains("port ==") {
                    if let Some(num) = clean_q.split("==").nth(1).and_then(|s| s.trim().parse::<u16>().ok()) {
                        return pkt.src_port == num || pkt.dst_port == num;
                    }
                }
                if clean_q.contains("process ==") {
                    if let Some(proc_target) = clean_q.split("==").nth(1).map(|s| s.trim()) {
                        return pkt.process_name.to_lowercase().contains(proc_target);
                    }
                }
                if clean_q.contains("protocol ==") {
                    if let Some(proto_target) = clean_q.split("==").nth(1).map(|s| s.trim()) {
                        return pkt.protocol.to_lowercase().contains(proto_target);
                    }
                }

                return pkt.process_name.to_lowercase().contains(&clean_q)
                    || pkt.protocol.to_lowercase().contains(&clean_q)
                    || pkt.src_ip.contains(&clean_q)
                    || pkt.dst_ip.contains(&clean_q)
                    || pkt.src_port.to_string().contains(&clean_q)
                    || pkt.dst_port.to_string().contains(&clean_q)
                    || pkt.pid.to_string().contains(&clean_q);
            }

            true
        })
        .collect();

    rsx! {
        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg overflow-hidden flex flex-col flex-1 shadow-sm select-none text-xs min-h-[260px]",
            div { class: "overflow-x-auto overflow-y-auto flex-1 max-h-[380px]",
                table { class: "w-full text-left border-collapse",
                    thead {
                        tr { class: "sticky top-0 border-b border-kamiki-border/80 text-kamiki-textSecondary text-[11px] font-medium bg-kamiki-panel z-10",
                            th { class: "px-3 py-2 font-normal", "Time" }
                            th { class: "px-3 py-2 font-normal", "Process" }
                            th { class: "px-3 py-2 font-normal", "PID" }
                            th { class: "px-3 py-2 font-normal", "Source" }
                            th { class: "px-3 py-2 font-normal", "Destination" }
                            th { class: "px-3 py-2 font-normal", "Protocol" }
                            th { class: "px-3 py-2 font-normal", "Info" }
                            th { class: "px-3 py-2 font-normal text-right", "Size" }
                        }
                    }
                    tbody { class: "divide-y divide-kamiki-border/20 font-mono text-[11px]",
                        if filtered_packets.is_empty() {
                            tr {
                                td {
                                    colspan: "8",
                                    class: "px-3 py-8 text-center text-kamiki-textSecondary font-sans text-xs",
                                    if selected_filter.is_some() {
                                        "No packets match the active protocol filter"
                                    } else {
                                        "No packets captured yet — click a network interface on the left to start live capture"
                                    }
                                }
                            }
                        } else {
                            for (idx, pkt) in filtered_packets.iter().cloned() {
                                {
                                    let is_selected = selected_idx == Some(idx);
                                    let src_str = format!("{}:{}", pkt.src_ip, pkt.src_port);
                                    let dst_str = format!("{}:{}", pkt.dst_ip, pkt.dst_port);
                                    let proc_name = pkt.process_name.clone();

                                    // Mock Info strings based on protocol
                                    let info_str = match pkt.protocol.as_str() {
                                        "TCP" => if pkt.src_port == 443 || pkt.dst_port == 443 { "TLSv1.3 Application Data" } else { "ACK" },
                                        "UDP" => if pkt.src_port == 53 || pkt.dst_port == 53 { "Standard query A example.com" } else { "UDP Payload" },
                                        "ICMP" => "Echo (ping) request",
                                        _ => "SSH Protocol Data"
                                    };

                                    rsx! {
                                        tr {
                                            key: "{idx}",
                                            class: if is_selected {
                                                "bg-[#253b59] text-kamiki-textPrimary border-l-2 border-kamiki-blue font-medium cursor-pointer transition-colors"
                                            } else {
                                                "hover:bg-kamiki-panelHover/60 text-kamiki-textSecondary hover:text-kamiki-textPrimary cursor-pointer transition-colors"
                                            },
                                            onclick: move |_| {
                                                state.selected_packet_idx.set(Some(idx));
                                            },

                                            // Time
                                            td { class: "px-3 py-1.5 whitespace-nowrap text-kamiki-textSecondary", "{pkt.timestamp}" }

                                            // Process Name & Icon
                                            td { class: "px-3 py-1.5 whitespace-nowrap font-sans font-medium text-kamiki-textPrimary",
                                                div { class: "flex items-center gap-1.5",
                                                    AppIcon { name: proc_name.clone(), class: "w-4 h-4 shrink-0".to_string() }
                                                    span { "{proc_name}" }
                                                }
                                            }

                                            // PID
                                            td { class: "px-3 py-1.5 whitespace-nowrap text-kamiki-textSecondary",
                                                if pkt.pid > 0 { "{pkt.pid}" } else { "—" }
                                            }

                                            // Source IP:Port
                                            td { class: "px-3 py-1.5 whitespace-nowrap text-kamiki-textSecondary", "{src_str}" }

                                            // Destination IP:Port
                                            td { class: "px-3 py-1.5 whitespace-nowrap text-kamiki-textSecondary", "{dst_str}" }

                                            // Protocol Badge
                                            td { class: "px-3 py-1.5 whitespace-nowrap font-sans font-semibold text-[10px]",
                                                span { class: if pkt.protocol == "TCP" {
                                                    "px-1.5 py-0.5 rounded bg-[#1f6feb] text-white border border-[#388bfd]"
                                                } else if pkt.protocol == "UDP" {
                                                    "px-1.5 py-0.5 rounded bg-[#8957e5] text-white border border-[#a371f7]"
                                                } else {
                                                    "px-1.5 py-0.5 rounded bg-gray-600 text-white border border-gray-500"
                                                },
                                                    "{pkt.protocol}"
                                                }
                                            }

                                            // Info
                                            td { class: "px-3 py-1.5 whitespace-nowrap font-sans text-kamiki-textPrimary", "{info_str}" }

                                            // Size (Right Aligned)
                                            td { class: "px-3 py-1.5 whitespace-nowrap text-right font-mono font-medium text-kamiki-textPrimary", "{pkt.pkt_len}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
