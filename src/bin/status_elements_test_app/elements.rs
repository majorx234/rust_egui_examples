use eframe::egui;

fn status_indicator_ui(ui: &mut egui::Ui, status: &bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    //TODO implement painter
    response
}

fn value_indicator_ui(ui: &mut egui::Ui, value: &u32) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    //TODO implement painter
    response
}

pub fn status_indicator(status: &bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| status_indicator_ui(ui, status)
}

pub fn value_indicator(value: &u32) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| value_indicator_ui(ui, value)
}
