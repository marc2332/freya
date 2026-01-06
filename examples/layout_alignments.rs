#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn cards() -> [Element; 3] {
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
    ]
}

fn app() -> impl IntoElement {
    ScrollView::new()
        .spacing(5.)
        .child(
            rect()
                .width(Size::fill())
                .height(Size::px(200.))
                .horizontal()
                .main_align(Alignment::End)
                .children(cards()),
        )
        .child(
            rect()
                .width(Size::fill())
                .height(Size::px(200.))
                .horizontal()
                .main_align(Alignment::Center)
                .children(cards()),
        )
        .child(
            rect()
                .width(Size::fill())
                .height(Size::px(200.))
                .horizontal()
                .main_align(Alignment::End)
                .children(cards()),
        )
        .child(
            rect()
                .width(Size::fill())
                .height(Size::px(200.))
                .horizontal()
                .main_align(Alignment::SpaceAround)
                .children(cards()),
        )
        .child(
            rect()
                .width(Size::fill())
                .height(Size::px(200.))
                .horizontal()
                .main_align(Alignment::SpaceBetween)
                .children(cards()),
        )
        .child(
            rect()
                .width(Size::fill())
                .height(Size::px(200.))
                .horizontal()
                .main_align(Alignment::SpaceEvenly)
                .children(cards()),
        )
}
