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
    let mut counter = use_state(|| 1);
    let mut future = use_future(move || {
        let counter = counter();
        async move {
            Timer::after(Duration::from_secs(1)).await;
            counter * 2
        }
    });

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .center()
        .child(
            Button::new()
                .on_press(move |_| counter.set(counter() + 1))
                .child(format!("Increase counter: {}", counter())),
        )
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
