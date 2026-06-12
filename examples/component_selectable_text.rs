#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let logo = ("rust-logo", include_bytes!("./rust_logo.png"));

    rect()
        .padding(25.)
        .spacing(10.)
        .font_size(25.)
        .child(label().font_size(35.).text("Select the text from below"))
        .child(
            SelectableText::new()
                .span("You can select")
                .child(
                    ImageViewer::new(logo)
                        .width(Size::px(32.))
                        .height(Size::px(32.)),
                )
                .span("this looooooooooong text")
                .child(Button::new().child("Button"))
                .span("And this long text too"),
        )
}
