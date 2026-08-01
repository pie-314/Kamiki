#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::data::state::AppState;

fn generate_smooth_path(pts: &[(f64, f64)]) -> (String, String) {
    if pts.is_empty() {
        return (String::new(), String::new());
    }

    let mut stroke_d = format!("M {:.1} {:.1}", pts[0].0, pts[0].1);
    
    for i in 0..pts.len() - 1 {
        let (x0, y0) = pts[i];
        let (x1, y1) = pts[i + 1];
        let cpx1 = x0 + (x1 - x0) * 0.5;
        let cpy1 = y0;
        let cpx2 = x0 + (x1 - x0) * 0.5;
        let cpy2 = y1;
        stroke_d.push_str(&format!(" C {:.1} {:.1}, {:.1} {:.1}, {:.1} {:.1}", cpx1, cpy1, cpx2, cpy2, x1, y1));
    }

    let last_x = pts.last().map(|p| p.0).unwrap_or(200.0);
    let mut area_d = stroke_d.clone();
    area_d.push_str(&format!(" L {:.1} 60.0 L 0.0 60.0 Z", last_x));

    (stroke_d, area_d)
}

pub fn ChartsRow() -> Element {
    let state = use_context::<AppState>();
    let history = state.traffic_history.read();
    let protocol_counts = state.protocol_counts.read();

    let mut traffic_hover = use_signal(|| None::<usize>);
    let mut conn_hover = use_signal(|| None::<usize>);

    // 1. Build Traffic Wave SVG paths for Sent and Received
    let mut sent_pts = Vec::new();
    let mut recv_pts = Vec::new();
    
    let max_sent = history.iter().map(|s| s.sent_bytes_in_window).max().unwrap_or(1).max(1);
    let max_recv = history.iter().map(|s| s.recv_bytes_in_window).max().unwrap_or(1).max(1);
    let max_bytes = max_sent.max(max_recv);

    for (idx, sample) in history.iter().enumerate() {
        let x = (idx as f64 / 59.0) * 200.0;
        let sent_y = 55.0 - ((sample.sent_bytes_in_window as f64 / max_bytes as f64) * 48.0);
        sent_pts.push((x, sent_y));

        let recv_y = 55.0 - ((sample.recv_bytes_in_window as f64 / max_bytes as f64) * 48.0);
        recv_pts.push((x, recv_y));
    }

    let (sent_stroke, sent_area) = generate_smooth_path(&sent_pts);
    let (recv_stroke, recv_area) = generate_smooth_path(&recv_pts);

    // 2. Compute Donut percentages and Arc Segment Lengths
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

    let circ = 226.19467f64;
    let tcp_len = (tcp_pct / 100.0) * circ;
    let udp_len = (udp_pct / 100.0) * circ;
    let tls_len = (tls_pct / 100.0) * circ;
    let icmp_len = (icmp_pct / 100.0) * circ;
    let other_len = (other_pct / 100.0) * circ;

    let tcp_off = 0.0f64;
    let udp_off = -tcp_len;
    let tls_off = -(tcp_len + udp_len);
    let icmp_off = -(tcp_len + udp_len + tls_len);
    let other_off = -(tcp_len + udp_len + tls_len + icmp_len);

    rsx! {
        div { class: "grid grid-cols-1 lg:grid-cols-3 gap-3 select-none text-xs",
            // Chart 1: Traffic (Total)
            div {
                class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex flex-col justify-between h-44 shadow-sm relative overflow-hidden group",
                onmousemove: move |evt| {
                    let coords = evt.element_coordinates();
                    let idx = ((coords.x / 280.0) * 59.0).clamp(0.0, 59.0) as usize;
                    traffic_hover.set(Some(idx));
                },
                onmouseleave: move |_| traffic_hover.set(None),

                div { class: "font-semibold text-kamiki-textPrimary flex items-center justify-between text-xs mb-1 z-10",
                    span { "Traffic (Total)" }
                    if let Some(h_idx) = *traffic_hover.read() {
                        if let Some(sample) = history.get(h_idx) {
                            span { class: "font-mono text-[10px] text-kamiki-blue bg-kamiki-blue/10 px-1.5 py-0.5 rounded border border-kamiki-blue/30",
                                "Sent: {sample.sent_bytes_in_window / 1024}KB/s | Recv: {sample.recv_bytes_in_window / 1024}KB/s"
                            }
                        }
                    }
                }

                div { class: "flex-1 flex flex-col justify-between py-1 relative",
                    div { class: "absolute inset-0 top-2 bottom-4 left-10 right-1 flex items-center",
                        svg { class: "w-full h-full overflow-visible", view_box: "0 0 200 60", preserve_aspect_ratio: "none",
                            defs {
                                linearGradient { id: "sentGrad", x1: "0", y1: "0", x2: "0", y2: "1",
                                    stop { offset: "0%", stop_color: "#3fb950", stop_opacity: "0.35" }
                                    stop { offset: "100%", stop_color: "#3fb950", stop_opacity: "0.0" }
                                }
                                linearGradient { id: "recvGrad", x1: "0", y1: "0", x2: "0", y2: "1",
                                    stop { offset: "0%", stop_color: "#58a6ff", stop_opacity: "0.35" }
                                    stop { offset: "100%", stop_color: "#58a6ff", stop_opacity: "0.0" }
                                }
                            }

                            line { x1: "0", y1: "0", x2: "200", y2: "0", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }
                            line { x1: "0", y1: "20", x2: "200", y2: "20", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }
                            line { x1: "0", y1: "40", x2: "200", y2: "40", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }

                            if !sent_area.is_empty() {
                                path { d: "{sent_area}", fill: "url(#sentGrad)" }
                            }
                            if !recv_area.is_empty() {
                                path { d: "{recv_area}", fill: "url(#recvGrad)" }
                            }

                            if !sent_stroke.is_empty() {
                                path { d: "{sent_stroke}", fill: "none", stroke: "#3fb950", stroke_width: "2.0" }
                            }
                            if !recv_stroke.is_empty() {
                                path { d: "{recv_stroke}", fill: "none", stroke: "#58a6ff", stroke_width: "2.0" }
                            }

                            if let Some(h_idx) = *traffic_hover.read() {
                                {
                                    let h_x = (h_idx as f64 / 59.0) * 200.0;
                                    let sent_y = sent_pts.get(h_idx).map(|p| p.1).unwrap_or(30.0);
                                    let recv_y = recv_pts.get(h_idx).map(|p| p.1).unwrap_or(30.0);

                                    rsx! {
                                        line { x1: "{h_x}", y1: "0", x2: "{h_x}", y2: "60", stroke: "#58a6ff", stroke_dasharray: "2 2", stroke_width: "1" }
                                        circle { cx: "{h_x}", cy: "{sent_y}", r: "3", fill: "#3fb950", stroke: "#ffffff", stroke_width: "1" }
                                        circle { cx: "{h_x}", cy: "{recv_y}", r: "3", fill: "#58a6ff", stroke: "#ffffff", stroke_width: "1" }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "flex flex-col justify-between h-full font-mono text-[9px] text-kamiki-textSecondary pointer-events-none z-10",
                        span { "{max_bytes / 1000} KB/s" }
                        span { "{max_bytes / 2000} KB/s" }
                        span { "0 B/s" }
                    }
                }

                div { class: "flex justify-between items-center text-[9px] font-mono text-kamiki-textSecondary pt-2 z-10",
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
            div {
                class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex flex-col justify-between h-44 shadow-sm overflow-hidden relative group",
                onmousemove: move |evt| {
                    let coords = evt.element_coordinates();
                    let idx = ((coords.x / 280.0) * 59.0).clamp(0.0, 59.0) as usize;
                    conn_hover.set(Some(idx));
                },
                onmouseleave: move |_| conn_hover.set(None),

                div { class: "font-semibold text-kamiki-textPrimary flex items-center justify-between text-xs mb-1 z-10",
                    span { "Connections" }
                    if let Some(h_idx) = *conn_hover.read() {
                        if let Some(sample) = history.get(h_idx) {
                            span { class: "font-mono text-[10px] text-[#bc8cff] bg-[#bc8cff]/10 px-1.5 py-0.5 rounded border border-[#bc8cff]/30",
                                "Flows: {sample.active_flows} active"
                            }
                        }
                    }
                }

                div { class: "flex-1 flex flex-col justify-between py-1 relative",
                    div { class: "absolute inset-0 top-2 bottom-4 left-6 right-1 flex items-center",
                        svg { class: "w-full h-full overflow-visible", view_box: "0 0 200 60", preserve_aspect_ratio: "none",
                            defs {
                                linearGradient { id: "connGrad", x1: "0", y1: "0", x2: "0", y2: "1",
                                    stop { offset: "0%", stop_color: "#bc8cff", stop_opacity: "0.35" }
                                    stop { offset: "100%", stop_color: "#bc8cff", stop_opacity: "0.0" }
                                }
                            }

                            line { x1: "0", y1: "0", x2: "200", y2: "0", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }
                            line { x1: "0", y1: "30", x2: "200", y2: "30", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }

                            if !recv_area.is_empty() {
                                path { d: "{recv_area}", fill: "url(#connGrad)" }
                            }

                            if !sent_stroke.is_empty() {
                                path { d: "{sent_stroke}", fill: "none", stroke: "#3fb950", stroke_width: "1.5" }
                            }
                            if !recv_stroke.is_empty() {
                                path { d: "{recv_stroke}", fill: "none", stroke: "#bc8cff", stroke_width: "1.5" }
                            }

                            if let Some(h_idx) = *conn_hover.read() {
                                {
                                    let h_x = (h_idx as f64 / 59.0) * 200.0;
                                    let conn_y = recv_pts.get(h_idx).map(|p| p.1).unwrap_or(30.0);

                                    rsx! {
                                        line { x1: "{h_x}", y1: "0", x2: "{h_x}", y2: "60", stroke: "#bc8cff", stroke_dasharray: "2 2", stroke_width: "1" }
                                        circle { cx: "{h_x}", cy: "{conn_y}", r: "3", fill: "#bc8cff", stroke: "#ffffff", stroke_width: "1" }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "flex flex-col justify-between h-full font-mono text-[9px] text-kamiki-textSecondary pointer-events-none z-10",
                        span { "100" }
                        span { "50" }
                        span { "0" }
                    }
                }

                div { class: "flex justify-between items-center text-[9px] font-mono text-kamiki-textSecondary pt-2 z-10",
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
                    div { class: "w-24 h-24 relative flex items-center justify-center shrink-0 overflow-hidden p-1",
                        svg { class: "w-full h-full transform -rotate-90", view_box: "0 0 100 100",
                            // Track circle
                            circle { cx: "50", cy: "50", r: "36", fill: "none", stroke: "#21262d", stroke_width: "10" }

                            if tcp_len > 0.0 {
                                circle { cx: "50", cy: "50", r: "36", fill: "none", stroke: "#3fb950", stroke_width: "10", stroke_dasharray: "{tcp_len} {circ - tcp_len}", stroke_dashoffset: "{tcp_off}" }
                            }
                            if udp_len > 0.0 {
                                circle { cx: "50", cy: "50", r: "36", fill: "none", stroke: "#58a6ff", stroke_width: "10", stroke_dasharray: "{udp_len} {circ - udp_len}", stroke_dashoffset: "{udp_off}" }
                            }
                            if tls_len > 0.0 {
                                circle { cx: "50", cy: "50", r: "36", fill: "none", stroke: "#d29922", stroke_width: "10", stroke_dasharray: "{tls_len} {circ - tls_len}", stroke_dashoffset: "{tls_off}" }
                            }
                            if icmp_len > 0.0 {
                                circle { cx: "50", cy: "50", r: "36", fill: "none", stroke: "#f85149", stroke_width: "10", stroke_dasharray: "{icmp_len} {circ - icmp_len}", stroke_dashoffset: "{icmp_off}" }
                            }
                            if other_len > 0.0 {
                                circle { cx: "50", cy: "50", r: "36", fill: "none", stroke: "#8957e5", stroke_width: "10", stroke_dasharray: "{other_len} {circ - other_len}", stroke_dashoffset: "{other_off}" }
                            }
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
