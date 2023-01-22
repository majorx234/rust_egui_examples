use eframe::egui;
use eframe::egui::plot::{Line, Plot, Value, Values};
use eframe::egui::Context;
use eframe::NativeOptions;

struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("measurements");
            let values = vec![
                Value { x: 0.0, y: 0.0 },
                Value { x: 1.0, y: 10.0 },
                Value { x: 3.0, y: 120.0 },
            ];
            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(Values::from_values(values)))
            })
        });
    }
}

fn main() {
    let app = App {};
    let options = NativeOptions::default();
    eframe::run_native("plotting app", options, Box::new(|_cc| Box::new(app)));
}
