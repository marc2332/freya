#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    ScrollView::new()
        .height(Size::fill())
        .max_height(Size::window_percent(50.))
        .child(rect().spacing(6.).padding(6.).children((0..5).map(|_| {
            rect()
                .width(Size::fill())
                .height(Size::window_percent(10.))
                .background((0, 119, 182))
                .into()
        })))
}
