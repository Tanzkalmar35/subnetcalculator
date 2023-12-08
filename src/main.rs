#![warn(clippy::all, rust_2018_idioms)]

mod app;

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Subnet Calculator",
        native_options,
        Box::new(|_| Box::new(app::App::new())),
    )
}
