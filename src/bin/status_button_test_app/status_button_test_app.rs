use crate::status_button::button;
use crossbeam_channel;
use eframe::egui;
use std::time::Instant;

pub struct StatusButtonTestApp {
    pub status: bool,
    pub status_receiver: Option<crossbeam_channel::Receiver<bool>>,
    pub sender_thread: Option<std::thread::JoinHandle<()>>,
    init_repainter: bool,
    timer: Instant,
}

impl StatusButtonTestApp {
    pub fn new(
        status: bool,
        status_receiver: Option<crossbeam_channel::Receiver<bool>>,
        sender_thread: Option<std::thread::JoinHandle<()>>,
    ) -> Self {
        StatusButtonTestApp {
            status,
            status_receiver,
            sender_thread,
            init_repainter: false,
            timer: Instant::now(),
        }
    }
}

impl Default for StatusButtonTestApp {
    fn default() -> StatusButtonTestApp {
        StatusButtonTestApp {
            status: false,
            status_receiver: None,
            sender_thread: None,
            init_repainter: false,
            timer: Instant::now(),
        }
    }
}

impl eframe::App for StatusButtonTestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.init_repainter {
            //thread::spawn(|| repainter(ctx));
            self.init_repainter = true;
        }
        let mut repaint = false;
        if let Some(ref status_receiver) = self.status_receiver {
            if let Ok(status) = status_receiver.try_recv() {
                if status != self.status {
                    self.status = status;
                    repaint = true;
                    self.timer = Instant::now();
                    println!("changed");
                }
            }
        }
        if self.timer.elapsed().as_millis() < 50 {
            repaint = true;
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("StatusButtonTestApp");
            ui.add(button(&mut self.status));
            if repaint {
                ui.ctx().request_repaint();
                println!("repaint");
            }
        });
    }
}

/*
fn repainter(
    ctx: egui::Context,
) {
        loop {
            if let Ok(trigger_note_msg) = rx_note_velocity.recv() {
                if let Some(ref tx_note_velocity) = tx_note_velocity {
                    tx_note_velocity.send(trigger_note_msg);
                }
            }
            ctx.request_repaint();
        }
    }
}*/
