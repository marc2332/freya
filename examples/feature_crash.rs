#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect().center().expanded().child(
        Button::new()
            .on_press(|_| panic!("This is an intentional crash."))
            .child("Crash"),
    )
}
