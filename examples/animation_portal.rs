use std::time::Duration;

use freya::{
    animation::Function,
    prelude::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn card(i: i32) -> impl IntoElement {
    rect()
        .expanded()
        .background((103, 80, 164))
        .corner_radius(16.)
        .center()
        .color(Color::WHITE)
        .child(format!("Number {}", i))
}

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit,
sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
 Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
 Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

fn popup(i: i32, mut show_popup: State<Option<i32>>) -> impl IntoElement {
    Popup::new()
        .on_close_request(move |_| show_popup.set(None))
        .child(PopupTitle::new("Title".to_string()))
        .child(
            PopupContent::new().child(
                rect()
                    .horizontal()
                    .spacing(8.)
                    .child(
                        Portal::new(i)
                            .width(Size::px(200.))
                            .height(Size::px(200.))
                            .function(Function::Expo)
                            .duration(Duration::from_millis(500))
                            .child(card(i)),
                    )
                    .child(LOREM_IPSUM),
            ),
        )
        .child(
            PopupButtons::new()
                .child(
                    Button::new()
                        .expanded()
                        .rounded()
                        .on_press(move |_| show_popup.set(None))
                        .child("Cancel"),
                )
                .child(
                    Button::new()
                        .filled()
                        .expanded()
                        .rounded()
                        .on_press(move |_| show_popup.set(None))
                        .child("Accept"),
                ),
        )
}

fn app() -> impl IntoElement {
    let mut show_popup = use_state::<Option<i32>>(|| None);

    rect()
        .spacing(8.)
        .center()
        .expanded()
        .horizontal()
        .maybe_child(show_popup().map(|i| popup(i, show_popup)))
        .children_iter((0..5).map(|i| {
            rect()
                .key(i)
                .spacing(6.)
                .child(
                    Portal::new(i)
                        .key(show_popup())
                        .show(show_popup() != Some(i)) // Hide the card if the element is open in the popup
                        .width(Size::px(100.))
                        .height(Size::px(100.))
                        .function(Function::Expo)
                        .duration(Duration::from_millis(500))
                        .child(card(i)),
                )
                .child(
                    Button::new()
                        .child("Open")
                        .rounded()
                        .on_press(move |_| show_popup.set(Some(i))),
                )
                .into()
        }))
}
