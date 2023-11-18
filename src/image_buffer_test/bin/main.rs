use eframe::egui::{lerp, Color32, Rgba, TextureHandle};
use eframe::{egui, App};
use epaint::textures::TextureOptions;
use std::collections::HashMap;
use std::convert::TryInto;

struct MyApp {
    tex_mngr: TextureManager,
    texture_id: Option<(egui::Vec2, egui::TextureId)>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            tex_mngr: Default::default(),
            texture_id: None,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some((size, texture_id)) = self.texture_id {
                ui.heading("This is an image:");
                ui.add(egui::Image::new((texture_id, size)));
            } else {
                let tex_color_start = Color32::from_rgb(64, 128, 255);
                let tex_color_end = Color32::from_rgb(255, 64, 255);
                let g = Gradient::ground_truth_gradient(
                    tex_color_start,
                    tex_color_end,
                    Interpolation::Linear,
                );

                self.texture_id = Some((
                    egui::Vec2::new(400.0, 300.0),
                    self.tex_mngr.get(ui.ctx(), &g).into(),
                ));
            }
        });
    }
}

#[derive(Clone, Copy)]
enum Interpolation {
    Linear,
    Gamma,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Gradient(pub Vec<Color32>);

impl Gradient {
    pub fn one_color(srgba: Color32) -> Self {
        Self(vec![srgba, srgba])
    }

    pub fn endpoints(left: Color32, right: Color32) -> Self {
        Self(vec![left, right])
    }

    pub fn ground_truth_gradient(
        left: Color32,
        right: Color32,
        interpolation: Interpolation,
    ) -> Self {
        match interpolation {
            Interpolation::Linear => Self::ground_truth_linear_gradient(left, right),
            Interpolation::Gamma => Self::ground_truth_gamma_gradient(left, right),
        }
    }

    pub fn ground_truth_linear_gradient(left: Color32, right: Color32) -> Self {
        let left = Rgba::from(left);
        let right = Rgba::from(right);

        let n = 255;
        Self(
            (0..=n)
                .map(|i| {
                    let t = i as f32 / n as f32;
                    Color32::from(lerp(left..=right, t))
                })
                .collect(),
        )
    }

    pub fn ground_truth_gamma_gradient(left: Color32, right: Color32) -> Self {
        let n = 255;
        Self(
            (0..=n)
                .map(|i| {
                    let t = i as f32 / n as f32;
                    lerp_color_gamma(left, right, t)
                })
                .collect(),
        )
    }

    /// Do premultiplied alpha-aware blending of the gradient on top of the fill color
    /// in gamma-space.
    pub fn with_bg_fill(self, bg: Color32) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|fg| {
                    let a = fg.a() as f32 / 255.0;
                    Color32::from_rgba_premultiplied(
                        (bg[0] as f32 * (1.0 - a) + fg[0] as f32).round() as u8,
                        (bg[1] as f32 * (1.0 - a) + fg[1] as f32).round() as u8,
                        (bg[2] as f32 * (1.0 - a) + fg[2] as f32).round() as u8,
                        (bg[3] as f32 * (1.0 - a) + fg[3] as f32).round() as u8,
                    )
                })
                .collect(),
        )
    }

    pub fn to_pixel_row(&self) -> Vec<Color32> {
        self.0.clone()
    }
}

fn lerp_color_gamma(left: Color32, right: Color32, t: f32) -> Color32 {
    Color32::from_rgba_premultiplied(
        lerp((left[0] as f32)..=(right[0] as f32), t).round() as u8,
        lerp((left[1] as f32)..=(right[1] as f32), t).round() as u8,
        lerp((left[2] as f32)..=(right[2] as f32), t).round() as u8,
        lerp((left[3] as f32)..=(right[3] as f32), t).round() as u8,
    )
}

#[derive(Default)]
struct TextureManager(HashMap<Gradient, TextureHandle>);

impl TextureManager {
    fn get(&mut self, ctx: &egui::Context, gradient: &Gradient) -> &TextureHandle {
        self.0.entry(gradient.clone()).or_insert_with(|| {
            let pixels: Vec<egui::epaint::Color32> = gradient.to_pixel_row().try_into().unwrap();
            let width: usize = pixels.len();
            let height = 1;
            ctx.load_texture(
                "color_test_gradient",
                egui::ColorImage {
                    size: [width, height],
                    pixels,
                },
                TextureOptions::LINEAR,
            )
        })
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
