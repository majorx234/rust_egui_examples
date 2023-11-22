use crate::status_button::button;
use eframe::egui;
pub struct StatusButtonTestApp {
    pub status: bool,
}

impl Default for StatusButtonTestApp {
    fn default() -> StatusButtonTestApp {
        StatusButtonTestApp { status: false }
    }
}

impl eframe::App for StatusButtonTestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("StatusButtonTestApp");
            ui.add(button(&mut self.status));
        });
    }
}
