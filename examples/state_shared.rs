#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let count = use_provide_context(|| State::create(0i32));

    rect()
        .expanded()
        .center()
        .spacing(8.0)
        .child(format!("Shared count: {}", count.read()))
        .child(
            rect()
                .horizontal()
                .center()
                .spacing(8.0)
                .child(IncrementButton)
                .child(DecrementButton)
                .child(ResetButton),
        )
}

#[derive(PartialEq)]
struct IncrementButton;
impl Component for IncrementButton {
    fn render(&self) -> impl IntoElement {
        let mut count = use_consume::<State<i32>>();

        Button::new()
            .on_press(move |_| *count.write() += 1)
            .child("Increment")
    }
}

#[derive(PartialEq)]
struct DecrementButton;
impl Component for DecrementButton {
    fn render(&self) -> impl IntoElement {
        let mut count = use_consume::<State<i32>>();

        Button::new()
            .on_press(move |_| *count.write() -= 1)
            .child("Decrement")
    }
}

#[derive(PartialEq)]
struct ResetButton;
impl Component for ResetButton {
    fn render(&self) -> impl IntoElement {
        let mut count = use_consume::<State<i32>>();

        Button::new().on_press(move |_| count.set(0)).child("Reset")
    }
}
