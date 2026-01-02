#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

// NOTES: Cross align does not use the new height so centering does not work as expected

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn cards() -> [Element; 5] {
    [
        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .background((255, 50, 50))
            .into(),
        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .background((50, 255, 50))
            .into(),
        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .background((50, 50, 255))
            .into(),
        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .background((200, 50, 200))
            .into(),
        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .background((150, 150, 150))
            .into(),
    ]
}

fn app() -> impl IntoElement {
    rect()
        .center()
        .expanded()
        .content(Content::Flex)
        .child(
            rect()
                .content(Content::Wrap)
                .width(Size::fill())
                .height(Size::flex(1.))
                .horizontal()
                .children(cards()),
        )
        .child(
            rect()
                .content(Content::Wrap)
                .width(Size::fill())
                .height(Size::flex(1.))
                .children(cards()),
        )
}
