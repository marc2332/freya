#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut length = use_state(|| 15usize);

    rect()
        .child(
            rect()
                .horizontal()
                .cross_align(Alignment::Center)
                .spacing(6.)
                .padding(6.)
                .child(
                    Button::new()
                        .on_press(move |_| *length.write() += 1)
                        .child("Increase"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| {
                            let mut length = length.write();
                            *length = length.saturating_sub(1);
                        })
                        .child("Decrease"),
                )
                .child(format!("Length: {}", length.read())),
        )
        .child(
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
            .length(*length.read())
            .item_size(100.)
            .height(Size::fill())
            .min_height(Size::px(200.))
            .max_height(Size::window_percent(50.)),
        )
}
