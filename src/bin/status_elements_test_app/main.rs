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

    let sender_thread = std::thread::spawn(move || {
        while true {
            thread::sleep(Duration::from_millis(2000));
            let _ = status_sender.try_send(true);
            thread::sleep(Duration::from_millis(500));
            let _ = status_sender.try_send(false);
        }
        exit(0);
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
                Some(sender_thread),
            ))
        }),
    );
}
