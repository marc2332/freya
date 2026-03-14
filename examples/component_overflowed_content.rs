#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(500., 450.)))
}

fn app() -> impl IntoElement {
    rect()
        .padding(20.)
        .spacing(10.)
        .child(
            Button::new().child(
                OverflowedContent::new()
                    .width(Size::px(100.))
                    .child(label().text("Right to Left (default)").max_lines(1)),
            ),
        )
        .child(
            Button::new().child(
                OverflowedContent::new()
                    .width(Size::px(100.))
                    .left_to_right()
                    .child(label().text("Left to Right direction").max_lines(1)),
            ),
        )
}
