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
        .content(Content::Flex)
        .center()
        .width(Size::fill())
        .expanded()
        .horizontal()
        .child(
            rect()
                .width(Size::px(100.))
                .height(Size::px(100.))
                .background((255, 50, 50)),
        )
        .child(
            rect()
                .width(Size::flex(1.))
                .height(Size::px(100.))
                .background((50, 255, 50)),
        )
        .child(
            rect()
                .width(Size::px(100.))
                .height(Size::px(100.))
                .background((50, 50, 255)),
        )
}
