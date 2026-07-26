#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::data::dummy::get_selected_packet_detail;

pub fn PacketDetails() -> Element {
    let detail = use_signal(get_selected_packet_detail);
    let d = detail.read();

    rsx! {
        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg overflow-hidden flex flex-col shadow-sm select-none text-xs",
            // Header Bar
            div { class: "px-3 py-2 border-b border-kamiki-border/80 bg-kamiki-panel/50 flex items-center justify-between",
                span { class: "font-semibold text-kamiki-textPrimary tracking-tight", "Packet Details" }
                div { class: "flex items-center gap-1 text-kamiki-textSecondary",
                    button { class: "p-0.5 hover:text-kamiki-textPrimary transition-colors", title: "Close Panel",
                        "✕"
                    }
                }
            }

            // Details Key-Value List
            div { class: "p-3 flex flex-col gap-2 font-mono text-[11px]",
                // Timestamp
                div { class: "flex justify-between items-center",
                    span { class: "text-kamiki-textSecondary font-sans", "Timestamp" }
                    span { class: "text-kamiki-textPrimary font-medium", "{d.timestamp}" }
                }

                // Process
                div { class: "flex justify-between items-center",
                    span { class: "text-kamiki-textSecondary font-sans", "Process" }
                    div { class: "flex items-center gap-1.5 text-kamiki-textPrimary font-sans font-medium",
                        span { "🦊" }
                        span { "{d.process} (PID {d.pid})" }
                    }
                }

                // Source
                div { class: "flex justify-between items-center",
                    span { class: "text-kamiki-textSecondary font-sans", "Source" }
                    span { class: "text-kamiki-textPrimary", "{d.source}" }
                }

                // Destination
                div { class: "flex justify-between items-center",
                    span { class: "text-kamiki-textSecondary font-sans", "Destination" }
                    span { class: "text-kamiki-textPrimary", "{d.destination}" }
                }

                // Protocol
                div { class: "flex justify-between items-center",
                    span { class: "text-kamiki-textSecondary font-sans", "Protocol" }
                    span { class: "px-1.5 py-0.5 rounded bg-blue-500/15 text-blue-400 font-sans font-semibold text-[10px] border border-blue-500/30",
                        "{d.protocol}"
                    }
                }

                // Length
                div { class: "flex justify-between items-center",
                    span { class: "text-kamiki-textSecondary font-sans", "Length" }
                    span { class: "text-kamiki-textPrimary", "{d.length} bytes" }
                }

                // Interface
                div { class: "flex justify-between items-center",
                    span { class: "text-kamiki-textSecondary font-sans", "Interface" }
                    span { class: "text-kamiki-textPrimary font-medium", "{d.interface}" }
                }

                // Direction
                div { class: "flex justify-between items-center",
                    span { class: "text-kamiki-textSecondary font-sans", "Direction" }
                    div { class: "flex items-center gap-1 text-emerald-400 font-sans font-medium",
                        span { "↑" }
                        span { "{d.direction}" }
                    }
                }
            }
        }
    }
}
