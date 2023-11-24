use crate::status_button::button;
use crossbeam_channel::unbounded;
use eframe::egui;

pub struct StatusButtonTestApp {
    pub status: bool,
    pub status_receiver: Option<std::sync::mpsc::Receiver<bool>>,
    pub sender_thread: Option<std::thread::JoinHandle<()>>,
}

impl StatusButtonTestApp {
    pub fn new(
        status: bool,
        status_receiver: Option<std::sync::mpsc::Receiver<bool>>,
        sender_thread: Option<std::thread::JoinHandle<()>>,
    ) -> Self {
        StatusButtonTestApp {
            status,
            status_receiver,
            sender_thread,
        }
    }
}

impl Default for StatusButtonTestApp {
    fn default() -> StatusButtonTestApp {
        StatusButtonTestApp {
            status: false,
            status_receiver: None,
            sender_thread: None,
        }
    }
}

impl eframe::App for StatusButtonTestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("StatusButtonTestApp");
            ui.add(button(&mut self.status));
        });
    }
}
