#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let platform = Platform::get();

    rect()
        .expanded()
        .center()
        .spacing(8.)
        .child(format!("{:?}", platform.focused_accessibility_id.read()))
        .child(Button::new().child("Button 1"))
        .child(Button::new().child("Button 2"))
}
