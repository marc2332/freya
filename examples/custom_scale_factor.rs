#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_size(500., 450.)
                .with_title("Custom scale factor")
                .with_custom_scale_factor(1.5),
        ),
    )
}

fn app() -> impl IntoElement {
    let custom_scale_factor = *Platform::get().custom_scale_factor.read();

    rect()
        .expanded()
        .center()
        .spacing(8.)
        .child(format!("Custom scale factor: {custom_scale_factor:.2}"))
        .child(
            rect()
                .horizontal()
                .spacing(8.)
                .child(
                    Button::new()
                        .on_press(move |_| {
                            Platform::get().set_custom_scale_factor(custom_scale_factor + 0.25)
                        })
                        .child("Increase"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| {
                            Platform::get().set_custom_scale_factor(custom_scale_factor - 0.25)
                        })
                        .child("Decrease"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| Platform::get().set_custom_scale_factor(1.0))
                        .child("Reset"),
                ),
        )
}
