use eframe::egui::{
    self, vec2, CursorIcon, Id, InnerResponse, LayerId, Order, Rect, Sense, Shape, Ui, Vec2,
};

pub fn drag_source(ui: &mut Ui, id: Id, body: impl FnOnce(&mut Ui)) {
    let is_being_dragged = ui.memory(|mem| mem.is_being_dragged(id));
    if !is_being_dragged {
        let response = ui.scope(body).response;

        // Check for drags:
        let response = ui.interact(response.rect, id, Sense::drag());
        if response.hovered() {
            ui.ctx().set_cursor_icon(CursorIcon::Grab);
        }
    } else {
        ui.ctx().set_cursor_icon(CursorIcon::Grabbing);

        // Paint the body to a new layer:
        let layer_id = LayerId::new(Order::Tooltip, id);
        let response = ui.with_layer_id(layer_id, body).response;

        // Now we move the visuals of the body to where the mouse is.
        // Normally you need to decide a location for a widget first,
        // because otherwise that widget cannot interact with the mouse.
        // However, a dragged component cannot be interacted with anyway
        // (anything with `Order::Tooltip` always gets an empty [`Response`])
        // So this is fine!

        if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
            let delta = pointer_pos - response.rect.center();
            ui.ctx().translate_layer(layer_id, delta);
        }
    }
}

pub fn drop_target<R>(
    ui: &mut Ui,
    can_accept_what_is_being_dragged: bool,
    body: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let is_being_dragged = ui.memory(|mem| mem.is_anything_being_dragged());

    let margin = Vec2::splat(4.0);

    let outer_rect_bounds = ui.available_rect_before_wrap();
    let inner_rect = outer_rect_bounds.shrink2(margin);
    let where_to_put_background = ui.painter().add(Shape::Noop);
    let mut content_ui = ui.child_ui(inner_rect, *ui.layout());
    let ret = body(&mut content_ui);
    let outer_rect = Rect::from_min_max(outer_rect_bounds.min, content_ui.min_rect().max + margin);
    let (rect, response) = ui.allocate_at_least(outer_rect.size(), Sense::hover());

    let style = if is_being_dragged && can_accept_what_is_being_dragged && response.hovered() {
        ui.visuals().widgets.active
    } else {
        ui.visuals().widgets.inactive
    };

    let mut fill = style.bg_fill;
    let mut stroke = style.bg_stroke;
    if is_being_dragged && !can_accept_what_is_being_dragged {
        fill = ui.visuals().gray_out(fill);
        stroke.color = ui.visuals().gray_out(stroke.color);
    }

    ui.painter().set(
        where_to_put_background,
        epaint::RectShape::new(rect, style.rounding, fill, stroke),
    );

    InnerResponse::new(ret, response)
}

pub struct DragAndDropAssignApp {
    base_elements: Vec<String>,
    assigned_elements_to_base: Vec<Vec<String>>,
    not_assigned_elements: Vec<String>,
    selected_elem: usize,
}

impl DragAndDropAssignApp {
    pub fn add_base_element(&mut self, new_element: String) {
        self.base_elements.push(new_element);
        let new_assigned_elements: Vec<String> = Vec::new();
        self.assigned_elements_to_base.push(new_assigned_elements);
    }
    pub fn add_not_assigned_elements(&mut self, new_element: String) {
        self.not_assigned_elements.push(new_element);
    }
}

impl Default for DragAndDropAssignApp {
    fn default() -> Self {
        let dummy_vec: Vec<String> = Vec::new();
        Self {
            base_elements: vec!["dummy".to_string()],
            assigned_elements_to_base: vec![dummy_vec],
            not_assigned_elements: Vec::new(),
            selected_elem: 0,
        }
    }
}

impl eframe::App for DragAndDropAssignApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("DragAndDropAssignApp");
            ui.columns(3, |columns| {
                columns[0].set_min_size(vec2(64.0, 100.0));
                columns[1].set_min_size(vec2(64.0, 100.0));
                columns[2].set_min_size(vec2(64.0, 100.0));
                columns[0].label("base_elements");
                if columns[0].button("add").clicked() {
                    self.add_base_element("test1".to_string());
                }
                columns[1].label("assigned_elements");
                columns[2].label("not_assigned_elements");

                let can_accept_what_is_being_dragged = true;
                let response =
                    drop_target(&mut columns[1], can_accept_what_is_being_dragged, |ui| {
                        let col_idx = 2;
                        for (row_idx, item) in self.assigned_elements_to_base[self.selected_elem]
                            .iter()
                            .enumerate()
                        {
                            let item_id = Id::new("Test Item").with(col_idx).with(row_idx);
                        }
                    });
            })
        });
    }
}
