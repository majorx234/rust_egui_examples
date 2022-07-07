use eframe::egui;
use eframe::egui::plot::{Line, Plot, Value, Values};

use cli_lib::read_data;

struct PlotApp {
    size: usize,
    wave_data: std::vec::Vec<f32>,
}

impl Default for PlotApp {
    fn default() -> Self {
        Self {
            size: 0,
            wave_data: Vec::new(),
        }
    }
}

impl eframe::App for PlotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let wave = (0..self.size).map(|i| {
            let x = i as f64;
            Value::new(x, self.wave_data[i] as f64)
        });

        let wave_line = Line::new(Values::from_values_iter(wave));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Plot");
            ui.horizontal(|ui| {
                Plot::new("my_wave")
                    .view_aspect(2.0)
                    .show(ui, |plot_ui| plot_ui.line(wave_line));
            })
        });
    }
}

fn plot_wave(wave_data: std::vec::Vec<f32>, size: usize) {
    println!("{}", size);
}

fn main() {
    let (values_size, values_data) = read_data();
    let plot_app = PlotApp {
        wave_data: values_data,
        size: values_size,
    };
    let options = eframe::NativeOptions::default();
    eframe::run_native("Plot App", options, Box::new(|_cc| Box::new(plot_app)));
}
