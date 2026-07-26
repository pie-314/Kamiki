#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::data::state::AppState;

pub fn ChartsRow() -> Element {
    let state = use_context::<AppState>();
    let history = state.traffic_history.read();
    let protocol_counts = state.protocol_counts.read();

    // 1. Build Traffic Wave SVG path dynamically
    let mut traffic_pts = Vec::new();
    let max_bytes = history.iter().map(|s| s.bytes_in_window).max().unwrap_or(1).max(1);

    for (idx, sample) in history.iter().enumerate() {
        let x = (idx as f64 / 59.0) * 200.0;
        let norm_y = (sample.bytes_in_window as f64 / max_bytes as f64) * 45.0;
        let y = 55.0 - norm_y;
        traffic_pts.push((x, y));
    }

    let mut path_d = String::new();
    if let Some((first_x, first_y)) = traffic_pts.first() {
        path_d.push_str(&format!("M {:.1} {:.1}", first_x, first_y));
        for (x, y) in traffic_pts.iter().skip(1) {
            path_d.push_str(&format!(" L {:.1} {:.1}", x, y));
        }
    }
    let mut area_d = path_d.clone();
    area_d.push_str(" L 200 60 L 0 60 Z");

    // 2. Compute Donut percentages
    let total_proto_cnt: u32 = protocol_counts.iter().map(|p| p.count).sum();
    let safe_total = if total_proto_cnt == 0 { 1 } else { total_proto_cnt };

    let tcp_count = protocol_counts.iter().find(|p| p.label == "TCP").map(|p| p.count).unwrap_or(0);
    let udp_count = protocol_counts.iter().find(|p| p.label == "UDP").map(|p| p.count).unwrap_or(0);
    let icmp_count = protocol_counts.iter().find(|p| p.label == "ICMP").map(|p| p.count).unwrap_or(0);
    let other_count = protocol_counts.iter().find(|p| p.label == "Other").map(|p| p.count).unwrap_or(0);

    let tcp_pct = (tcp_count as f64 / safe_total as f64) * 100.0;
    let udp_pct = (udp_count as f64 / safe_total as f64) * 100.0;
    let icmp_pct = (icmp_count as f64 / safe_total as f64) * 100.0;
    let other_pct = (other_count as f64 / safe_total as f64) * 100.0;

    rsx! {
        div { class: "grid grid-cols-1 lg:grid-cols-3 gap-3 select-none text-xs",
            // Chart 1: Traffic (Total)
            div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex flex-col justify-between h-44 shadow-sm",
                div { class: "font-semibold text-kamiki-textPrimary flex items-center justify-between text-xs mb-1",
                    span { "Traffic (Live Volume)" }
                }

                div { class: "flex-1 flex flex-col justify-between py-1 relative",
                    div { class: "absolute inset-0 top-3 bottom-4 left-10 right-1 flex items-center",
                        svg { class: "w-full h-full overflow-visible", view_box: "0 0 200 60", preserve_aspect_ratio: "none",
                            line { x1: "0", y1: "0", x2: "200", y2: "0", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }
                            line { x1: "0", y1: "20", x2: "200", y2: "20", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }
                            line { x1: "0", y1: "40", x2: "200", y2: "40", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }

                            if !path_d.is_empty() {
                                path { d: "{area_d}", fill: "rgba(88, 166, 255, 0.15)" }
                                path { d: "{path_d}", fill: "none", stroke: "#58a6ff", stroke_width: "1.5" }
                            }
                        }
                    }

                    div { class: "flex flex-col justify-between h-full font-mono text-[9px] text-kamiki-textSecondary pointer-events-none z-10",
                        span { "{max_bytes} B" }
                        span { "{max_bytes / 2} B" }
                        span { "0 B" }
                    }
                }

                div { class: "flex justify-between pl-10 pr-1 text-[9px] font-mono text-kamiki-textSecondary pt-1 border-t border-kamiki-border/40",
                    span { "60s" }
                    span { "45s" }
                    span { "30s" }
                    span { "15s" }
                    span { "now" }
                }
            }

            // Chart 2: Connections
            div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex flex-col justify-between h-44 shadow-sm",
                div { class: "font-semibold text-kamiki-textPrimary flex items-center justify-between text-xs mb-1",
                    span { "Active Flows" }
                }

                div { class: "flex-1 flex flex-col justify-between py-1 relative",
                    div { class: "absolute inset-0 top-3 bottom-4 left-6 right-1 flex items-center",
                        svg { class: "w-full h-full overflow-visible", view_box: "0 0 200 60", preserve_aspect_ratio: "none",
                            line { x1: "0", y1: "0", x2: "200", y2: "0", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }
                            line { x1: "0", y1: "30", x2: "200", y2: "30", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }

                            if !path_d.is_empty() {
                                path { d: "{path_d}", fill: "none", stroke: "#3fb950", stroke_width: "1.5" }
                            }
                        }
                    }

                    div { class: "flex flex-col justify-between h-full font-mono text-[9px] text-kamiki-textSecondary pointer-events-none z-10",
                        span { "Active" }
                        span { "Idle" }
                        span { "0" }
                    }
                }

                div { class: "flex justify-between pl-6 pr-1 text-[9px] font-mono text-kamiki-textSecondary pt-1 border-t border-kamiki-border/40",
                    span { "60s" }
                    span { "45s" }
                    span { "30s" }
                    span { "15s" }
                    span { "now" }
                }
            }

            // Chart 3: Protocols Donut Chart
            div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex flex-col justify-between h-44 shadow-sm",
                div { class: "font-semibold text-kamiki-textPrimary flex items-center justify-between text-xs mb-1",
                    span { "Protocols" }
                }

                div { class: "flex-1 flex items-center justify-between gap-2 px-2",
                    div { class: "w-24 h-24 relative flex items-center justify-center shrink-0",
                        svg { class: "w-full h-full transform -rotate-90", view_box: "0 0 36 36",
                            circle { cx: "18", cy: "18", r: "14", fill: "none", stroke: "#3fb950", stroke_width: "5", stroke_dasharray: "{tcp_pct} {100.0 - tcp_pct}", stroke_dashoffset: "0" }
                            circle { cx: "18", cy: "18", r: "14", fill: "none", stroke: "#58a6ff", stroke_width: "5", stroke_dasharray: "{udp_pct} {100.0 - udp_pct}", stroke_dashoffset: "{-tcp_pct}" }
                            circle { cx: "18", cy: "18", r: "14", fill: "none", stroke: "#f85149", stroke_width: "5", stroke_dasharray: "{icmp_pct} {100.0 - icmp_pct}", stroke_dashoffset: "{-(tcp_pct + udp_pct)}" }
                        }
                    }

                    div { class: "flex flex-col gap-1 text-[11px] font-mono text-kamiki-textSecondary flex-1 pl-2",
                        div { class: "flex items-center justify-between",
                            div { class: "flex items-center gap-1.5",
                                span { class: "w-2 h-2 rounded-xs bg-[#3fb950]" }
                                span { class: "font-sans text-kamiki-textPrimary", "TCP" }
                            }
                            span { class: "text-kamiki-textSecondary", "({tcp_pct:.1}%)" }
                        }
                        div { class: "flex items-center justify-between",
                            div { class: "flex items-center gap-1.5",
                                span { class: "w-2 h-2 rounded-xs bg-[#58a6ff]" }
                                span { class: "font-sans text-kamiki-textPrimary", "UDP" }
                            }
                            span { class: "text-kamiki-textSecondary", "({udp_pct:.1}%)" }
                        }
                        div { class: "flex items-center justify-between",
                            div { class: "flex items-center gap-1.5",
                                span { class: "w-2 h-2 rounded-xs bg-[#f85149]" }
                                span { class: "font-sans text-kamiki-textPrimary", "ICMP" }
                            }
                            span { class: "text-kamiki-textSecondary", "({icmp_pct:.1}%)" }
                        }
                        div { class: "flex items-center justify-between",
                            div { class: "flex items-center gap-1.5",
                                span { class: "w-2 h-2 rounded-xs bg-[#bc8cff]" }
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
