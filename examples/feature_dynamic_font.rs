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
        .spacing(16.)
        .child(
            label()
                .font_family("Samuel Morse")
                .font_size(48.)
                .text("Hello, World!"),
        )
        .child(
            Button::new()
                .on_press(|_| {
                    Platform::get().load_font(
                        "Samuel Morse",
                        include_bytes!("./SamuelMorse.otf").as_slice(),
                    );
                })
                .child("Load font"),
        )
}
