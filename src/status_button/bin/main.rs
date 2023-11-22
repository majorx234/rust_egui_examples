use eframe::egui;
pub mod status_button_test_app;
use crate::status_button_test_app::StatusButtonTestApp;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "drag_and_drop_assign",
        options,
        Box::new(|_cc| Box::new(StatusButtonTestApp::default())),
    );
}
