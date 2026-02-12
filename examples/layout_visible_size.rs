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
        .background((255, 0, 0))
        .child("Hello, World!")
        .overflow(Overflow::Clip)
        .visible_height(VisibleSize::inner_percent(50.))
}
