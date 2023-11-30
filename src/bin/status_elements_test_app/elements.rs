use eframe::egui;
use epaint::Stroke;

fn status_indicator_ui(ui: &mut egui::Ui, status: &bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    //TODO implement painter
    response
}

fn value_indicator_ui(ui: &mut egui::Ui, value: u32) -> egui::Response {
    let value: u32 = value.min(127);
    let mut fill_level: f32 = value as f32 / 127.0;
    fill_level = fill_level.clamp(0.0, 1.0);
    let width = 8.0;
    let height = 1.0;
    let desired_size = ui.spacing().interact_size.y * egui::vec2(width, height);

    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
    //TODO implement painter
    if ui.is_rect_visible(response.rect) {
        let visuals = ui.style().visuals.clone();
        let rounding = rect.height() / 2.0;
        ui.painter()
            .rect(rect, rounding, visuals.extreme_bg_color, Stroke::NONE);
        let inner_rect = egui::Rect::from_min_size(
            rect.min,
            egui::vec2(
                (rect.width() * fill_level).max(rect.height()),
                rect.height(),
            ),
        );
        let (dark, bright) = (0.7, 1.0);
        let color_factor = bright;
        ui.painter().rect(
            inner_rect,
            rounding,
            egui::Color32::from(egui::Rgba::from(visuals.selection.bg_fill) * color_factor as f32),
            Stroke::NONE,
        );
        let text: egui::WidgetText = format!("{}", (fill_level * 127.0) as usize).into();
        let galley = text.into_galley(ui, Some(false), f32::INFINITY, egui::TextStyle::Button);
        let text_pos = rect.left_center() - egui::Vec2::new(0.0, galley.size().y / 2.0)
            + egui::vec2(ui.spacing().item_spacing.x, 0.0);
        let text_color = visuals
            .override_text_color
            .unwrap_or(visuals.selection.stroke.color);
        galley.paint_with_fallback_color(&ui.painter().with_clip_rect(rect), text_pos, text_color);
    }
    response
}

pub fn status_indicator(status: &bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| status_indicator_ui(ui, status)
}

pub fn value_indicator(value: u32) -> impl egui::Widget {
    move |ui: &mut egui::Ui| value_indicator_ui(ui, value)
}
