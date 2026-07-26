#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::{ChartsRow, PacketTable, TopProcessesTable};

pub fn MainContent() -> Element {
    rsx! {
        section { class: "flex-1 flex flex-col overflow-y-auto p-3 gap-3 bg-kamiki-bg min-w-0 select-none",
            TopProcessesTable {}
            ChartsRow {}
            PacketTable {}
        }
    }
}
