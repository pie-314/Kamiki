#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::data::state::AppState;

pub fn PacketDetails() -> Element {
    let state = use_context::<AppState>();
    let selected_pkt = state.selected_packet();
    let capture_iface = state.capture.read().interface.clone().unwrap_or_else(|| "eth0".into());

    rsx! {
        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg overflow-hidden flex flex-col shadow-sm select-none text-xs",
            div { class: "px-3 py-2 border-b border-kamiki-border/80 bg-kamiki-panel/50 flex items-center justify-between",
                span { class: "font-semibold text-kamiki-textPrimary tracking-tight", "Packet Details" }
            }

            if let Some(pkt) = selected_pkt {
                div { class: "p-3 flex flex-col gap-2 font-mono text-[11px]",
                    // Timestamp
                    div { class: "flex justify-between items-center",
                        span { class: "text-kamiki-textSecondary font-sans", "Timestamp" }
                        span { class: "text-kamiki-textPrimary font-medium truncate max-w-[140px]", "{pkt.timestamp}" }
                    }

                    // Process
                    div { class: "flex justify-between items-center",
                        span { class: "text-kamiki-textSecondary font-sans", "Process" }
                        div { class: "flex items-center gap-1.5 text-kamiki-textPrimary font-sans font-medium",
                            span { "⚙" }
                            span {
                                if pkt.pid > 0 {
                                    "{pkt.process_name} (PID {pkt.pid})"
                                } else {
                                    "{pkt.process_name}"
                                }
                            }
                        }
                    }

                    // Source
                    div { class: "flex justify-between items-center",
                        span { class: "text-kamiki-textSecondary font-sans", "Source" }
                        span { class: "text-kamiki-textPrimary", "{pkt.src_ip}:{pkt.src_port}" }
                    }

                    // Destination
                    div { class: "flex justify-between items-center",
                        span { class: "text-kamiki-textSecondary font-sans", "Destination" }
                        span { class: "text-kamiki-textPrimary", "{pkt.dst_ip}:{pkt.dst_port}" }
                    }

                    // Protocol
                    div { class: "flex justify-between items-center",
                        span { class: "text-kamiki-textSecondary font-sans", "Protocol" }
                        span { class: "px-1.5 py-0.5 rounded bg-blue-500/15 text-blue-400 font-sans font-semibold text-[10px] border border-blue-500/30",
                            "{pkt.protocol}"
                        }
                    }

                    // Length
                    div { class: "flex justify-between items-center",
                        span { class: "text-kamiki-textSecondary font-sans", "Length" }
                        span { class: "text-kamiki-textPrimary", "{pkt.pkt_len} bytes" }
                    }

                    // Interface
                    div { class: "flex justify-between items-center",
                        span { class: "text-kamiki-textSecondary font-sans", "Interface" }
                        span { class: "text-kamiki-textPrimary font-medium", "{capture_iface}" }
                    }
                }
            } else {
                div { class: "p-4 text-center text-kamiki-textSecondary font-sans text-xs",
                    "Click any row in the packet table to inspect details"
                }
            }
        }
    }
}
