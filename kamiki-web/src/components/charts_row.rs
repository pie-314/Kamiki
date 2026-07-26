#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn ChartsRow() -> Element {
    rsx! {
        div { class: "grid grid-cols-1 lg:grid-cols-3 gap-3 select-none text-xs",
            // ⑦ Chart 1: Traffic (Total)
            div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex flex-col justify-between h-44 shadow-sm",
                // Title
                div { class: "font-semibold text-kamiki-textPrimary flex items-center justify-between text-xs mb-1",
                    span { "Traffic (Total)" }
                }

                // Chart Viewport (Y-Axis + SVG Line/Area Graph + X-Axis)
                div { class: "flex-1 flex flex-col justify-between py-1 relative",
                    // SVG Wave Graphs
                    div { class: "absolute inset-0 top-3 bottom-4 left-10 right-1 flex items-center",
                        svg { class: "w-full h-full overflow-visible", view_box: "0 0 200 60", preserve_aspect_ratio: "none",
                            // Grid lines
                            line { x1: "0", y1: "0", x2: "200", y2: "0", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }
                            line { x1: "0", y1: "20", x2: "200", y2: "20", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }
                            line { x1: "0", y1: "40", x2: "200", y2: "40", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }

                            // Sent (Blue Area + Line)
                            path {
                                d: "M 0 45 Q 25 20, 50 35 T 100 25 T 150 40 T 200 15 L 200 60 L 0 60 Z",
                                fill: "rgba(88, 166, 255, 0.15)",
                            }
                            path {
                                d: "M 0 45 Q 25 20, 50 35 T 100 25 T 150 40 T 200 15",
                                fill: "none",
                                stroke: "#58a6ff",
                                stroke_width: "1.5",
                            }

                            // Received (Green Area + Line)
                            path {
                                d: "M 0 50 Q 30 30, 60 40 T 110 35 T 160 20 T 200 30 L 200 60 L 0 60 Z",
                                fill: "rgba(63, 185, 80, 0.15)",
                            }
                            path {
                                d: "M 0 50 Q 30 30, 60 40 T 110 35 T 160 20 T 200 30",
                                fill: "none",
                                stroke: "#3fb950",
                                stroke_width: "1.5",
                            }
                        }
                    }

                    // Y-Axis labels
                    div { class: "flex flex-col justify-between h-full font-mono text-[9px] text-kamiki-textSecondary pointer-events-none z-10",
                        span { "1.5 MB/s" }
                        span { "1.0 MB/s" }
                        span { "500 KB/s" }
                        span { "0 B/s" }
                    }
                }

                // X-Axis labels
                div { class: "flex justify-between pl-10 pr-1 text-[9px] font-mono text-kamiki-textSecondary pt-1 border-t border-kamiki-border/40",
                    span { "60s" }
                    span { "45s" }
                    span { "30s" }
                    span { "15s" }
                    span { "now" }
                }

                // Legend
                div { class: "flex items-center justify-center gap-4 text-[10px] text-kamiki-textSecondary pt-1",
                    div { class: "flex items-center gap-1.5",
                        span { class: "w-2.5 h-0.5 bg-[#3fb950] rounded" }
                        span { "Sent" }
                    }
                    div { class: "flex items-center gap-1.5",
                        span { class: "w-2.5 h-0.5 bg-[#58a6ff] rounded" }
                        span { "Received" }
                    }
                }
            }

            // ⑧ Chart 2: Connections
            div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex flex-col justify-between h-44 shadow-sm",
                // Title
                div { class: "font-semibold text-kamiki-textPrimary flex items-center justify-between text-xs mb-1",
                    span { "Connections" }
                }

                // Chart Viewport
                div { class: "flex-1 flex flex-col justify-between py-1 relative",
                    // SVG Line Graph
                    div { class: "absolute inset-0 top-3 bottom-4 left-6 right-1 flex items-center",
                        svg { class: "w-full h-full overflow-visible", view_box: "0 0 200 60", preserve_aspect_ratio: "none",
                            line { x1: "0", y1: "0", x2: "200", y2: "0", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }
                            line { x1: "0", y1: "30", x2: "200", y2: "30", stroke: "#30363d", stroke_dasharray: "2 2", stroke_width: "0.5" }

                            // Established (Green line)
                            path {
                                d: "M 0 35 Q 20 25, 40 30 T 80 20 T 120 28 T 160 18 T 200 22",
                                fill: "none",
                                stroke: "#3fb950",
                                stroke_width: "1.5",
                            }

                            // Time Wait (Purple line)
                            path {
                                d: "M 0 52 Q 25 48, 50 50 T 100 45 T 150 49 T 200 46",
                                fill: "none",
                                stroke: "#bc8cff",
                                stroke_width: "1.5",
                            }

                            // Others (Blue line)
                            path {
                                d: "M 0 58 Q 30 55, 60 57 T 120 54 T 200 56",
                                fill: "none",
                                stroke: "#58a6ff",
                                stroke_width: "1.5",
                            }
                        }
                    }

                    // Y-Axis labels
                    div { class: "flex flex-col justify-between h-full font-mono text-[9px] text-kamiki-textSecondary pointer-events-none z-10",
                        span { "80" }
                        span { "40" }
                        span { "0" }
                    }
                }

                // X-Axis labels
                div { class: "flex justify-between pl-6 pr-1 text-[9px] font-mono text-kamiki-textSecondary pt-1 border-t border-kamiki-border/40",
                    span { "60s" }
                    span { "45s" }
                    span { "30s" }
                    span { "15s" }
                    span { "now" }
                }

                // Legend
                div { class: "flex items-center justify-center gap-3 text-[10px] text-kamiki-textSecondary pt-1",
                    div { class: "flex items-center gap-1.5",
                        span { class: "w-2.5 h-0.5 bg-[#3fb950] rounded" }
                        span { "Established" }
                    }
                    div { class: "flex items-center gap-1.5",
                        span { class: "w-2.5 h-0.5 bg-[#bc8cff] rounded" }
                        span { "Time Wait" }
                    }
                    div { class: "flex items-center gap-1.5",
                        span { class: "w-2.5 h-0.5 bg-[#58a6ff] rounded" }
                        span { "Others" }
                    }
                }
            }

            // ⑨ Chart 3: Protocols Donut Chart
            div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg p-3 flex flex-col justify-between h-44 shadow-sm",
                // Title
                div { class: "font-semibold text-kamiki-textPrimary flex items-center justify-between text-xs mb-1",
                    span { "Protocols" }
                }

                // Content: Donut SVG + Legend List Side-by-Side
                div { class: "flex-1 flex items-center justify-between gap-2 px-2",
                    // SVG Donut Chart
                    div { class: "w-24 h-24 relative flex items-center justify-center shrink-0",
                        svg { class: "w-full h-full transform -rotate-90", view_box: "0 0 36 36",
                            // Donut Slices (stroke-dasharray & stroke-dashoffset)
                            // Circle 1: TCP 82.2% (green #3fb950)
                            circle { cx: "18", cy: "18", r: "14", fill: "none", stroke: "#3fb950", stroke_width: "5", stroke_dasharray: "72.3 27.7", stroke_dashoffset: "0" }
                            // Circle 2: UDP 10.5% (blue #58a6ff)
                            circle { cx: "18", cy: "18", r: "14", fill: "none", stroke: "#58a6ff", stroke_width: "5", stroke_dasharray: "9.2 90.8", stroke_dashoffset: "-72.3" }
                            // Circle 3: TLS 4.1% (yellow #d29922)
                            circle { cx: "18", cy: "18", r: "14", fill: "none", stroke: "#d29922", stroke_width: "5", stroke_dasharray: "3.6 96.4", stroke_dashoffset: "-81.5" }
                            // Circle 4: ICMP 1.0% (red #f85149)
                            circle { cx: "18", cy: "18", r: "14", fill: "none", stroke: "#f85149", stroke_width: "5", stroke_dasharray: "0.9 99.1", stroke_dashoffset: "-85.1" }
                            // Circle 5: Other 2.2% (purple #bc8cff)
                            circle { cx: "18", cy: "18", r: "14", fill: "none", stroke: "#bc8cff", stroke_width: "5", stroke_dasharray: "1.9 98.1", stroke_dashoffset: "-86.0" }
                        }
                    }

                    // Legend List
                    div { class: "flex flex-col gap-1 text-[11px] font-mono text-kamiki-textSecondary flex-1 pl-2",
                        div { class: "flex items-center justify-between",
                            div { class: "flex items-center gap-1.5",
                                span { class: "w-2 h-2 rounded-xs bg-[#3fb950]" }
                                span { class: "font-sans text-kamiki-textPrimary", "TCP" }
                            }
                            span { class: "text-kamiki-textSecondary", "(82.2%)" }
                        }
                        div { class: "flex items-center justify-between",
                            div { class: "flex items-center gap-1.5",
                                span { class: "w-2 h-2 rounded-xs bg-[#58a6ff]" }
                                span { class: "font-sans text-kamiki-textPrimary", "UDP" }
                            }
                            span { class: "text-kamiki-textSecondary", "(10.5%)" }
                        }
                        div { class: "flex items-center justify-between",
                            div { class: "flex items-center gap-1.5",
                                span { class: "w-2 h-2 rounded-xs bg-[#d29922]" }
                                span { class: "font-sans text-kamiki-textPrimary", "TLS" }
                            }
                            span { class: "text-kamiki-textSecondary", "(4.1%)" }
                        }
                        div { class: "flex items-center justify-between",
                            div { class: "flex items-center gap-1.5",
                                span { class: "w-2 h-2 rounded-xs bg-[#f85149]" }
                                span { class: "font-sans text-kamiki-textPrimary", "ICMP" }
                            }
                            span { class: "text-kamiki-textSecondary", "(1.0%)" }
                        }
                        div { class: "flex items-center justify-between",
                            div { class: "flex items-center gap-1.5",
                                span { class: "w-2 h-2 rounded-xs bg-[#bc8cff]" }
                                span { class: "font-sans text-kamiki-textPrimary", "Other" }
                            }
                            span { class: "text-kamiki-textSecondary", "(2.2%)" }
                        }
                    }
                }
            }
        }
    }
}
