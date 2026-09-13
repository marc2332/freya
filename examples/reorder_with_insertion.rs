#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(400., 400.)))
}

#[derive(PartialEq)]
struct Badge;

impl Component for Badge {
    fn render(&self) -> impl IntoElement {
        label().text("badge").color((15, 163, 242))
    }
}

fn app() -> impl IntoElement {
    let mut reordered = use_state(|| false);

    let items: &[u8] = if reordered() {
        &[2, 1, 5, 0]
    } else {
        &[0, 5, 2]
    };

    rect()
        .expanded()
        .center()
        .spacing(8.)
        .child(
            Button::new()
                .on_press(move |_| {
                    let next = !reordered();
                    reordered.set(next);
                })
                .child("Reorder"),
        )
        .children(items.iter().map(|item| {
            rect()
                .key(item)
                .horizontal()
                .spacing(8.)
                .padding(8.)
                .background((230, 230, 230))
                .child(format!("Item {item}"))
                .maybe_child((reordered() && *item == 5).then_some(Badge))
        }))
}
