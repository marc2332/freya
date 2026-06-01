#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::{
    prelude::*,
    sdk::use_track_watcher,
};
use tokio::{
    runtime::Builder,
    sync::watch,
};

fn main() {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    let _rt = rt.enter();

    let (tx, rx) = watch::channel(1);

    launch(
        LaunchConfig::new()
            .with_future(move |_| async move {
                let mut interval = tokio::time::interval(Duration::from_secs(1));
                interval.tick().await;
                let mut i = 1;
                loop {
                    interval.tick().await;
                    i += 1;
                    let _ = tx.send(i);
                }
            })
            .with_window(WindowConfig::new_app(MyApp { rx })),
    )
}

struct MyApp {
    rx: watch::Receiver<i32>,
}

impl App for MyApp {
    fn render(&self) -> impl IntoElement {
        use_track_watcher(&self.rx);

        rect()
            .expanded()
            .center()
            .child(format!("Latest value is {}", *self.rx.borrow()))
    }
}
