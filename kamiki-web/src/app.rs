#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::{HeaderBar, LeftSidebar, MainContent, RightSidebar, StatusBar};

static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: TAILWIND_CSS }
        main { class: "h-screen w-screen bg-kamiki-bg text-kamiki-textPrimary flex flex-col overflow-hidden font-sans antialiased select-none",
            // ① HeaderBar
            HeaderBar {}

            // Central Workspace Assembly (LeftSidebar, MainContent, RightSidebar)
            div { class: "flex-1 flex overflow-hidden min-h-0",
                LeftSidebar {}
                MainContent {}
                RightSidebar {}
            }

            // ⑤ StatusBar
            StatusBar {}
        }
    }
}