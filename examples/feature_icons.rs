#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::{
    icons,
    prelude::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .horizontal()
        .main_align(Alignment::SpaceEvenly)
        .cross_align(Alignment::Center)
        .expanded()
        .child(
            svg(icons::lucide::antenna())
                .theme_accent_color()
                .width(Size::px(100.))
                .height(Size::px(100.)),
        )
        .child(
            svg(icons::lucide::shield())
                .color((120, 50, 255))
                .stroke_width(4.0)
                .width(Size::px(100.))
                .height(Size::px(100.)),
        )
}
