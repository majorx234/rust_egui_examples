use eframe::egui::plot::Value;
use std::collections::VecDeque;

pub struct Measurements {
    pub values_deq: VecDeque<Value>,
}

impl Measurements {
    pub fn new() -> Self {
        Self {
            values_deq: VecDeque::default(),
        }
    }
}
