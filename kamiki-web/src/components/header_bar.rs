#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::data::dummy::get_capture_state;

pub fn HeaderBar() -> Element {
    let capture = use_signal(get_capture_state);
    let cap = capture.read();

    rsx! {
        header { class: "h-12 border-b border-kamiki-border bg-kamiki-panel flex items-center justify-between px-3 shrink-0 z-10 select-none text-xs",
            // Left Branding & Live Stats
            div { class: "flex items-center gap-4 shrink-0",
                // Logo
                div { class: "flex items-center gap-2",
                    div { class: "w-7 h-7 rounded bg-gradient-to-br from-indigo-500 via-purple-600 to-pink-500 flex items-center justify-center text-white font-bold text-base shadow-sm",
                        "K"
                    }
                    div { class: "flex items-baseline gap-2",
                        span { class: "font-semibold text-sm text-kamiki-textPrimary tracking-tight", "Kamiki" }
                        span { class: "text-[11px] text-kamiki-textSecondary hidden md:inline", "eBPF Network Observability" }
                    }
                }

                // Live Indicator & Capture Counters
                div { class: "flex items-center gap-3 bg-kamiki-bg/60 border border-kamiki-border px-2.5 py-1 rounded-full text-[11px]",
                    // Pulse Badge
                    div { class: "flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-emerald-500/10 text-emerald-400 font-medium border border-emerald-500/20",
                        span { class: "w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" }
                        "LIVE"
                    }
                    // Uptime
                    span { class: "font-mono text-kamiki-textPrimary", "{cap.uptime}" }
                    span { class: "text-kamiki-border", "│" }
                    // Events
                    span { class: "text-kamiki-textSecondary",
                        "Events: "
                        span { class: "text-kamiki-textPrimary font-mono font-medium", "{cap.events}" }
                    }
                    span { class: "text-kamiki-border", "│" }
                    // Dropped
                    span { class: "text-kamiki-textSecondary",
                        "Dropped: "
                        span { class: "text-kamiki-textPrimary font-mono font-medium", "{cap.dropped}" }
                    }
                }
            }

            // Center Filter Search Bar
            div { class: "flex-1 max-w-xl mx-4",
                div { class: "relative flex items-center",
                    input {
                        r#type: "text",
                        class: "w-full bg-kamiki-bg border border-kamiki-border focus:border-kamiki-blue focus:outline-none rounded px-3 py-1.5 text-xs text-kamiki-textPrimary placeholder:text-kamiki-textSecondary/60 font-mono transition-colors pr-8",
                        placeholder: "Filter (e.g. tcp.port == 443 or process == \"firefox\")",
                    }
                    // Filter Icon right
                    button {
                        class: "absolute right-2 text-kamiki-textSecondary hover:text-kamiki-textPrimary transition-colors p-0.5",
                        title: "Filter settings",
                        svg {
                            class: "w-3.5 h-3.5",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z"
                            }
                        }
                    }
                }
            }

            // Right Action Controls
            div { class: "flex items-center gap-1 shrink-0 text-kamiki-textSecondary",
                button {
                    class: "p-1.5 rounded hover:bg-kamiki-panelHover hover:text-kamiki-textPrimary transition-colors",
                    title: "Filter Options",
                    svg {
                        class: "w-4 h-4",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"
                        }
                    }
                }
                button {
                    class: "p-1.5 rounded hover:bg-kamiki-panelHover hover:text-kamiki-textPrimary transition-colors",
                    title: "Settings",
                    svg {
                        class: "w-4 h-4",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                        }
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                        }
                    }
                }
            }
        }
    }
}
