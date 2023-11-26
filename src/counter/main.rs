use eframe::egui;

struct CounterApp {
    counter: u32,
}

impl Default for CounterApp {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

impl eframe::App for CounterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Counter");
            ui.horizontal(|ui| {
                if ui.button("up").clicked() {
                    self.counter += 1;
                }
                if ui.button("down").clicked() {
                    if self.counter > 0 {
                        self.counter -= 1;
                    }
                }
                ui.label(format!("value: {}", self.counter));
            })
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Counter App",
        options,
        Box::new(|_cc| Box::new(CounterApp::default())),
    );
}
