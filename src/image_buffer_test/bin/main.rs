use eframe::{egui, App};
#[derive(Default)]
struct MyApp {
    texture: Option<(egui::Vec2, egui::TextureId)>,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some((size, texture)) = self.texture {
                ui.heading("This is an image:");
                ui.add(egui::Image::new(texture, size));
            }
        });
    }
}

fn main() {
    let my_app = MyApp::default();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Image buffer Test App",
        options,
        Box::new(|_cc| Box::new(my_app)),
    );
}
