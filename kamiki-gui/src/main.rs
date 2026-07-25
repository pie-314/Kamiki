mod app;

use anyhow::Result;
use app::KamikiApp;
use eframe::NativeOptions;

fn main() -> Result<()> {
    env_logger::init();

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Kamiki - eBPF Network Observability")
            .with_inner_size([1200.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Kamiki",
        options,
        Box::new(|cc| Ok(Box::new(KamikiApp::new(cc)))),
    )
    .map_err(|e| anyhow::anyhow!("eframe error: {e}"))
}
