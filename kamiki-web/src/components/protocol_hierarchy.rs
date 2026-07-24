#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn ProtocolHierarchy() -> Element {
    let mut eth_open = use_signal(|| true);
    let mut ip_open = use_signal(|| true);
    let mut tcp_open = use_signal(|| true);
    let mut tls_open = use_signal(|| true);

    rsx! {
        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg overflow-hidden flex flex-col shadow-sm select-none text-xs",
            // Header Bar
            div { class: "px-3 py-2 border-b border-kamiki-border/80 bg-kamiki-panel/50 flex items-center justify-between",
                span { class: "font-semibold text-kamiki-textPrimary tracking-tight", "Protocol Hierarchy" }
                button { class: "p-0.5 text-kamiki-textSecondary hover:text-kamiki-textPrimary transition-colors", title: "Expand Tree",
                    "⛶"
                }
            }

            // Tree Content
            div { class: "p-2.5 flex flex-col gap-1 font-mono text-[11px] text-kamiki-textSecondary",
                // Level 1: Ethernet II
                div { class: "flex flex-col gap-1",
                    div {
                        class: "flex items-center gap-1.5 cursor-pointer hover:text-kamiki-textPrimary transition-colors",
                        onclick: move |_| eth_open.toggle(),
                        span { class: "text-[9px] w-3 text-center", if *eth_open.read() { "▼" } else { "▶" } }
                        span { class: "font-sans font-medium text-kamiki-textPrimary", "Ethernet II" }
                    }

                    if *eth_open.read() {
                        // Level 2: IPv4
                        div { class: "pl-3 flex flex-col gap-1 border-l border-kamiki-border/40 ml-1.5",
                            div {
                                class: "flex items-center gap-1.5 cursor-pointer hover:text-kamiki-textPrimary transition-colors",
                                onclick: move |_| ip_open.toggle(),
                                span { class: "text-[9px] w-3 text-center", if *ip_open.read() { "▼" } else { "▶" } }
                                span { class: "font-sans font-medium text-kamiki-textPrimary", "IPv4" }
                            }

                            if *ip_open.read() {
                                // Level 3: TCP
                                div { class: "pl-3 flex flex-col gap-1 border-l border-kamiki-border/40 ml-1.5",
                                    div {
                                        class: "flex items-center gap-1.5 cursor-pointer hover:text-kamiki-textPrimary transition-colors",
                                        onclick: move |_| tcp_open.toggle(),
                                        span { class: "text-[9px] w-3 text-center", if *tcp_open.read() { "▼" } else { "▶" } }
                                        span { class: "font-sans font-medium text-kamiki-textPrimary", "TCP" }
                                    }

                                    if *tcp_open.read() {
                                        // Level 4: TLSv1.3
                                        div { class: "pl-3 flex flex-col gap-1 border-l border-kamiki-border/40 ml-1.5",
                                            div {
                                                class: "flex items-center gap-1.5 cursor-pointer hover:text-kamiki-textPrimary transition-colors",
                                                onclick: move |_| tls_open.toggle(),
                                                span { class: "text-[9px] w-3 text-center", if *tls_open.read() { "▼" } else { "▶" } }
                                                span { class: "font-sans font-medium text-kamiki-textPrimary", "TLSv1.3" }
                                            }

                                            if *tls_open.read() {
                                                // Level 5: Application Data
                                                div { class: "pl-4 text-kamiki-textSecondary hover:text-kamiki-textPrimary flex items-center gap-1.5 cursor-pointer py-0.5",
                                                    span { class: "text-[9px]", "▶" }
                                                    span { class: "font-sans", "Application Data (1448 bytes)" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
