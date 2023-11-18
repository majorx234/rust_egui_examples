use eframe::egui;
use eframe::egui::Context;
use eframe::NativeOptions;
use egui_plot::{Line, Plot};
use measurements::Measurements;

mod measurements;

struct App {
    measurements: Measurements,
}

impl App {
    fn new() -> Self {
        Self {
            measurements: Measurements::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("measurements");
            let lines_vec = vec![[0.0, 0.0], [1.0, 10.0], [3.0, 120.0]];
            plot.show(ui, |plot_ui| plot_ui.line(Line::new(lines_vec)))
        });
    }
}

fn main() {
    let app = App::new();
    let options = NativeOptions::default();
    eframe::run_native("plotting app", options, Box::new(|_cc| Box::new(app)));
}
