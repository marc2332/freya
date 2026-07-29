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
            TooltipContainer::new(Tooltip::new_text("Top!"))
                .position(AttachedPosition::Top)
                .child(Button::new().child("Top")),
        )
        .child(
            TooltipContainer::new(Tooltip::new_text("Bottom!"))
                .position(AttachedPosition::Bottom)
                .child(Button::new().child("Bottom")),
        )
        .child(
            TooltipContainer::new(Tooltip::new_text("Left!"))
                .position(AttachedPosition::Left)
                .child(Button::new().child("Left")),
        )
        .child(
            TooltipContainer::new(Tooltip::new_text("Right!"))
                .position(AttachedPosition::Right)
                .child(Button::new().child("Right")),
        )
        .child(
            TooltipContainer::new(
                Tooltip::new().child(
                    rect()
                        .horizontal()
                        .cross_align(Alignment::Center)
                        .spacing(6.)
                        .child(
                            rect()
                                .width(Size::px(10.))
                                .height(Size::px(10.))
                                .corner_radius(5.)
                                .background(Color::GREEN),
                        )
                        .child("Connected"),
                ),
            )
            .child(Button::new().child("Custom")),
        )
}
