#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    prelude::*,
    winit::event_loop::EventLoop,
};

fn main() {
    let event_loop = EventLoop::<NativeEvent>::with_user_event()
        .build()
        .expect("Failed to create event loop.");

    launch(
        LaunchConfig::new()
            .with_window(WindowConfig::new(app).with_title("Custom Event Loop"))
            .with_event_loop(event_loop),
    );
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .center()
        .child("Window launched with a custom event loop")
}
