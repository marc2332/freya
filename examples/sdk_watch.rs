#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::{
    prelude::*,
    sdk::use_track_watcher,
};
use tokio::sync::watch;

fn main() {
    let (tx, rx) = watch::channel(1);

    std::thread::spawn(move || {
        let mut i = 1;
        loop {
            std::thread::sleep(Duration::from_secs(1));
            i += 1;
            let _ = tx.send(i);
        }
    });

    launch(LaunchConfig::new().with_window(WindowConfig::new(AppComponent::new(App { rx }))))
}

struct App {
    rx: watch::Receiver<i32>,
}

impl Component for App {
    fn render(&self) -> impl IntoElement {
        use_track_watcher(&self.rx);

        rect()
            .expanded()
            .center()
            .child(format!("Latest value is {}", *self.rx.borrow()))
    }
}
