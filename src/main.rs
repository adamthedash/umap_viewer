use crate::app::UMAPViewer;

mod app;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "SVS Viewer",
        native_options,
        Box::new(|cc| Box::new(UMAPViewer::new(cc))),
    )
}