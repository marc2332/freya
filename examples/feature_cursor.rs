#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .center()
        .spacing(10.)
        .child(
            rect()
                .padding((8., 16.))
                .corner_radius(8.)
                .background((233, 233, 233))
                .cursor(CursorIcon::Pointer)
                .child("Hover me!"),
        )
        .child(
            rect()
                .padding((8., 16.))
                .corner_radius(8.)
                .background((233, 233, 233))
                .cursor(CursorIcon::Grab)
                .child("Or me!"),
        )
}
