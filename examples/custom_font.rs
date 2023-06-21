#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

static KABLAMO: &[u8] = include_bytes!("./Kablammo-Regular.ttf");

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_width(200.0)
            .with_height(200.0)
            .with_font("kablammo", KABLAMO)
            .build(),
    );
}

fn app(cx: Scope) -> Element {
    render!(
        rect {
            display: "center",
            height: "100%",
            width: "100%",
            label {
                width: "100%",
                font_size: "20",
                font_family: "kablammo",
                align: "center",
                "This font is called Kablamo"
            }
        }
    )
}
