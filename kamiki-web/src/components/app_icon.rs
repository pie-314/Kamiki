#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn AppIcon(name: String, class: Option<String>) -> Element {
    let cls = class.unwrap_or_else(|| "text-xs inline-block shrink-0".to_string());
    let name_lower = name.to_lowercase();

    if name_lower.contains("firefox") {
        rsx! { i { class: "devicon-firefox-plain colored {cls}" } }
    } else if name_lower.contains("discord") {
        rsx! { i { class: "devicon-discord-plain colored {cls}" } }
    } else if name_lower.contains("spotify") {
        rsx! { i { class: "fa-brands fa-spotify text-[#1DB954] {cls}" } }
    } else if name_lower.contains("ssh") || name_lower.contains("terminal") || name_lower.contains("bash") {
        rsx! { i { class: "fa-solid fa-terminal text-[#58a6ff] {cls}" } }
    } else if name_lower.contains("curl") || name_lower.contains("http") || name_lower.contains("api") {
        rsx! { i { class: "fa-solid fa-globe text-[#bc8cff] {cls}" } }
    } else if name_lower.contains("chrome") {
        rsx! { i { class: "devicon-chrome-plain colored {cls}" } }
    } else {
        rsx! { i { class: "fa-solid fa-gear text-kamiki-textSecondary {cls}" } }
    }
}
