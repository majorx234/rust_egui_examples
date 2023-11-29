pub mod elements_test_app;
use crate::elements_test_app::ElementsTestApp;
mod elements;
use crossbeam_channel::unbounded;
use std::{process::exit, thread, time::Duration};

fn main() {
    let options = eframe::NativeOptions::default();

    let (status_sender, status_receiver): (
        crossbeam_channel::Sender<bool>,
        crossbeam_channel::Receiver<bool>,
    ) = unbounded();

    let (value_sender, value_receiver): (
        crossbeam_channel::Sender<u32>,
        crossbeam_channel::Receiver<u32>,
    ) = unbounded();

    let status_sender_thread = std::thread::spawn(move || {
        while true {
            thread::sleep(Duration::from_millis(2000));
            let _ = status_sender.try_send(true);
            thread::sleep(Duration::from_millis(500));
            let _ = status_sender.try_send(false);
        }
    });

    let value_sender_thread = std::thread::spawn(move || {
        let mut value: u32 = 0;
        while true {
            value = (value + 1) % 128;
            let _ = value_sender.try_send(value);
            thread::sleep(Duration::from_millis(100));
        }
    });

    let _ = eframe::run_native(
        "status_elements_test_app",
        options,
        Box::new(|cc| {
            Box::new(ElementsTestApp::new(
                cc,
                false,
                0,
                Some(status_receiver),
                Some(value_receiver),
                Some(status_sender_thread),
                Some(value_sender_thread),
            ))
        }),
    );
}
