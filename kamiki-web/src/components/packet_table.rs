#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::data::dummy::get_packets;

pub fn PacketTable() -> Element {
    let packets = use_signal(get_packets);

    rsx! {
        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg overflow-hidden flex flex-col flex-1 shadow-sm select-none text-xs min-h-[260px]",
            // Scrollable Table Container
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
                        for (idx, pkt) in packets.read().iter().enumerate() {
                            tr {
                                key: "{idx}",
                                class: if pkt.is_selected {
                                    "bg-kamiki-blue/20 text-kamiki-textPrimary border-l-2 border-kamiki-blue font-medium cursor-pointer transition-colors"
                                } else {
                                    "hover:bg-kamiki-panelHover/60 text-kamiki-textSecondary hover:text-kamiki-textPrimary cursor-pointer transition-colors"
                                },

                                // Time
                                td { class: "px-3 py-1.5 whitespace-nowrap text-kamiki-textSecondary", "{pkt.time}" }

                                // Process Name & Icon
                                td { class: "px-3 py-1.5 whitespace-nowrap font-sans font-medium text-kamiki-textPrimary",
                                    div { class: "flex items-center gap-1.5",
                                        span { class: "text-[10px]",
                                            if pkt.process.starts_with('f') { "🦊" }
                                            else if pkt.process.starts_with('d') { "🎮" }
                                            else if pkt.process.starts_with('s') && pkt.process.contains("ssh") { ">_" }
                                            else if pkt.process.starts_with('s') && pkt.process.contains("spot") { "🎵" }
                                            else if pkt.process.starts_with('c') { "//" }
                                            else { "⚙" }
                                        }
                                        span { "{pkt.process}" }
                                    }
                                }

                                // PID
                                td { class: "px-3 py-1.5 whitespace-nowrap text-kamiki-textSecondary", "{pkt.pid}" }

                                // Source IP:Port
                                td { class: "px-3 py-1.5 whitespace-nowrap text-kamiki-textSecondary", "{pkt.source}" }

                                // Destination IP:Port
                                td { class: "px-3 py-1.5 whitespace-nowrap text-kamiki-textSecondary", "{pkt.destination}" }

                                // Protocol Badge
                                td { class: "px-3 py-1.5 whitespace-nowrap font-sans font-semibold text-[10px]",
                                    span { class: if pkt.protocol == "TCP" {
                                        "px-1.5 py-0.5 rounded bg-blue-500/15 text-blue-400 border border-blue-500/30"
                                    } else if pkt.protocol == "UDP" {
                                        "px-1.5 py-0.5 rounded bg-purple-500/15 text-purple-400 border border-purple-500/30"
                                    } else {
                                        "px-1.5 py-0.5 rounded bg-gray-500/15 text-gray-400 border border-gray-500/30"
                                    },
                                        "{pkt.protocol}"
                                    }
                                }

                                // Info
                                td { class: "px-3 py-1.5 truncate max-w-xs text-kamiki-textPrimary font-sans text-[11px]", "{pkt.info}" }

                                // Size (Right Aligned)
                                td { class: "px-3 py-1.5 whitespace-nowrap text-right font-mono font-medium text-kamiki-textPrimary", "{pkt.size}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
