use eframe::{NativeOptions, egui};

mod lib;
use lib::*;

fn main() -> Result<(), eframe::Error> {
    // It's ok to panic if startup fails.
    let config = startup().unwrap();
    let icon_rx = cache_process(&config);

    eframe::run_native(
        "Launchy", // Use this to make it float in hypr (windowrule = match:title ^Launchy$, float on)
        NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_decorations(false)
                .with_transparent(true)
                .with_inner_size([450.0, 180.0]),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(MyApp::new(config, icon_rx)))),
    )
}
