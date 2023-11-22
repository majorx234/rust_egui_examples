use eframe::egui;

fn button_ui(ui: &mut egui::Ui, status: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *status = !*status;
        response.mark_changed(); // report back that the value changed
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *status, ""));
    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *status);
        let visuals = ui.style().interact_selectable(&response, *status);

        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }
    response
}

pub fn button(status: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| button_ui(ui, status)
}
