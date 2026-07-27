#![allow(unused_imports)]

mod app;
mod components;
mod data;
mod server;

use app::App;
use dioxus::prelude::*;

fn main() {
    let _ = dioxus_logger::init(dioxus_logger::tracing::Level::INFO);

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Assets are bundled automatically by `dx build` manganis asset pipeline
    }

    launch(App);
}
