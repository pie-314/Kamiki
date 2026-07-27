#![allow(non_snake_case)]

use std::collections::HashMap;
use dioxus::prelude::*;
use crate::data::state::AppState;

#[derive(Clone, Debug)]
struct ProcessRowData {
    name: String,
    pid_str: String,
    connections: u32,
    total_bytes: u64,
    bytes_str: String,
    top_remote: String,
}

pub fn TopProcessesTable() -> Element {
    let state = use_context::<AppState>();
    let packets = state.packets.read();

    // Aggregate packets by process_name
    let mut proc_map: HashMap<String, (u32, u32, u64, String)> = HashMap::new();

    for pkt in packets.iter() {
        let entry = proc_map.entry(pkt.process_name.clone()).or_insert((
            pkt.pid,
            0,
            0,
            format!("{}:{}", pkt.dst_ip, pkt.dst_port),
        ));

        entry.1 += 1;
        entry.2 += pkt.pkt_len as u64;
    }

    let mut rows: Vec<ProcessRowData> = proc_map
        .into_iter()
        .map(|(name, (pid, connections, total_bytes, top_remote))| {
            let pid_str = if pid > 0 { format!("{}", pid) } else { "—".into() };
            let bytes_str = if total_bytes > 1_000_000 {
                format!("{:.1} MB", total_bytes as f64 / 1_000_000.0)
            } else if total_bytes > 1_000 {
                format!("{:.1} KB", total_bytes as f64 / 1_000.0)
            } else {
                format!("{} B", total_bytes)
            };

            ProcessRowData {
                name,
                pid_str,
                connections,
                total_bytes,
                bytes_str,
                top_remote,
            }
        })
        .collect();

    rows.sort_by(|a, b| b.total_bytes.cmp(&a.total_bytes));
    if rows.len() > 6 {
        rows.truncate(6);
    }

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
                            th { class: "px-3 py-1.5 font-normal font-semibold text-kamiki-textPrimary", "Total Bytes" }
                            th { class: "px-3 py-1.5 font-normal", "Top Remote" }
                        }
                    }
                    tbody { class: "divide-y divide-kamiki-border/30",
                        if rows.is_empty() {
                            tr {
                                td {
                                    colspan: "5",
                                    class: "px-3 py-4 text-center text-kamiki-textSecondary font-mono text-[11px]",
                                    "No process traffic recorded yet — select an interface to capture"
                                }
                            }
                        } else {
                            for proc in rows.iter() {
                                tr {
                                    key: "{proc.name}",
                                    class: "hover:bg-kamiki-panelHover/60 transition-colors group",

                                    // Process Name & Icon
                                    td { class: "px-3 py-1.5 flex items-center gap-2 font-medium text-kamiki-textPrimary",
                                        span { class: "w-4 h-4 flex items-center justify-center font-mono text-[10px] rounded bg-kamiki-bg border border-kamiki-border text-indigo-400",
                                            if proc.name.starts_with('f') { "🦊" }
                                            else if proc.name.starts_with('d') { "🎮" }
                                            else if proc.name.starts_with('s') { ">_" }
                                            else if proc.name.starts_with('c') { "//" }
                                            else { "⚙" }
                                        }
                                        span { "{proc.name}" }
                                    }

                                    // PID
                                    td { class: "px-3 py-1.5 font-mono text-kamiki-textSecondary", "{proc.pid_str}" }

                                    // Connections + Sparkline Activity Bars
                                    td { class: "px-3 py-1.5",
                                        div { class: "flex items-center gap-2",
                                            span { class: "font-mono font-medium text-kamiki-textPrimary w-5", "{proc.connections}" }
                                            div { class: "flex items-end gap-[2px] h-3 w-16 opacity-80 group-hover:opacity-100 transition-opacity",
                                                div { class: "w-1 rounded-xs bg-emerald-500", style: "height: {std::cmp::min(100, proc.connections * 2)}%" }
                                                div { class: "w-1 rounded-xs bg-emerald-400", style: "height: {std::cmp::min(100, proc.connections * 3)}%" }
                                                div { class: "w-1 rounded-xs bg-emerald-500", style: "height: {std::cmp::min(100, proc.connections * 1 + 20)}%" }
                                                div { class: "w-1 rounded-xs bg-emerald-400", style: "height: {std::cmp::min(100, proc.connections * 2 + 10)}%" }
                                                div { class: "w-1 rounded-xs bg-emerald-500", style: "height: {std::cmp::min(100, proc.connections * 3)}%" }
                                            }
                                        }
                                    }

                                    // Total Bytes (Highlighted)
                                    td { class: "px-3 py-1.5 font-mono font-semibold text-kamiki-textPrimary", "{proc.bytes_str}" }

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
}
