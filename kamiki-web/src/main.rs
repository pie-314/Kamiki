#![allow(unused_imports)]

mod api_client;
mod app;
mod components;
mod data;

use app::App;
use dioxus::prelude::*;

fn main() {
    let _ = dioxus_logger::init(dioxus_logger::tracing::Level::INFO);
    launch(App);
}
