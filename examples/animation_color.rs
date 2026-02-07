#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::{
    animation::*,
    prelude::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut animation = use_animation(|_| AnimColor::new((246, 240, 240), (205, 86, 86)).time(400));

    rect()
        .background(&*animation.read())
        .expanded()
        .center()
        .spacing(8.0)
        .child(
            Button::new()
                .on_press(move |_| {
                    animation.start();
                })
                .child("Start"),
        )
        .child(
            Button::new()
                .on_press(move |_| {
                    animation.reverse();
                })
                .child("Reverse"),
        )
}
