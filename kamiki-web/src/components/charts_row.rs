#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::data::state::AppState;

pub fn ChartsRow() -> Element {
    let state = use_context::<AppState>();
    let history = state.traffic_history.read();
    let protocol_counts = state.protocol_counts.read();

    // 1. Build Traffic Wave SVG paths for Sent and Received
    let mut sent_pts = Vec::new();
    let mut recv_pts = Vec::new();
    
    let max_sent = history.iter().map(|s| s.sent_bytes_in_window).max().unwrap_or(1).max(1);
    let max_recv = history.iter().map(|s| s.recv_bytes_in_window).max().unwrap_or(1).max(1);
    let max_bytes = max_sent.max(max_recv);

    for (idx, sample) in history.iter().enumerate() {
        let x = (idx as f64 / 59.0) * 200.0;
        
        let sent_y = 55.0 - ((sample.sent_bytes_in_window as f64 / max_bytes as f64) * 45.0);
        sent_pts.push((x, sent_y));

        let recv_y = 55.0 - ((sample.recv_bytes_in_window as f64 / max_bytes as f64) * 45.0);
        recv_pts.push((x, recv_y));
    }

    let mut sent_path = String::new();
    if let Some((x, y)) = sent_pts.first() {
        sent_path.push_str(&format!("M {:.1} {:.1}", x, y));
        for (x, y) in sent_pts.iter().skip(1) { sent_path.push_str(&format!(" L {:.1} {:.1}", x, y)); }
    }
    
    let mut recv_path = String::new();
    if let Some((x, y)) = recv_pts.first() {
        recv_path.push_str(&format!("M {:.1} {:.1}", x, y));
        for (x, y) in recv_pts.iter().skip(1) { recv_path.push_str(&format!(" L {:.1} {:.1}", x, y)); }
    }

    // 2. Compute Donut percentages
    let total_proto_cnt: u32 = protocol_counts.iter().map(|p| p.count).sum();
    let safe_total = if total_proto_cnt == 0 { 1 } else { total_proto_cnt };

    let tcp_count = protocol_counts.iter().find(|p| p.label == "TCP").map(|p| p.count).unwrap_or(0);
    let udp_count = protocol_counts.iter().find(|p| p.label == "UDP").map(|p| p.count).unwrap_or(0);
    let tls_count = protocol_counts.iter().find(|p| p.label == "TLS").map(|p| p.count).unwrap_or(0);
    let icmp_count = protocol_counts.iter().find(|p| p.label == "ICMP").map(|p| p.count).unwrap_or(0);
    let other_count = protocol_counts.iter().find(|p| p.label == "Other").map(|p| p.count).unwrap_or(0);

    let tcp_pct = (tcp_count as f64 / safe_total as f64) * 100.0;
    let udp_pct = (udp_count as f64 / safe_total as f64) * 100.0;
    let tls_pct = (tls_count as f64 / safe_total as f64) * 100.0;
    let icmp_pct = (icmp_count as f64 / safe_total as f64) * 100.0;
    let other_pct = (other_count as f64 / safe_total as f64) * 100.0;

    rsx! {
        div { class: "grid grid-cols-1 lg:grid-cols-3 gap-3 select-none text-xs",
            // Chart 1: Traffic (Total)
            div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex flex-col justify-between h-44 shadow-sm",
                div { class: "font-semibold text-kamiki-textPrimary flex items-center justify-between text-xs mb-1",
                    span { "Traffic (Total)" }
                }

                div { class: "flex-1 flex flex-col justify-between py-1 relative",
                    div { class: "absolute inset-0 top-2 bottom-4 left-10 right-1 flex items-center",
                        svg { class: "w-full h-full overflow-visible", view_box: "0 0 200 60", preserve_aspect_ratio: "none",
                            line { x1: "0", y1: "0", x2: "200", y2: "0", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }
                            line { x1: "0", y1: "20", x2: "200", y2: "20", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }
                            line { x1: "0", y1: "40", x2: "200", y2: "40", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }

                            if !sent_path.is_empty() {
                                path { d: "{sent_path}", fill: "none", stroke: "#3fb950", stroke_width: "2.0" }
                            }
                            if !recv_path.is_empty() {
                                path { d: "{recv_path}", fill: "none", stroke: "#58a6ff", stroke_width: "2.0" }
                            }
                        }
                    }

                    div { class: "flex flex-col justify-between h-full font-mono text-[9px] text-kamiki-textSecondary pointer-events-none z-10",
                        span { "{max_bytes / 1000} KB/s" }
                        span { "{max_bytes / 2000} KB/s" }
                        span { "0 B/s" }
                    }
                }

                div { class: "flex justify-between items-center text-[9px] font-mono text-kamiki-textSecondary pt-2",
                    div { class: "flex gap-4",
                        span { "60s" }
                        span { "45s" }
                        span { "30s" }
                        span { "15s" }
                        span { "now" }
                    }
                    div { class: "flex gap-3",
                        div { class: "flex items-center gap-1", span { class: "w-2 h-0.5 bg-[#3fb950]" }, "Sent" }
                        div { class: "flex items-center gap-1", span { class: "w-2 h-0.5 bg-[#58a6ff]" }, "Received" }
                    }
                }
            }

            // Chart 2: Connections
            div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex flex-col justify-between h-44 shadow-sm overflow-hidden",
                div { class: "font-semibold text-kamiki-textPrimary flex items-center justify-between text-xs mb-1",
                    span { "Connections" }
                }

                div { class: "flex-1 flex flex-col justify-between py-1 relative",
                    div { class: "absolute inset-0 top-2 bottom-4 left-6 right-1 flex items-center",
                        svg { class: "w-full h-full overflow-visible", view_box: "0 0 200 60", preserve_aspect_ratio: "none",
                            line { x1: "0", y1: "0", x2: "200", y2: "0", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }
                            line { x1: "0", y1: "30", x2: "200", y2: "30", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }

                            if !sent_path.is_empty() {
                                path { d: "{sent_path}", fill: "none", stroke: "#3fb950", stroke_width: "1.5" }
                                path { d: "{recv_path}", fill: "none", stroke: "#bc8cff", stroke_width: "1.5" }
                            }
                        }
                    }

                    div { class: "flex flex-col justify-between h-full font-mono text-[9px] text-kamiki-textSecondary pointer-events-none z-10",
                        span { "100" }
                        span { "50" }
                        span { "0" }
                    }
                }

                div { class: "flex justify-between items-center text-[9px] font-mono text-kamiki-textSecondary pt-2",
                    div { class: "flex gap-4",
                        span { "60s" }
                        span { "45s" }
                        span { "30s" }
                        span { "15s" }
                        span { "now" }
                    }
                    div { class: "flex gap-2",
                        div { class: "flex items-center gap-1", span { class: "w-2 h-0.5 bg-[#3fb950]" }, "Established" }
                        div { class: "flex items-center gap-1", span { class: "w-2 h-0.5 bg-[#bc8cff]" }, "Time Wait" }
                        div { class: "flex items-center gap-1", span { class: "w-2 h-0.5 bg-[#58a6ff]" }, "Others" }
                    }
                }
            }

            // Chart 3: Protocols Donut Chart
            div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex flex-col justify-between h-44 shadow-sm overflow-hidden",
                div { class: "font-semibold text-kamiki-textPrimary flex items-center justify-between text-xs mb-1",
                    span { "Protocols" }
                }

                div { class: "flex-1 flex items-center justify-between gap-3 px-1 min-h-0",
                    div { class: "w-20 h-20 relative flex items-center justify-center shrink-0 overflow-hidden",
                        svg { class: "w-full h-full transform -rotate-90", view_box: "0 0 36 36",
                            circle { cx: "18", cy: "18", r: "14", fill: "none", stroke: "#3fb950", stroke_width: "4", path_length: "100", stroke_dasharray: "{tcp_pct} {100.0 - tcp_pct}", stroke_dashoffset: "0" }
                            circle { cx: "18", cy: "18", r: "14", fill: "none", stroke: "#58a6ff", stroke_width: "4", path_length: "100", stroke_dasharray: "{udp_pct} {100.0 - udp_pct}", stroke_dashoffset: "{-tcp_pct}" }
                            circle { cx: "18", cy: "18", r: "14", fill: "none", stroke: "#d29922", stroke_width: "4", path_length: "100", stroke_dasharray: "{tls_pct} {100.0 - tls_pct}", stroke_dashoffset: "{-(tcp_pct + udp_pct)}" }
                            circle { cx: "18", cy: "18", r: "14", fill: "none", stroke: "#f85149", stroke_width: "4", path_length: "100", stroke_dasharray: "{icmp_pct} {100.0 - icmp_pct}", stroke_dashoffset: "{-(tcp_pct + udp_pct + tls_pct)}" }
                            circle { cx: "18", cy: "18", r: "14", fill: "none", stroke: "#8957e5", stroke_width: "4", path_length: "100", stroke_dasharray: "{other_pct} {100.0 - other_pct}", stroke_dashoffset: "{-(tcp_pct + udp_pct + tls_pct + icmp_pct)}" }
                        }
                    }

                    div { class: "flex flex-col gap-1.5 text-[11px] font-mono text-kamiki-textSecondary flex-1 pl-2",
                        div { class: "flex items-center justify-between",
                            div { class: "flex items-center gap-1.5",
                                span { class: "w-2.5 h-2.5 rounded-sm bg-[#3fb950]" }
                                span { class: "font-sans text-kamiki-textPrimary", "TCP" }
                            }
                            span { class: "text-kamiki-textSecondary", "({tcp_pct:.1}%)" }
                        }
                        div { class: "flex items-center justify-between",
                            div { class: "flex items-center gap-1.5",
                                span { class: "w-2.5 h-2.5 rounded-sm bg-[#58a6ff]" }
                                span { class: "font-sans text-kamiki-textPrimary", "UDP" }
                            }
                            span { class: "text-kamiki-textSecondary", "({udp_pct:.1}%)" }
                        }
                        div { class: "flex items-center justify-between",
                            div { class: "flex items-center gap-1.5",
                                span { class: "w-2.5 h-2.5 rounded-sm bg-[#d29922]" }
                                span { class: "font-sans text-kamiki-textPrimary", "TLS" }
                            }
                            span { class: "text-kamiki-textSecondary", "({tls_pct:.1}%)" }
                        }
                        div { class: "flex items-center justify-between",
                            div { class: "flex items-center gap-1.5",
                                span { class: "w-2.5 h-2.5 rounded-sm bg-[#f85149]" }
                                span { class: "font-sans text-kamiki-textPrimary", "ICMP" }
                            }
                            span { class: "text-kamiki-textSecondary", "({icmp_pct:.1}%)" }
                        }
                        div { class: "flex items-center justify-between",
                            div { class: "flex items-center gap-1.5",
                                span { class: "w-2.5 h-2.5 rounded-sm bg-[#8957e5]" }
                                span { class: "font-sans text-kamiki-textPrimary", "Other" }
                            }
                            span { class: "text-kamiki-textSecondary", "({other_pct:.1}%)" }
                        }
                    }
                }
            }
        }
    }
}
