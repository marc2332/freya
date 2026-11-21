#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect().expanded().center().child(
        rect()
            .width(Size::px(280.))
            .height(Size::px(320.))
            .mask(
                LinearGradient::new()
                    .stop((Color::TRANSPARENT, 0.))
                    .stop((Color::BLACK, 15.))
                    .stop((Color::BLACK, 85.))
                    .stop((Color::TRANSPARENT, 100.)),
            )
            .child(ScrollView::new().children((0..30).map(|index| {
                rect()
                    .key(index)
                    .width(Size::fill())
                    .height(Size::px(48.))
                    .margin(4.)
                    .corner_radius(10.)
                    .center()
                    .background((15, 163, 242))
                    .color(Color::WHITE)
                    .child(format!("Item {}", index + 1))
            }))),
    )
}
