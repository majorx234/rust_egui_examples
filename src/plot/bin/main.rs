use eframe::egui::plot::{Line, Plot, Value, Values};

use cli_lib::read_data;

fn plot_wave(wave_data: std::vec::Vec<f32>, size: usize) {
    println!("{}", size);
}

fn main() {
    let (size, values_data) = read_data();
    plot_wave(values_data, size);
}
