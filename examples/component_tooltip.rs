#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .center()
        .spacing(10.)
        .child(
            TooltipContainer::new(Tooltip::new("Top!"))
                .position(AttachedPosition::Top)
                .child(Button::new().child("Top")),
        )
        .child(
            TooltipContainer::new(Tooltip::new("Bottom!"))
                .position(AttachedPosition::Bottom)
                .child(Button::new().child("Bottom")),
        )
        .child(
            TooltipContainer::new(Tooltip::new("Left!"))
                .position(AttachedPosition::Left)
                .child(Button::new().child("Left")),
        )
        .child(
            TooltipContainer::new(Tooltip::new("Right!"))
                .position(AttachedPosition::Right)
                .child(Button::new().child("Right")),
        )
}
