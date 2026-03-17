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
                    .child(label().text("RTL from edge (default)").max_lines(1)),
            ),
        )
        .child(
            Button::new().child(
                OverflowedContent::new()
                    .width(Size::px(100.))
                    .start_visible()
                    .child(label().text("RTL from visible position").max_lines(1)),
            ),
        )
        .child(
            Button::new().child(
                OverflowedContent::new()
                    .width(Size::px(100.))
                    .left_to_right()
                    .child(label().text("LTR from edge position").max_lines(1)),
            ),
        )
        .child(
            Button::new().child(
                OverflowedContent::new()
                    .width(Size::px(100.))
                    .left_to_right()
                    .start_visible()
                    .child(label().text("LTR from visible position").max_lines(1)),
            ),
        )
}
