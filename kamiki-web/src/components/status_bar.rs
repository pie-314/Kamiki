#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::data::state::AppState;

pub fn StatusBar() -> Element {
    let state = use_context::<AppState>();
    let stats = state.stats.read();
    let cap = state.capture.read();

    let iface = cap.interface.clone().unwrap_or_else(|| stats.interface.clone());
    let kernel = if stats.kernel.is_empty() { "Linux".into() } else { stats.kernel.clone() };

    let bytes_formatted = if stats.total_bytes > 1_000_000 {
        format!("{:.1} MB", stats.total_bytes as f64 / 1_000_000.0)
    } else if stats.total_bytes > 1_000 {
        format!("{:.1} KB", stats.total_bytes as f64 / 1_000.0)
    } else {
        format!("{} B", stats.total_bytes)
    };

    rsx! {
        footer { class: "h-8 border-t border-kamiki-border bg-kamiki-panel flex items-center justify-between px-3 shrink-0 text-[11px] text-kamiki-textSecondary z-10 select-none",
            // Left Group: Interface & Kernel Info
            div { class: "flex items-center gap-3",
                div { class: "flex items-center gap-1.5",
                    span { "Interface:" }
                    span { class: "font-mono font-medium text-kamiki-textPrimary", "{iface}" }
                }
                span { class: "text-kamiki-border", "│" }
                div { class: "flex items-center gap-1.5 hidden sm:flex",
                    span { "Kernel:" }
                    span { class: "font-mono font-medium text-kamiki-textPrimary truncate max-w-[180px]", "{kernel}" }
                }
            }

            // Center Group: Packet & Flow Telemetry Metrics
            div { class: "flex items-center gap-4 font-mono",
                div { class: "flex items-center gap-1.5",
                    span { class: "text-kamiki-textSecondary font-sans", "Packets:" }
                    span { class: "font-medium text-kamiki-textPrimary", "{stats.total_pkts}" }
                }
                span { class: "text-kamiki-border hidden md:inline", "│" }
                div { class: "flex items-center gap-1.5 hidden md:flex",
                    span { class: "text-kamiki-textSecondary font-sans", "Bytes:" }
                    span { class: "font-medium text-kamiki-textPrimary", "{bytes_formatted}" }
                }
                span { class: "text-kamiki-border hidden lg:inline", "│" }
                div { class: "flex items-center gap-1.5 hidden lg:flex",
                    span { class: "text-kamiki-textSecondary font-sans", "Flows:" }
                    span { class: "font-medium text-kamiki-textPrimary", "{stats.active_flows}" }
                }
            }

            // Right Group: Capture Telemetry Status
            div { class: "flex items-center gap-3",
                div { class: "flex items-center gap-1.5 font-mono",
                    span { class: "text-kamiki-textSecondary font-sans", "Engine:" }
                    span { class: if cap.is_live { "font-medium text-emerald-400" } else { "font-medium text-gray-400" },
                        if cap.is_live { "ACTIVE" } else { "READY" }
                    }
                }
            }
        }
    }
}
