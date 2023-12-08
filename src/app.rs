pub struct App;

impl Default for App {
    fn default() -> Self {
        Self {
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new() -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        create_appbar(ctx);
        create_content(ctx)
    }
}

fn create_appbar(ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| { // menu bar
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            ui.add_space(16.0);

            egui::widgets::global_dark_light_mode_buttons(ui);
        });
    });
}

fn create_content(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Subnet Address calculator");
    });
}
