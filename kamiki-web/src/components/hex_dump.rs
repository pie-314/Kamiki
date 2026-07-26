#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::data::dummy::get_hex_dump;

pub fn HexDump() -> Element {
    let hex_lines = use_signal(get_hex_dump);

    rsx! {
        div { class: "bg-kamiki-panel border border-kamiki-border rounded-lg overflow-hidden flex flex-col shadow-sm select-none text-xs",
            // Header Bar
            div { class: "px-3 py-2 border-b border-kamiki-border/80 bg-kamiki-panel/50 flex items-center justify-between",
                span { class: "font-semibold text-kamiki-textPrimary tracking-tight", "Hex Dump" }
            }

            // Monospace Hex Dump Grid
            div { class: "p-2.5 overflow-x-auto font-mono text-[10px] leading-relaxed text-kamiki-textSecondary bg-kamiki-bg/50",
                for line in hex_lines.read().iter() {
                    div { key: "{line.offset}", class: "flex items-center gap-3 hover:text-kamiki-textPrimary transition-colors py-0.5",
                        // Offset (Muted Blue/Gray)
                        span { class: "text-kamiki-blue/80 w-8 shrink-0 select-all", "{line.offset}" }

                        // Hex Bytes
                        span { class: "text-kamiki-textPrimary tracking-wider shrink-0 select-all font-medium", "{line.hex}" }

                        // ASCII Representation
                        span { class: "text-kamiki-textSecondary/80 border-l border-kamiki-border/60 pl-3 shrink-0 select-all font-mono", "{line.ascii}" }
                    }
                }
            }
        }
    }
}
