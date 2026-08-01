#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn AppIcon(name: String, class: Option<String>) -> Element {
    let cls = class.unwrap_or_else(|| "w-4 h-4 inline-block shrink-0 object-contain".to_string());
    let icon_url = format!("/api/icon?name={}", name);

    rsx! {
        img {
            src: "{icon_url}",
            class: "{cls}",
            alt: "{name}",
            loading: "lazy",
        }
    }
}
