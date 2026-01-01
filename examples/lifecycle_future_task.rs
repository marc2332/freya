#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::time::Duration;

use async_io::Timer;
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(500., 450.)))
}

fn app() -> impl IntoElement {
    let mut future = use_future(|| async {
        Timer::after(Duration::from_secs(1)).await;
        123
    });

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .center()
        .child(
            Button::new()
                .on_press(move |_| future.start())
                .child("Restart"),
        )
        .child(match *future.state() {
            FutureState::Fulfilled(d) => d.to_string(),
            FutureState::Pending => "Pending".to_string(),
            FutureState::Loading => "Loading".to_string(),
        })
}
