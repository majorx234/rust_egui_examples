use eframe::egui;

struct CounterApp {
    app_name: String,
}


impl Default for CounterApp {
    fn default() -> Self {
        Self {
            app_name: "Counter App".to_owned(),
        }
    }
}

impl eframe::App for CounterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Counter");
        });
    }
}



fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Counter App",
        options,
        Box::new(|_cc| Box::new(CounterApp::default())),
    );
}
