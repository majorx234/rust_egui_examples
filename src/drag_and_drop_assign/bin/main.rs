use eframe;
pub mod drag_and_drop_assign_app;
use crate::drag_and_drop_assign_app::DragAndDropAssignApp;

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "drag_and_drop_assign",
        options,
        Box::new(|_cc| Box::new(DragAndDropAssignApp::default())),
    );
}
