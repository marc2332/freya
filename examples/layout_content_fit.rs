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
        .content(Content::Fit)
        .background((125, 125, 125))
        .padding(8.)
        .child(
            rect()
                .background((255, 0, 0))
                .width(Size::fill_minimum())
                .height(Size::px(100.)),
        )
        .child(
            rect()
                .background((0, 0, 255))
                .width(Size::fill_minimum())
                .height(Size::px(100.))
                .child("Hello, World!"),
        )
        .child(
            rect()
                .background((0, 255, 0))
                .width(Size::fill_minimum())
                .height(Size::px(100.)),
        )
}
