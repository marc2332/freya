#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut length = use_state(|| 30usize);

    rect()
        .expanded()
        .child(format!("{} items", *length.read()))
        .child(
            VirtualScrollView::new(move |i, _| {
                let is_last = i == *length.read() - 1;
                rect()
                    .key(i)
                    .height(Size::px(50.))
                    .padding(4.)
                    .maybe(is_last, |el| el.on_visible(move |_| *length.write() += 10))
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
            .length(*length.read())
            .item_size(50.)
            .expanded(),
        )
}
