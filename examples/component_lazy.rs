#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    ScrollView::new().expanded().children((0..50).map(|index| {
        Lazy::new()
            .key(index)
            .height(Size::px(100.))
            .width(Size::fill())
            .padding(4.)
            .child(
                rect()
                    .expanded()
                    .center()
                    .corner_radius(8.)
                    .color((255, 255, 255))
                    .background((0, 119, 182))
                    .child(format!("Item {index}")),
            )
    }))
}
