#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_size(500., 450.)
                .with_title("Software Renderer"),
        ),
    )
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .center()
        .child("This app is rendered on the CPU with the software renderer.")
}
