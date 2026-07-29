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
        .width(Size::fill())
        .height(Size::fill())
        .child(
            VirtualScrollView::new(|item, _| {
                rect()
                    .key(item.index)
                    .height(Size::px(item.size))
                    .padding(4.)
                    .child(
                        rect()
                            .width(Size::fill())
                            .height(Size::fill())
                            .padding(4.)
                            .corner_radius(8.)
                            .color((255, 255, 255))
                            .background((0, 119, 182))
                            .child(format!("Item {}", item.index)),
                    )
                    .into()
            })
            .length(300usize)
            .item_size(50.)
            .height(Size::percent(50.)),
        )
        .child(
            VirtualScrollView::new(|item, _| {
                rect()
                    .key(item.index)
                    .width(Size::px(item.size))
                    .padding(4.)
                    .child(
                        rect()
                            .width(Size::fill())
                            .height(Size::fill())
                            .center()
                            .padding(4.)
                            .corner_radius(8.)
                            .color((255, 255, 255))
                            .background((202, 103, 2))
                            .child(format!("Item {}", item.index)),
                    )
                    .into()
            })
            .direction(Direction::horizontal())
            .length(300usize)
            .item_size(|index: usize| if index.is_multiple_of(2) { 140. } else { 80. })
            .height(Size::percent(50.)),
        )
}
