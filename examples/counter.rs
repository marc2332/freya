#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(
        LaunchConfig::new()
            .with_font(
                "Embedded Noto Color Emoji",
                Bytes::from_static(include_bytes!(
                    "assets/noto-color-emoji/NotoColorEmoji-Regular.ttf"
                )),
            )
            .with_window(WindowConfig::new(app).with_size(1000., 800.)),
    )
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .padding(20.)
        .background((245, 245, 245))
        .color((20, 20, 20))
        .font_family("Embedded Noto Color Emoji")
        .font_size(36.)
        .child("🔍")
}
