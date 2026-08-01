use std::collections::HashMap;
use dioxus::prelude::*;
use crate::data::state::AppState;
use crate::components::AppIcon;

#[derive(Clone, Debug)]
struct ProcessRowData {
    name: String,
    pid_str: String,
    connections: u32,
    sent_bytes: u64,
    recv_bytes: u64,
    total_bytes: u64,
    sent_str: String,
    recv_str: String,
    bytes_str: String,
    top_remote: String,
}

fn format_bytes(bytes: u64) -> String {
    if bytes > 1_000_000 {
        format!("{:.1} MB", bytes as f64 / 1_000_000.0)
    } else if bytes > 1_000 {
        format!("{:.1} KB", bytes as f64 / 1_000.0)
    } else {
        format!("{} B", bytes)
    }
}

pub fn TopProcessesTable() -> Element {
    let state = use_context::<AppState>();
    let packets = state.packets.read();
    let search_query = state.search_query.read().clone();

    // Aggregate packets by process_name
    let mut proc_map: HashMap<String, (u32, u32, u64, u64, u64, String)> = HashMap::new();

    for pkt in packets.iter() {
        if !search_query.trim().is_empty() {
            let q = search_query.trim().to_lowercase();
            let clean_q = q.replace("\"", "");

            let matches = pkt.process_name.to_lowercase().contains(&clean_q)
                || pkt.protocol.to_lowercase().contains(&clean_q)
                || pkt.dst_ip.contains(&clean_q)
                || pkt.src_ip.contains(&clean_q)
                || pkt.dst_port.to_string().contains(&clean_q);
            if !matches { continue; }
        }
        let entry = proc_map.entry(pkt.process_name.clone()).or_insert((
            pkt.pid,
            0,
            0,
            0,
            0,
            format!("{}:{}", pkt.dst_ip, pkt.dst_port),
        ));

        entry.1 += 1;
        entry.4 += pkt.pkt_len as u64; // total
        
        // Mock Sent/Recv
        if pkt.src_port > 0 {
            entry.2 += pkt.pkt_len as u64; // sent
        } else {
            entry.3 += pkt.pkt_len as u64; // recv
        }
    }

    let mut rows: Vec<ProcessRowData> = proc_map
        .into_iter()
        .map(|(name, (pid, connections, mut sent_bytes, mut recv_bytes, total_bytes, top_remote))| {
            // Adjust mock data to ensure sent + recv = total if they are 0
            if sent_bytes == 0 && recv_bytes == 0 && total_bytes > 0 {
                sent_bytes = total_bytes / 3;
                recv_bytes = total_bytes - sent_bytes;
            }

            ProcessRowData {
                name,
                pid_str: if pid > 0 { format!("{}", pid) } else { "—".into() },
                connections,
                sent_bytes,
                recv_bytes,
                total_bytes,
                sent_str: format_bytes(sent_bytes),
                recv_str: format_bytes(recv_bytes),
                bytes_str: format_bytes(total_bytes),
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
                            th { class: "px-3 py-1.5 font-normal", "Sent" }
                            th { class: "px-3 py-1.5 font-normal", "Received" }
                            th { class: "px-3 py-1.5 font-normal font-semibold text-kamiki-textPrimary", "Total" }
                            th { class: "px-3 py-1.5 font-normal", "Top Remote" }
                        }
                    }
                    tbody { class: "divide-y divide-kamiki-border/30",
                        if rows.is_empty() {
                            tr {
                                td {
                                    colspan: "7",
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
                                        AppIcon { name: proc.name.clone(), class: "w-4 h-4 shrink-0".to_string() }
                                        span { "{proc.name}" }
                                    }

                                    // PID
                                    td { class: "px-3 py-1.5 font-mono text-kamiki-textSecondary", "{proc.pid_str}" }

                                    // Connections + Sparkline Activity Bars
                                    td { class: "px-3 py-1.5",
                                        div { class: "flex items-center gap-2",
                                            span { class: "font-mono font-medium text-kamiki-textPrimary w-5", "{proc.connections}" }
                                            div { class: "flex items-end gap-[2px] h-3 w-10 opacity-90",
                                                div { class: "w-1 rounded-xs bg-emerald-500", style: "height: {std::cmp::min(100, proc.connections * 2 + 20)}%" }
                                                div { class: "w-1 rounded-xs bg-emerald-400", style: "height: {std::cmp::min(100, proc.connections * 4 + 40)}%" }
                                                div { class: "w-1 rounded-xs bg-emerald-500", style: "height: {std::cmp::min(100, proc.connections * 1 + 60)}%" }
                                                div { class: "w-1 rounded-xs bg-emerald-400", style: "height: {std::cmp::min(100, proc.connections * 3 + 30)}%" }
                                            }
                                        }
                                    }

                                    // Sent
                                    td { class: "px-3 py-1.5 font-mono text-kamiki-textSecondary", "{proc.sent_str}" }
                                    
                                    // Received
                                    td { class: "px-3 py-1.5 font-mono text-kamiki-textSecondary", "{proc.recv_str}" }

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

