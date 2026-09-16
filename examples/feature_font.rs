#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

const EMBEDDED_FONT: &str = "Samuel Morse Embedded";
const DYNAMIC_FONT: &str = "Samuel Morse Dynamic";

fn main() {
    launch(
        LaunchConfig::new()
            .with_font(
                EMBEDDED_FONT,
                Bytes::from_static(include_bytes!("./SamuelMorse.otf")),
            )
            .with_window(WindowConfig::new(app)),
    )
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .padding(24.)
        .spacing(24.)
        .child(
            rect()
                .spacing(8.)
                .child(label().font_size(20.).text("Default font"))
                .child("This text uses Freya's default font."),
        )
        .child(
            rect()
                .spacing(8.)
                .child(label().font_size(20.).text("Embedded font"))
                .child(
                    label()
                        .font_family(EMBEDDED_FONT)
                        .font_size(32.)
                        .text("This font was embedded at startup."),
                ),
        )
        .child(
            rect()
                .spacing(8.)
                .child(label().font_size(20.).text("Dynamic font"))
                .child(
                    label()
                        .font_family(DYNAMIC_FONT)
                        .font_size(32.)
                        .text("This font is loaded after startup."),
                )
                .child(
                    Button::new()
                        .on_press(|_| {
                            Platform::get().load_font(
                                DYNAMIC_FONT,
                                include_bytes!("./SamuelMorse.otf").as_slice(),
                            );
                        })
                        .child("Load dynamic font"),
                ),
        )
}
