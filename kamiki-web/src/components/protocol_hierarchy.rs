#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::data::state::AppState;

pub fn ProtocolHierarchy() -> Element {
    let state = use_context::<AppState>();
    let selected_pkt = state.selected_packet();

    let mut eth_open = use_signal(|| true);
    let mut ip_open = use_signal(|| true);
    let mut l4_open = use_signal(|| true);

    rsx! {
        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg overflow-hidden flex flex-col shadow-sm select-none text-xs",
            div { class: "px-3 py-2 border-b border-kamiki-border/80 bg-kamiki-panel/50 flex items-center justify-between",
                span { class: "font-semibold text-kamiki-textPrimary tracking-tight", "Protocol Hierarchy" }
            }

            if let Some(pkt) = selected_pkt {
                div { class: "p-2.5 flex flex-col gap-1 font-mono text-[11px] text-kamiki-textSecondary",
                    // Layer 2: Ethernet II
                    div { class: "flex flex-col gap-1",
                        div {
                            class: "flex items-center gap-1.5 cursor-pointer hover:text-kamiki-textPrimary transition-colors",
                            onclick: move |_| eth_open.toggle(),
                            span { class: "text-[9px] w-3 text-center", if *eth_open.read() { "▼" } else { "▶" } }
                            span { class: "font-sans font-medium text-kamiki-textPrimary", "Ethernet II (14 bytes)" }
                        }

                        if *eth_open.read() {
                            // Layer 3: IPv4
                            div { class: "pl-3 flex flex-col gap-1 border-l border-kamiki-border/40 ml-1.5",
                                div {
                                    class: "flex items-center gap-1.5 cursor-pointer hover:text-kamiki-textPrimary transition-colors",
                                    onclick: move |_| ip_open.toggle(),
                                    span { class: "text-[9px] w-3 text-center", if *ip_open.read() { "▼" } else { "▶" } }
                                    span { class: "font-sans font-medium text-kamiki-textPrimary", "IPv4 ({pkt.src_ip} -> {pkt.dst_ip})" }
                                }

                                if *ip_open.read() {
                                    // Layer 4: Transport (TCP / UDP / ICMP)
                                    div { class: "pl-3 flex flex-col gap-1 border-l border-kamiki-border/40 ml-1.5",
                                        div {
                                            class: "flex items-center gap-1.5 cursor-pointer hover:text-kamiki-textPrimary transition-colors",
                                            onclick: move |_| l4_open.toggle(),
                                            span { class: "text-[9px] w-3 text-center", if *l4_open.read() { "▼" } else { "▶" } }
                                            span { class: "font-sans font-medium text-kamiki-textPrimary", "{pkt.protocol} ({pkt.src_port} -> {pkt.dst_port})" }
                                        }

                                        if *l4_open.read() {
                                            // Layer 7: Payload Mock
                                            if pkt.protocol == "TCP" && (pkt.src_port == 443 || pkt.dst_port == 443) {
                                                div { class: "pl-3 flex flex-col gap-1 border-l border-kamiki-border/40 ml-1.5",
                                                    div { class: "flex items-center gap-1.5 cursor-pointer hover:text-kamiki-textPrimary transition-colors",
                                                        span { class: "text-[9px] w-3 text-center", "▼" }
                                                        span { class: "font-sans font-medium text-kamiki-textPrimary", "Transport Layer Security (TLSv1.3)" }
                                                    }
                                                    div { class: "pl-4 text-kamiki-textSecondary flex items-center gap-1.5 py-0.5",
                                                        span { class: "text-[9px]", "▶" }
                                                        span { class: "font-sans", "Application Data ({pkt.pkt_len} bytes)" }
                                                    }
                                                }
                                            } else {
                                                div { class: "pl-4 text-kamiki-textSecondary hover:text-kamiki-textPrimary flex items-center gap-1.5 cursor-pointer py-0.5",
                                                    span { class: "text-[9px]", "▶" }
                                                    span { class: "font-sans", "Payload Data ({pkt.pkt_len} bytes)" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                div { class: "p-4 text-center text-kamiki-textSecondary font-sans text-xs",
                    "Select a packet to view protocol stack"
                }
            }
        }
    }
}
