#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut color = use_state(|| Color::from_rgb(205, 86, 86));

    rect()
        .padding(24.)
        .spacing(12.)
        .child(ColorPicker::new(move |c| color.set(c)).value(color()))
        .child(format!("{:?}", color.read().to_rgb()))
}
