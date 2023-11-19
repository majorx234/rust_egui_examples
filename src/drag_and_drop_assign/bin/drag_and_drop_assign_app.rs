use eframe::egui;

pub struct DragAndDropAssignApp {
    base_elements: Vec<String>,
    assigned_elements_to_base: Vec<Vec<String>>,
    not_assigned_elements: Vec<String>,
}

impl Default for DragAndDropAssignApp {
    fn default() -> Self {
        Self {
            base_elements: Vec::new(),
            assigned_elements_to_base: Vec::new(),
            not_assigned_elements: Vec::new(),
        }
    }
}

impl eframe::App for DragAndDropAssignApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("DragAndDropAssignApp");
            ui.columns(3, |columns| {
                columns[0].label("base_elements");
                columns[1].label("assigned_elements");
                columns[2].label("not_assigned_elements");
            })
        });
    }
}
