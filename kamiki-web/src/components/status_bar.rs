#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::data::dummy::get_system_metrics;

pub fn StatusBar() -> Element {
    let metrics = use_signal(get_system_metrics);
    let m = metrics.read();

    rsx! {
        footer { class: "h-8 border-t border-kamiki-border bg-kamiki-panel flex items-center justify-between px-3 shrink-0 text-[11px] text-kamiki-textSecondary z-10 select-none",
            // Left Group: Interface & Kernel Info
            div { class: "flex items-center gap-3",
                div { class: "flex items-center gap-1.5",
                    span { "Interface:" }
                    span { class: "font-mono font-medium text-kamiki-textPrimary", "{m.interface}" }
                }
                span { class: "text-kamiki-border", "│" }
                div { class: "flex items-center gap-1.5 hidden sm:flex",
                    span { "Kernel:" }
                    span { class: "font-mono font-medium text-kamiki-textPrimary", "{m.kernel}" }
                }
            }

            // Center Group: Packet & Flow Telemetry Metrics
            div { class: "flex items-center gap-4 font-mono",
                div { class: "flex items-center gap-1.5",
                    span { class: "text-kamiki-textSecondary font-sans", "Packets:" }
                    span { class: "font-medium text-kamiki-textPrimary", "{m.packets}" }
                }
                span { class: "text-kamiki-border hidden md:inline", "│" }
                div { class: "flex items-center gap-1.5 hidden md:flex",
                    span { class: "text-kamiki-textSecondary font-sans", "Bytes:" }
                    span { class: "font-medium text-kamiki-textPrimary", "{m.bytes}" }
                }
                span { class: "text-kamiki-border hidden lg:inline", "│" }
                div { class: "flex items-center gap-1.5 hidden lg:flex",
                    span { class: "text-kamiki-textSecondary font-sans", "Flows:" }
                    span { class: "font-medium text-kamiki-textPrimary", "{m.flows}" }
                }
                span { class: "text-kamiki-border hidden lg:inline", "│" }
                div { class: "flex items-center gap-1.5 hidden lg:flex",
                    span { class: "text-kamiki-textSecondary font-sans", "Connections:" }
                    span { class: "font-medium text-kamiki-textPrimary", "{m.connections}" }
                }
            }

            // Right Group: CPU & Memory Utilization Progress Bar
            div { class: "flex items-center gap-3",
                div { class: "flex items-center gap-1.5 font-mono",
                    span { class: "text-kamiki-textSecondary font-sans", "CPU:" }
                    span { class: "font-medium text-kamiki-textPrimary", "{m.cpu}" }
                }
                span { class: "text-kamiki-border", "│" }
                div { class: "flex items-center gap-2 font-mono",
                    span { class: "text-kamiki-textSecondary font-sans", "Mem:" }
                    span { class: "font-medium text-kamiki-textPrimary", "{m.memory}" }
                    // Memory Progress Bar
                    div { class: "w-24 h-2 bg-kamiki-bg border border-kamiki-border rounded-full overflow-hidden flex",
                        div { class: "h-full w-[42%] bg-gradient-to-r from-emerald-500 to-emerald-400 rounded-full" }
                    }
                }
            }
        }
    }
}
