#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::data::dummy::get_processes;

pub fn TopProcessesTable() -> Element {
    let processes = use_signal(get_processes);

    rsx! {
        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg overflow-hidden flex flex-col shadow-sm select-none text-xs",
            // Card Title Header
            div { class: "px-3 py-2 border-b border-kamiki-border/80 bg-kamiki-panel/50 flex items-center justify-between",
                span { class: "font-semibold text-kamiki-textPrimary tracking-tight", "Top Processes" }
            }

            // Table Wrapper
            div { class: "overflow-x-auto",
                table { class: "w-full text-left border-collapse",
                    thead {
                        tr { class: "border-b border-kamiki-border/60 text-kamiki-textSecondary text-[11px] font-medium bg-kamiki-bg/40",
                            th { class: "px-3 py-1.5 font-normal", "Process" }
                            th { class: "px-3 py-1.5 font-normal", "PID" }
                            th { class: "px-3 py-1.5 font-normal", "Connections" }
                            th { class: "px-3 py-1.5 font-normal", "Sent" }
                            th { class: "px-3 py-1.5 font-normal", "Received" }
                            th { class: "px-3 py-1.5 font-normal font-semibold text-kamiki-textPrimary", "Total" }
                            th { class: "px-3 py-1.5 font-normal", "Top Remote" }
                        }
                    }
                    tbody { class: "divide-y divide-kamiki-border/30",
                        for proc in processes.read().iter() {
                            tr {
                                key: "{proc.pid}",
                                class: "hover:bg-kamiki-panelHover/60 transition-colors group",

                                // Process Name & Icon
                                td { class: "px-3 py-1.5 flex items-center gap-2 font-medium text-kamiki-textPrimary",
                                    span { class: "w-4 h-4 flex items-center justify-center font-mono text-[10px] rounded bg-kamiki-bg border border-kamiki-border {proc.icon_color}",
                                        if proc.name.starts_with('f') { "🦊" }
                                        else if proc.name.starts_with('d') { "🎮" }
                                        else if proc.name.starts_with('s') && proc.name.contains("ssh") { ">_" }
                                        else if proc.name.starts_with('s') && proc.name.contains("spot") { "🎵" }
                                        else if proc.name.starts_with('c') { "//" }
                                        else { "⚙" }
                                    }
                                    span { "{proc.name}" }
                                }

                                // PID
                                td { class: "px-3 py-1.5 font-mono text-kamiki-textSecondary", "{proc.pid}" }

                                // Connections + Sparkline Activity Bars
                                td { class: "px-3 py-1.5",
                                    div { class: "flex items-center gap-2",
                                        span { class: "font-mono font-medium text-kamiki-textPrimary w-5", "{proc.connections}" }
                                        // Inline Activity Sparkline Bars
                                        div { class: "flex items-end gap-[2px] h-3 w-16 opacity-80 group-hover:opacity-100 transition-opacity",
                                            div { class: "w-1 rounded-xs bg-emerald-500", style: "height: {std::cmp::min(100, proc.connections * 2)}%" }
                                            div { class: "w-1 rounded-xs bg-emerald-400", style: "height: {std::cmp::min(100, proc.connections * 3)}%" }
                                            div { class: "w-1 rounded-xs bg-emerald-500", style: "height: {std::cmp::min(100, proc.connections * 1 + 20)}%" }
                                            div { class: "w-1 rounded-xs bg-emerald-400", style: "height: {std::cmp::min(100, proc.connections * 2 + 10)}%" }
                                            div { class: "w-1 rounded-xs bg-emerald-500", style: "height: {std::cmp::min(100, proc.connections * 3)}%" }
                                        }
                                    }
                                }

                                // Sent
                                td { class: "px-3 py-1.5 font-mono text-kamiki-textSecondary", "{proc.sent}" }

                                // Received
                                td { class: "px-3 py-1.5 font-mono text-kamiki-textSecondary", "{proc.received}" }

                                // Total (Highlighted)
                                td { class: "px-3 py-1.5 font-mono font-semibold text-kamiki-textPrimary", "{proc.total}" }

                                // Top Remote
                                td { class: "px-3 py-1.5 font-mono text-kamiki-textSecondary group-hover:text-kamiki-textPrimary transition-colors", "{proc.top_remote}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
