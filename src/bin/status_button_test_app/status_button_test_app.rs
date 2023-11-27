use crate::status_button::button;
use crossbeam_channel::unbounded;
use eframe::egui;
use std::thread;
use std::time::Instant;

pub struct StatusButtonTestApp {
    pub status: bool,
    pub status_receiver: Option<crossbeam_channel::Receiver<bool>>,
    pub sender_thread: Option<std::thread::JoinHandle<()>>,
    init_repainter: bool,
    timer: Instant,
    repainter_thread_handle: Option<std::thread::JoinHandle<()>>,
}

impl StatusButtonTestApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        status: bool,
        mim_status_receiver: Option<crossbeam_channel::Receiver<bool>>,
        sender_thread: Option<std::thread::JoinHandle<()>>,
    ) -> Self {
        let (mim_status_sender, status_receiver) = unbounded();
        let ctx = cc.egui_ctx.clone();

        let repainter_thread_handle =
            thread::spawn(|| repainter(ctx, mim_status_receiver, Some(mim_status_sender)));

        StatusButtonTestApp {
            status,
            status_receiver: Some(status_receiver),
            sender_thread,
            init_repainter: false,
            timer: Instant::now(),
            repainter_thread_handle: Some(repainter_thread_handle),
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
            repainter_thread_handle: None,
        }
    }
}

impl eframe::App for StatusButtonTestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

fn repainter(
    ctx: egui::Context,
    status_receiver: Option<crossbeam_channel::Receiver<bool>>,
    status_sender: Option<crossbeam_channel::Sender<bool>>,
) {
    loop {
        if let Some(ref status_receiver) = status_receiver {
            if let Ok(trigger_msg) = status_receiver.recv() {
                if let Some(ref status_sender) = status_sender {
                    status_sender.send(trigger_msg);
                    ctx.request_repaint();
                }
            }
        }
    }
}
