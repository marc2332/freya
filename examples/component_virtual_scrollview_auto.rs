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
            .width(Size::fill())
            .height(Size::px(100.))
            .padding(6.)
            .child(
                rect()
                    .expanded()
                    .corner_radius(8.)
                    .background((0, 119, 182))
                    .main_align(Alignment::center())
                    .cross_align(Alignment::center())
                    .child(label().color(Color::WHITE).text(format!("Item {i}"))),
            )
            .into()
    })
    .length(30usize)
    .item_size(100.)
    .height(Size::fill())
    .max_height(Size::window_percent(50.))
}
