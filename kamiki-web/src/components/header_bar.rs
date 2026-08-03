#![allow(non_snake_case)]

use crate::data::state::AppState;
use dioxus::prelude::*;

pub fn HeaderBar() -> Element {
    let mut state = use_context::<AppState>();
    let cap = state.capture.read();
    let uptime = cap.formatted_uptime();
    let is_live = cap.is_live;
    let query_val = state.search_query.read().clone();
    let mut show_menu = use_signal(|| false);

    let presets = [
        ("tcp.port == 443", "HTTPS (443)"),
        ("process == \"firefox\"", "Firefox"),
        ("process == \"discord\"", "Discord"),
        ("port == 53", "DNS (53)"),
        ("port == 22", "SSH (22)"),
        ("protocol == \"UDP\"", "UDP Only"),
        ("10.0.2.15", "Local IP"),
    ];

    rsx! {
        header { class: "h-12 border-b border-kamiki-border bg-kamiki-panel flex items-center justify-between px-3 shrink-0 z-20 select-none text-xs relative",
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
                    if is_live {
                        div { class: "flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-emerald-500/10 text-emerald-400 font-medium border border-emerald-500/20",
                            span { class: "w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" }
                            "LIVE"
                        }
                    } else {
                        div { class: "flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-gray-500/10 text-gray-400 font-medium border border-gray-500/20",
                            span { class: "w-1.5 h-1.5 rounded-full bg-gray-400" }
                            "PAUSED"
                        }
                    }
                    span { class: "font-mono text-kamiki-textPrimary", "{uptime}" }
                    span { class: "text-kamiki-border", "│" }
                    span { class: "text-kamiki-textSecondary",
                        "Events: "
                        span { class: "text-kamiki-textPrimary font-mono font-medium", "{cap.total_events}" }
                    }
                    span { class: "text-kamiki-border", "│" }
                    span { class: "text-kamiki-textSecondary",
                        "Dropped: "
                        span { class: "text-kamiki-textPrimary font-mono font-medium", "{cap.dropped}" }
                    }
                }
            }

            // Center Filter Search Bar
            div { class: "flex-1 max-w-xl mx-4 relative",
                div { class: "relative flex items-center",
                    input {
                        r#type: "text",
                        class: if !query_val.is_empty() {
                            "w-full bg-kamiki-bg border border-kamiki-blue focus:outline-none rounded px-3 py-1.5 text-xs text-kamiki-textPrimary font-mono transition-colors pr-14 shadow-sm"
                        } else {
                            "w-full bg-kamiki-bg border border-kamiki-border focus:border-kamiki-blue focus:outline-none rounded px-3 py-1.5 text-xs text-kamiki-textPrimary placeholder:text-kamiki-textSecondary/60 font-mono transition-colors pr-8"
                        },
                        placeholder: "Filter (e.g. tcp.port == 443 or process == \"firefox\")",
                        value: "{query_val}",
                        oninput: move |evt| state.search_query.set(evt.value()),
                    }

                    // Clear button
                    if !query_val.is_empty() {
                        button {
                            class: "absolute right-7 text-kamiki-textSecondary hover:text-kamiki-textPrimary text-sm px-1 transition-colors",
                            title: "Clear filter",
                            onclick: move |_| state.search_query.set(String::new()),
                            "×"
                        }
                    }

                    // Filter Funnel Icon Button
                    button {
                        class: if *show_menu.read() {
                            "absolute right-2 text-kamiki-blue hover:text-kamiki-blue transition-colors p-0.5 bg-kamiki-blue/15 rounded"
                        } else {
                            "absolute right-2 text-kamiki-textSecondary hover:text-kamiki-textPrimary transition-colors p-0.5"
                        },
                        title: "Filter preset menu",
                        onclick: move |_| show_menu.toggle(),
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

                // Filter Presets Dropdown Popover
                if *show_menu.read() {
                    div { class: "absolute top-full left-0 right-0 mt-1 bg-kamiki-panel border border-kamiki-border rounded-lg shadow-xl p-2.5 z-30 flex flex-col gap-2 select-none",
                        div { class: "flex items-center justify-between border-b border-kamiki-border/60 pb-1.5 text-[11px]",
                            span { class: "font-semibold text-kamiki-textPrimary", "Quick Expression Presets" }
                            button {
                                class: "text-kamiki-textSecondary hover:text-kamiki-textPrimary text-xs",
                                onclick: move |_| show_menu.set(false),
                                "✕"
                            }
                        }
                        div { class: "flex flex-wrap gap-1.5",
                            for (expr, label) in presets.iter() {
                                {
                                    let expr_str = expr.to_string();
                                    rsx! {
                                        button {
                                            key: "{expr_str}",
                                            class: "px-2 py-1 rounded bg-kamiki-bg border border-kamiki-border hover:border-kamiki-blue hover:text-kamiki-blue text-[11px] font-mono text-kamiki-textSecondary transition-colors",
                                            onclick: move |_| {
                                                state.search_query.set(expr_str.clone());
                                                show_menu.set(false);
                                            },
                                            "{label}"
                                        }
                                    }
                                }
                            }
                        }
                        if !query_val.is_empty() {
                            button {
                                class: "w-full text-center py-1 mt-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20 text-[11px] font-medium border border-red-500/30 transition-colors",
                                onclick: move |_| {
                                    state.search_query.set(String::new());
                                    show_menu.set(false);
                                },
                                "Clear Search Query"
                            }
                        }
                    }
                }
            }

            // Right Action Controls
            div { class: "flex items-center gap-1 shrink-0 text-kamiki-textSecondary",
                button {
                    class: "p-1.5 rounded hover:bg-kamiki-panelHover hover:text-kamiki-textPrimary transition-colors",
                    title: "Toggle presets",
                    onclick: move |_| show_menu.toggle(),
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
            }
        }
    }
}
