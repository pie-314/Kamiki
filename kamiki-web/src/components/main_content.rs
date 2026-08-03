#![allow(non_snake_case)]

use crate::components::{AppIcon, ChartsRow, PacketTable, TopProcessesTable};
use crate::data::state::{AppState, NavView};
use dioxus::prelude::*;

pub fn MainContent() -> Element {
    let state = use_context::<AppState>();
    let active_view = *state.active_view.read();
    let flows = state.flows.read();
    let packets = state.packets.read();
    let stats = state.stats.read();

    rsx! {
        section { class: "flex-1 flex flex-col overflow-y-auto p-3 gap-3 bg-kamiki-bg min-w-0 select-none",
            match active_view {
                NavView::Dashboard => rsx! {
                    TopProcessesTable {}
                    ChartsRow {}
                    PacketTable {}
                },
                NavView::Connections => rsx! {
                    div { class: "flex-1 flex flex-col gap-3",
                        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex items-center justify-between shadow-sm",
                            div { class: "flex items-center gap-3",
                                span { class: "p-2 rounded bg-kamiki-blue/10 text-kamiki-blue font-bold text-sm", "🔌" }
                                div {
                                    div { class: "font-semibold text-kamiki-textPrimary text-sm", "Active Network Connections" }
                                    div { class: "text-kamiki-textSecondary text-xs", "Live socket flows captured via eBPF kernel probes" }
                                }
                            }
                            div { class: "flex items-center gap-4 text-xs font-mono",
                                div { class: "px-3 py-1 rounded bg-kamiki-bg border border-kamiki-border flex gap-2",
                                    span { class: "text-kamiki-textSecondary", "Flows:" }
                                    span { class: "text-kamiki-blue font-bold", "{stats.active_flows}" }
                                }
                                div { class: "px-3 py-1 rounded bg-kamiki-bg border border-kamiki-border flex gap-2",
                                    span { class: "text-kamiki-textSecondary", "Total Traffic:" }
                                    span { class: "text-emerald-400 font-bold", "{stats.total_bytes / 1024} KB" }
                                }
                            }
                        }

                        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg overflow-hidden flex-1 shadow-sm flex flex-col",
                            div { class: "overflow-x-auto flex-1",
                                table { class: "w-full text-left border-collapse",
                                    thead {
                                        tr { class: "border-b border-kamiki-border/80 text-kamiki-textSecondary text-[11px] font-medium bg-kamiki-panel sticky top-0",
                                            th { class: "px-3 py-2 font-normal", "Source" }
                                            th { class: "px-3 py-2 font-normal", "Destination" }
                                            th { class: "px-3 py-2 font-normal", "Protocol" }
                                            th { class: "px-3 py-2 font-normal", "Packets" }
                                            th { class: "px-3 py-2 font-normal", "Bytes" }
                                            th { class: "px-3 py-2 font-normal", "Sent" }
                                            th { class: "px-3 py-2 font-normal", "Received" }
                                            th { class: "px-3 py-2 font-normal text-right", "State" }
                                        }
                                    }
                                    tbody { class: "divide-y divide-kamiki-border/20 font-mono text-[11px]",
                                        if flows.is_empty() {
                                            tr {
                                                td { colspan: "8", class: "px-3 py-8 text-center text-kamiki-textSecondary font-sans text-xs",
                                                    "No active connections recorded yet"
                                                }
                                            }
                                        } else {
                                            for flow in flows.iter() {
                                                tr { class: "hover:bg-kamiki-panelHover/60 text-kamiki-textSecondary hover:text-kamiki-textPrimary transition-colors",
                                                    td { class: "px-3 py-2 text-kamiki-textPrimary", "{flow.src_ip}:{flow.src_port}" }
                                                    td { class: "px-3 py-2 text-kamiki-textPrimary", "{flow.dst_ip}:{flow.dst_port}" }
                                                    td { class: "px-3 py-2",
                                                        span { class: "px-1.5 py-0.5 rounded bg-blue-500/15 text-blue-400 font-sans font-semibold text-[10px]", "{flow.protocol}" }
                                                    }
                                                    td { class: "px-3 py-2", "{flow.packets}" }
                                                    td { class: "px-3 py-2 text-kamiki-textPrimary font-medium", "{flow.bytes} B" }
                                                    td { class: "px-3 py-2 text-[#3fb950]", "{flow.sent_bytes} B" }
                                                    td { class: "px-3 py-2 text-[#58a6ff]", "{flow.recv_bytes} B" }
                                                    td { class: "px-3 py-2 text-right",
                                                        span { class: "px-1.5 py-0.5 rounded bg-emerald-500/15 text-emerald-400 font-sans text-[10px]", "ESTABLISHED" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                NavView::Packets => rsx! {
                    div { class: "flex-1 flex flex-col gap-3",
                        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex items-center justify-between shadow-sm",
                            div { class: "flex items-center gap-3",
                                span { class: "p-2 rounded bg-kamiki-blue/10 text-kamiki-blue font-bold text-sm", "📦" }
                                div {
                                    div { class: "font-semibold text-kamiki-textPrimary text-sm", "Packet Feed Workspace" }
                                    div { class: "text-kamiki-textSecondary text-xs", "Real-time eBPF packet capture log" }
                                }
                            }
                        }
                        PacketTable {}
                    }
                },
                NavView::Processes => rsx! {
                    div { class: "flex-1 flex flex-col gap-3",
                        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex items-center justify-between shadow-sm",
                            div { class: "flex items-center gap-3",
                                span { class: "p-2 rounded bg-kamiki-blue/10 text-kamiki-blue font-bold text-sm", "⚙️" }
                                div {
                                    div { class: "font-semibold text-kamiki-textPrimary text-sm", "Process Traffic Monitor" }
                                    div { class: "text-kamiki-textSecondary text-xs", "Bandwidth consumption mapped to local Linux processes" }
                                }
                            }
                        }
                        TopProcessesTable {}
                    }
                },
                NavView::Events => rsx! {
                    div { class: "flex-1 flex flex-col gap-3",
                        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex items-center justify-between shadow-sm",
                            div { class: "flex items-center gap-3",
                                span { class: "p-2 rounded bg-kamiki-blue/10 text-kamiki-blue font-bold text-sm", "🛡️" }
                                div {
                                    div { class: "font-semibold text-kamiki-textPrimary text-sm", "eBPF Security & System Events" }
                                    div { class: "text-kamiki-textSecondary text-xs", "Kernel level network events and protocol alerts" }
                                }
                            }
                        }
                        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg overflow-hidden flex-1 shadow-sm flex flex-col p-3 gap-2 font-mono text-xs",
                            div { class: "px-3 py-2 rounded bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 flex justify-between items-center",
                                span { "INFO [eBPF Prober] XDP network prober attached to interface successfully" }
                                span { class: "text-[10px] text-kamiki-textSecondary", "LIVE" }
                            }
                            for pkt in packets.iter().take(10) {
                                div { class: "px-3 py-2 rounded bg-kamiki-bg border border-kamiki-border/60 text-kamiki-textSecondary flex justify-between items-center",
                                    div { class: "flex items-center gap-2",
                                        AppIcon { name: pkt.process_name.clone(), class: "w-4 h-4 shrink-0".to_string() }
                                        span { class: "text-kamiki-textPrimary font-semibold", "{pkt.process_name}" }
                                        span { "opened {pkt.protocol} flow {pkt.src_ip}:{pkt.src_port} -> {pkt.dst_ip}:{pkt.dst_port}" }
                                    }
                                    span { class: "text-[10px]", "{pkt.timestamp}" }
                                }
                            }
                        }
                    }
                },
                NavView::Timeline => rsx! {
                    div { class: "flex-1 flex flex-col gap-3",
                        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex items-center justify-between shadow-sm",
                            div { class: "flex items-center gap-3",
                                span { class: "p-2 rounded bg-kamiki-blue/10 text-kamiki-blue font-bold text-sm", "⏳" }
                                div {
                                    div { class: "font-semibold text-kamiki-textPrimary text-sm", "Network Activity Timeline" }
                                    div { class: "text-kamiki-textSecondary text-xs", "Chronological bandwidth trends and event timeline" }
                                }
                            }
                        }
                        ChartsRow {}
                    }
                },
            }
        }
    }
}
