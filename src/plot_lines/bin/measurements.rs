use eframe::egui::plot::Value;
use std::collections::VecDeque;

pub struct Measurements {
    pub values_deq: VecDeque<(Vec<(f32, f32)>, f32)>,
    pub length: usize,
}

impl Measurements {
    pub fn new() -> Self {
        Self {
            values_deq: VecDeque::default(),
            length: 0,
        }
    }

    pub fn add(&mut self, new_values: &[f32], in_x: f32) {
        if let Some((_, last_x)) = self.values_deq.back() {
            if in_x < *last_x {
                self.values_deq.clear()
            }
        }

        let new_points = new_values
            .iter()
            .enumerate()
            .map(|(x, y)| (x as f32, *y))
            .collect::<Vec<(f32, f32)>>();

        self.values_deq.push_back((new_points, in_x));
    }
}
