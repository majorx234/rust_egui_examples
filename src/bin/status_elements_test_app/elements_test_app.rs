use crate::elements::{status_indicator, value_indicator};
use crossbeam_channel::unbounded;
use eframe::egui;
use std::thread;
use std::time::Instant;

pub struct ElementsTestApp {
    pub status: bool,
    pub value: u32,
    pub status_receiver: Option<crossbeam_channel::Receiver<bool>>,
    pub value_receiver: Option<crossbeam_channel::Receiver<u32>>,
    pub status_sender_thread: Option<std::thread::JoinHandle<()>>,
    pub value_sender_thread: Option<std::thread::JoinHandle<()>>,
    timer: Instant,
    repainter1_thread_handle: Option<std::thread::JoinHandle<()>>,
    repainter2_thread_handle: Option<std::thread::JoinHandle<()>>,
}

impl ElementsTestApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        status: bool,
        value: u32,
        mim_status_receiver: Option<crossbeam_channel::Receiver<bool>>,
        mim_value_receiver: Option<crossbeam_channel::Receiver<u32>>,
        status_sender_thread: Option<std::thread::JoinHandle<()>>,
        value_sender_thread: Option<std::thread::JoinHandle<()>>,
    ) -> Self {
        let (mim_status_sender, status_receiver) = unbounded();
        let (mim_value_sender, value_receiver) = unbounded();

        let ctx1 = cc.egui_ctx.clone();
        let ctx2 = cc.egui_ctx.clone();

        let repainter1_thread_handle =
            thread::spawn(|| repainter::<bool>(ctx1, mim_status_receiver, Some(mim_status_sender)));

        let repainter2_thread_handle =
            thread::spawn(|| repainter::<u32>(ctx2, mim_value_receiver, Some(mim_value_sender)));

        ElementsTestApp {
            status,
            value: 0,
            status_receiver: Some(status_receiver),
            value_receiver: Some(value_receiver),
            status_sender_thread,
            value_sender_thread,
            timer: Instant::now(),
            repainter1_thread_handle: Some(repainter1_thread_handle),
            repainter2_thread_handle: Some(repainter2_thread_handle),
        }
    }
}

impl Default for ElementsTestApp {
    fn default() -> ElementsTestApp {
        ElementsTestApp {
            status: false,
            value: 0,
            status_receiver: None,
            value_receiver: None,
            status_sender_thread: None,
            value_sender_thread: None,
            timer: Instant::now(),
            repainter1_thread_handle: None,
            repainter2_thread_handle: None,
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
        if let Some(ref value_receiver) = self.value_receiver {
            if let Ok(value) = value_receiver.try_recv() {
                self.value = value;
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("StatusElementsTestApp");
            ui.add(status_indicator(&self.status));
            ui.add(value_indicator(self.value));
        });
    }
}

fn repainter<T>(
    ctx: egui::Context,
    receiver: Option<crossbeam_channel::Receiver<T>>,
    sender: Option<crossbeam_channel::Sender<T>>,
) {
    loop {
        if let Some(ref receiver) = receiver {
            if let Ok(trigger_msg) = receiver.recv() {
                if let Some(ref sender) = sender {
                    let _ = sender.send(trigger_msg);
                    ctx.request_repaint();
                }
            }
        }
    }
}
