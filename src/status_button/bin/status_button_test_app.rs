use eframe::egui;
pub struct StatusButtonTestApp {}

impl Default for StatusButtonTestApp {
    fn default() -> StatusButtonTestApp {
        StatusButtonTestApp {}
    }
}

impl eframe::App for StatusButtonTestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("StatusButtonTestApp");
        });
    }
}
