#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut value = use_state(|| 0.0f64);

    rect()
        .center()
        .expanded()
        .spacing(4.)
        .child(format!("{}%", value().floor()))
        .child(
            Slider::new(move |e| value.set(e))
                .value(value())
                .size(Size::px(250.)),
        )
        .child(
            Slider::new(move |e| value.set(e))
                .direction(Direction::Vertical)
                .value(value())
                .size(Size::px(250.)),
        )
        .child(
            Slider::new(move |e| value.set(e))
                .enabled(false)
                .value(value())
                .size(Size::px(150.)),
        )
}
