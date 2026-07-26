mod app;
mod components;
mod data;
mod server;


use app::App;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

fn main() {
    let _ = dioxus_logger::init(Level::INFO);
    launch(App);
}
