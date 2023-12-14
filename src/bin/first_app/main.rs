use eframe::egui;

struct MyApp {
    app_name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            app_name: "First App".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Programm name: ");
                ui.text_edit_singleline(&mut self.app_name);
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "first App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
