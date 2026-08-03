#![allow(non_snake_case)]

use crate::api_client::start_capture;
use crate::data::state::{AppState, NavView};
use dioxus::prelude::*;

pub fn LeftSidebar() -> Element {
    let mut state = use_context::<AppState>();
    let filters = state.protocol_counts.read();
    let interfaces = state.interfaces.read();
    let active_iface = state.capture.read().interface.clone();
    let current_view = *state.active_view.read();
    let active_filter = state.selected_filter.read().clone();

    rsx! {
        aside { class: "w-52 border-r border-kamiki-border bg-kamiki-panel flex flex-col shrink-0 overflow-y-auto select-none text-xs p-2 gap-4",
            // Section 1: OVERVIEW Navigation Menu
            div { class: "flex flex-col gap-1",
                div { class: "px-2 py-1 text-[10px] font-semibold text-kamiki-textSecondary/70 uppercase tracking-wider",
                    "Overview"
                }

                // Dashboard
                button {
                    class: if current_view == NavView::Dashboard {
                        "flex items-center gap-2.5 px-2.5 py-1.5 rounded bg-kamiki-blue/15 text-kamiki-blue font-medium border-l-2 border-kamiki-blue transition-colors text-left"
                    } else {
                        "flex items-center gap-2.5 px-2.5 py-1.5 rounded text-kamiki-textSecondary hover:text-kamiki-textPrimary hover:bg-kamiki-panelHover transition-colors text-left"
                    },
                    onclick: move |_| state.active_view.set(NavView::Dashboard),
                    svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", view_box: "0 0 24 24",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" }
                    }
                    span { "Dashboard" }
                }

                // Connections
                button {
                    class: if current_view == NavView::Connections {
                        "flex items-center gap-2.5 px-2.5 py-1.5 rounded bg-kamiki-blue/15 text-kamiki-blue font-medium border-l-2 border-kamiki-blue transition-colors text-left"
                    } else {
                        "flex items-center gap-2.5 px-2.5 py-1.5 rounded text-kamiki-textSecondary hover:text-kamiki-textPrimary hover:bg-kamiki-panelHover transition-colors text-left"
                    },
                    onclick: move |_| state.active_view.set(NavView::Connections),
                    svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", view_box: "0 0 24 24",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1" }
                    }
                    span { "Connections" }
                }

                // Packets
                button {
                    class: if current_view == NavView::Packets {
                        "flex items-center gap-2.5 px-2.5 py-1.5 rounded bg-kamiki-blue/15 text-kamiki-blue font-medium border-l-2 border-kamiki-blue transition-colors text-left"
                    } else {
                        "flex items-center gap-2.5 px-2.5 py-1.5 rounded text-kamiki-textSecondary hover:text-kamiki-textPrimary hover:bg-kamiki-panelHover transition-colors text-left"
                    },
                    onclick: move |_| state.active_view.set(NavView::Packets),
                    svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", view_box: "0 0 24 24",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" }
                    }
                    span { "Packets" }
                }

                // Processes
                button {
                    class: if current_view == NavView::Processes {
                        "flex items-center gap-2.5 px-2.5 py-1.5 rounded bg-kamiki-blue/15 text-kamiki-blue font-medium border-l-2 border-kamiki-blue transition-colors text-left"
                    } else {
                        "flex items-center gap-2.5 px-2.5 py-1.5 rounded text-kamiki-textSecondary hover:text-kamiki-textPrimary hover:bg-kamiki-panelHover transition-colors text-left"
                    },
                    onclick: move |_| state.active_view.set(NavView::Processes),
                    svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", view_box: "0 0 24 24",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" }
                    }
                    span { "Processes" }
                }

                // Events
                button {
                    class: if current_view == NavView::Events {
                        "flex items-center gap-2.5 px-2.5 py-1.5 rounded bg-kamiki-blue/15 text-kamiki-blue font-medium border-l-2 border-kamiki-blue transition-colors text-left"
                    } else {
                        "flex items-center gap-2.5 px-2.5 py-1.5 rounded text-kamiki-textSecondary hover:text-kamiki-textPrimary hover:bg-kamiki-panelHover transition-colors text-left"
                    },
                    onclick: move |_| state.active_view.set(NavView::Events),
                    svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", view_box: "0 0 24 24",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" }
                    }
                    span { "Events" }
                }

                // Timeline
                button {
                    class: if current_view == NavView::Timeline {
                        "flex items-center gap-2.5 px-2.5 py-1.5 rounded bg-kamiki-blue/15 text-kamiki-blue font-medium border-l-2 border-kamiki-blue transition-colors text-left"
                    } else {
                        "flex items-center gap-2.5 px-2.5 py-1.5 rounded text-kamiki-textSecondary hover:text-kamiki-textPrimary hover:bg-kamiki-panelHover transition-colors text-left"
                    },
                    onclick: move |_| state.active_view.set(NavView::Timeline),
                    svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", view_box: "0 0 24 24",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" }
                    }
                    span { "Timeline" }
                }
            }

            // Divider
            div { class: "h-px bg-kamiki-border/60 mx-1" }

            // Section 2: FILTERS (Protocol Counters)
            div { class: "flex flex-col gap-1.5",
                div { class: "px-2 py-0.5 text-[10px] font-semibold text-kamiki-textSecondary/70 uppercase tracking-wider flex items-center justify-between",
                    span { "Filters" }
                    if active_filter.is_some() {
                        button {
                            class: "text-[9px] text-kamiki-blue hover:underline lowercase",
                            onclick: move |_| state.selected_filter.set(None),
                            "clear"
                        }
                    }
                }

                for item in filters.iter() {
                    {
                        let label = item.label.clone();
                        let is_selected = active_filter.as_ref() == Some(&label);

                        rsx! {
                            div {
                                key: "{label}",
                                class: if is_selected {
                                    "flex items-center justify-between px-2.5 py-1 rounded bg-kamiki-blue/20 text-kamiki-textPrimary font-medium border border-kamiki-blue/50 cursor-pointer transition-colors"
                                } else {
                                    "flex items-center justify-between px-2.5 py-1 rounded hover:bg-kamiki-panelHover cursor-pointer transition-colors group"
                                },
                                onclick: move |_| {
                                    if is_selected {
                                        state.selected_filter.set(None);
                                    } else {
                                        state.selected_filter.set(Some(label.clone()));
                                    }
                                },
                                div { class: "flex items-center gap-2",
                                    span { class: "w-2 h-2 rounded-full {item.color_class}" }
                                    span { class: if is_selected { "text-kamiki-textPrimary font-semibold" } else { "text-kamiki-textSecondary group-hover:text-kamiki-textPrimary transition-colors" }, "{label}" }
                                }
                                span { class: "font-mono text-[11px] text-kamiki-textSecondary group-hover:text-kamiki-textPrimary font-medium", "{item.count}" }
                            }
                        }
                    }
                }
            }

            // Divider
            div { class: "h-px bg-kamiki-border/60 mx-1" }

            // Section 3: INTERFACES (Click to start capture)
            div { class: "flex flex-col gap-1.5",
                div { class: "px-2 py-0.5 text-[10px] font-semibold text-kamiki-textSecondary/70 uppercase tracking-wider flex items-center justify-between",
                    span { "Interfaces" }
                }

                for iface in interfaces.iter() {
                    {
                        let name = iface.name.clone();
                        let is_active = active_iface.as_ref() == Some(&name);

                        rsx! {
                            div {
                                key: "{name}",
                                class: if is_active {
                                    "flex items-center justify-between px-2.5 py-1 rounded bg-kamiki-blue/10 border border-kamiki-blue/30 cursor-pointer transition-colors group"
                                } else {
                                    "flex items-center justify-between px-2.5 py-1 rounded hover:bg-kamiki-panelHover cursor-pointer transition-colors group"
                                },
                                onclick: move |_| {
                                    let name_clone = name.clone();
                                    spawn(async move {
                                        let _ = start_capture(name_clone.clone()).await;
                                        let mut cap = state.capture.write();
                                        cap.is_live = true;
                                        cap.interface = Some(name_clone);
                                    });
                                },
                                div { class: "flex items-center gap-2",
                                    span { class: if is_active && state.capture.read().is_live { "w-2 h-2 rounded-full bg-emerald-400 animate-pulse" } else { "w-2 h-2 rounded-full bg-gray-500" } }
                                    span { class: "font-mono font-medium text-kamiki-textPrimary", "{iface.name}" }
                                }
                                span { class: "text-[11px] text-kamiki-textSecondary", "{iface.speed}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
