pub mod status_button_test_app;
use crate::status_button_test_app::StatusButtonTestApp;
mod status_button;
use crossbeam_channel::unbounded;
use std::sync::mpsc;
use std::{process::exit, thread, time::Duration};

fn main() {
    let options = eframe::NativeOptions::default();

    let (status_sender, status_receiver): (
        std::sync::mpsc::SyncSender<bool>,
        std::sync::mpsc::Receiver<bool>,
    ) = mpsc::sync_channel(64);

    let sender_thread = std::thread::spawn(move || {
        while true {
            thread::sleep(Duration::from_millis(2000));
            status_sender.try_send(true);
            thread::sleep(Duration::from_millis(500));
            status_sender.try_send(false);
        }
    });

    let status = false;

    let status_button_test_app =
        StatusButtonTestApp::new(status, Some(status_receiver), Some(sender_thread));

    let _ = eframe::run_native(
        "drag_and_drop_assign",
        options,
        Box::new(|_cc| Box::new(status_button_test_app)),
    );
}
