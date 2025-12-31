#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::{
    material_design::{
        ButtonRippleExt,
        Ripple,
    },
    prelude::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_title("Ripple Effect")))
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .center()
        .spacing(12.)
        .child(Button::new().ripple().child("Click me!"))
        .child(
            Ripple::new().child(
                rect()
                    .width(Size::px(100.))
                    .height(Size::px(70.))
                    .center()
                    .background((70, 40, 120))
                    .color(Color::WHITE)
                    .child("Click me!"),
            ),
        )
}
