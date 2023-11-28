use crate::elements::{status_indicator, value_indicator};
use crossbeam_channel::unbounded;
use eframe::egui;
use std::thread;
use std::time::Instant;

pub struct ElementsTestApp {
    pub status: bool,
    pub value: u32,
    pub status_receiver: Option<crossbeam_channel::Receiver<bool>>,
    pub sender_thread: Option<std::thread::JoinHandle<()>>,
    timer: Instant,
    repainter_thread_handle: Option<std::thread::JoinHandle<()>>,
}

impl ElementsTestApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        status: bool,
        value: u32,
        mim_status_receiver: Option<crossbeam_channel::Receiver<bool>>,
        sender_thread: Option<std::thread::JoinHandle<()>>,
    ) -> Self {
        let (mim_status_sender, status_receiver) = unbounded();
        let ctx = cc.egui_ctx.clone();

        let repainter_thread_handle =
            thread::spawn(|| repainter(ctx, mim_status_receiver, Some(mim_status_sender)));

        ElementsTestApp {
            status,
            value: 0,
            status_receiver: Some(status_receiver),
            sender_thread,
            timer: Instant::now(),
            repainter_thread_handle: Some(repainter_thread_handle),
        }
    }
}

impl Default for ElementsTestApp {
    fn default() -> ElementsTestApp {
        ElementsTestApp {
            status: false,
            value: 0,
            status_receiver: None,
            sender_thread: None,
            timer: Instant::now(),
            repainter_thread_handle: None,
        }
    }
}

impl eframe::App for ElementsTestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(ref status_receiver) = self.status_receiver {
            if let Ok(status) = status_receiver.try_recv() {
                if status != self.status {
                    self.status = status;
                    self.timer = Instant::now();
                }
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("StatusElementsTestApp");
            ui.add(status_indicator(&self.status));
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
                    let _ = status_sender.send(trigger_msg);
                    ctx.request_repaint();
                }
            }
        }
    }
}
