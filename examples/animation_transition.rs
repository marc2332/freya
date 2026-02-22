#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::{
    animation::*,
    prelude::*,
};
use rand::Rng;

/// Convenience helper used only by this example
fn random_color() -> Color {
    let mut rng = rand::rng();
    Color::from_rgb(rng.random(), rng.random(), rng.random())
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut color = use_state(random_color);
    let animation = use_animation_transition(color, |from, to| AnimColor::new(from, to).time(500));

    rect()
        .background(&*animation.read())
        .expanded()
        .center()
        .spacing(8.0)
        .child(
            Button::new()
                .on_press(move |_| {
                    color.set(random_color());
                })
                .child("Random"),
        )
}
