#![allow(non_snake_case)]

use crate::data::state::AppState;
use dioxus::prelude::*;

pub fn HexDump() -> Element {
    let state = use_context::<AppState>();
    let selected_pkt = state.selected_packet();

    rsx! {
        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg overflow-hidden flex flex-col shadow-sm select-none text-xs",
            div { class: "px-3 py-2 border-b border-kamiki-border/80 bg-kamiki-panel/50 flex items-center justify-between",
                span { class: "font-semibold text-kamiki-textPrimary tracking-tight", "Hex Dump" }
                span { class: "text-[10px] text-kamiki-textSecondary font-mono", "(Reconstructed Header)" }
            }

            if let Some(pkt) = selected_pkt {
                {
                    // Reconstruct synthetic 32-byte header representation from packet metadata
                    let proto_num = match pkt.protocol.as_str() {
                        "TCP" => 6u8,
                        "UDP" => 17u8,
                        "ICMP" => 1u8,
                        _ => 0u8,
                    };

                    let mut bytes = vec![
                        0x45,
                        0x00,
                        ((pkt.pkt_len >> 8) & 0xFF) as u8,
                        (pkt.pkt_len & 0xFF) as u8,
                        0x40,
                        0x00,
                        0x40, // TTL 64
                        proto_num,
                    ];

                    // Ports
                    bytes.push(((pkt.src_port >> 8) & 0xFF) as u8);
                    bytes.push((pkt.src_port & 0xFF) as u8);
                    bytes.push(((pkt.dst_port >> 8) & 0xFF) as u8);
                    bytes.push((pkt.dst_port & 0xFF) as u8);

                    // Padding bytes to 32 bytes
                    while bytes.len() < 32 {
                        bytes.push(0x00);
                    }

                    let line1_hex = bytes[0..16].iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ");
                    let line2_hex = bytes[16..32].iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ");

                    let line1_ascii = bytes[0..16].iter().map(|b| if b.is_ascii_graphic() { *b as char } else { '.' }).collect::<String>();
                    let line2_ascii = bytes[16..32].iter().map(|b| if b.is_ascii_graphic() { *b as char } else { '.' }).collect::<String>();

                    rsx! {
                        div { class: "p-2.5 overflow-x-auto font-mono text-[10px] leading-relaxed text-kamiki-textSecondary bg-kamiki-bg/50",
                            div { class: "flex items-center gap-3 hover:text-kamiki-textPrimary transition-colors py-0.5",
                                span { class: "text-kamiki-blue/80 w-8 shrink-0 select-all", "0000" }
                                span { class: "text-kamiki-textPrimary tracking-wider shrink-0 select-all font-medium", "{line1_hex}" }
                                span { class: "text-kamiki-textSecondary/80 border-l border-kamiki-border/60 pl-3 shrink-0 select-all font-mono", "{line1_ascii}" }
                            }
                            div { class: "flex items-center gap-3 hover:text-kamiki-textPrimary transition-colors py-0.5",
                                span { class: "text-kamiki-blue/80 w-8 shrink-0 select-all", "0010" }
                                span { class: "text-kamiki-textPrimary tracking-wider shrink-0 select-all font-medium", "{line2_hex}" }
                                span { class: "text-kamiki-textSecondary/80 border-l border-kamiki-border/60 pl-3 shrink-0 select-all font-mono", "{line2_ascii}" }
                            }
                        }
                    }
                }
            } else {
                div { class: "p-4 text-center text-kamiki-textSecondary font-sans text-xs",
                    "Select a packet to view hex dump"
                }
            }
        }
    }
}
