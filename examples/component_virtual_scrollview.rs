#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    VirtualScrollView::new(|i, _| {
        rect()
            .key(i)
            .height(Size::px(50.))
            .padding(4.)
            .child(
                rect()
                    .width(Size::fill())
                    .height(Size::fill())
                    .padding(4.)
                    .corner_radius(8.)
                    .color((255, 255, 255))
                    .background((0, 119, 182))
                    .child(format!("Item {i}")),
            )
            .into()
    })
    .length(300)
    .item_size(50.)
    .height(Size::percent(100.))
}
