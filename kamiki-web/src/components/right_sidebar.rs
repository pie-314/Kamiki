#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::{HexDump, PacketDetails, ProtocolHierarchy};

pub fn RightSidebar() -> Element {
    rsx! {
        aside { class: "w-72 border-l border-kamiki-border bg-kamiki-panel flex flex-col p-3 shrink-0 overflow-y-auto gap-3 min-w-0 select-none",
            PacketDetails {}
            ProtocolHierarchy {}
            HexDump {}
        }
    }
}
