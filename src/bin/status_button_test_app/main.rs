pub mod status_button_test_app;
use crate::status_button_test_app::StatusButtonTestApp;
mod status_button;
use crossbeam_channel::unbounded;
use std::{process::exit, thread, time::Duration};

fn main() {
    let options = eframe::NativeOptions::default();

    let (status_sender, status_receiver): (
        crossbeam_channel::Sender<bool>,
        crossbeam_channel::Receiver<bool>,
    ) = unbounded();

    let sender_thread = std::thread::spawn(move || {
        while true {
            thread::sleep(Duration::from_millis(2000));
            let _ = status_sender.try_send(true);
            thread::sleep(Duration::from_millis(500));
            let _ = status_sender.try_send(false);
        }
        exit(0);
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
