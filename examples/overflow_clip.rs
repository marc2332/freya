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
        .width(Size::px(300.))
        .height(Size::px(300.))
        .overflow(Overflow::Clip)
        .child(
            rect()
                .width(Size::px(600.))
                .height(Size::px(600.))
                .background((0, 119, 182)),
        )
}
