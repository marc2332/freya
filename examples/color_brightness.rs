#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut color = use_state(|| Color::from_rgb(128, 128, 128));
    let selected = color();
    let background = if selected.is_dark() {
        Color::WHITE
    } else {
        Color::BLACK
    };

    rect()
        .expanded()
        .center()
        .spacing(12.)
        .background(background)
        .color(selected)
        .child(format!("Selected color: {}", selected.to_hex_string()))
        .child(format!("Red: {}", selected.r()))
        .child(
            Slider::new(move |value: f64| {
                let current = color();
                color.set(Color::from_rgb(
                    (value * 255. / 100.).round() as u8,
                    current.g(),
                    current.b(),
                ));
            })
            .value(selected.r() as f64 * 100. / 255.)
            .size(Size::px(250.)),
        )
        .child(format!("Green: {}", selected.g()))
        .child(
            Slider::new(move |value: f64| {
                let current = color();
                color.set(Color::from_rgb(
                    current.r(),
                    (value * 255. / 100.).round() as u8,
                    current.b(),
                ));
            })
            .value(selected.g() as f64 * 100. / 255.)
            .size(Size::px(250.)),
        )
        .child(format!("Blue: {}", selected.b()))
        .child(
            Slider::new(move |value: f64| {
                let current = color();
                color.set(Color::from_rgb(
                    current.r(),
                    current.g(),
                    (value * 255. / 100.).round() as u8,
                ));
            })
            .value(selected.b() as f64 * 100. / 255.)
            .size(Size::px(250.)),
        )
}
