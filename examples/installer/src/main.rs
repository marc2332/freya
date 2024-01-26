#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

const ICON: &[u8] = include_bytes!("../assets/icon.png");

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_title("Freya & cargo-packager")
            .with_width(400.)
            .with_height(300.)
            .with_icon(LaunchConfig::load_icon(ICON))
            .build(),
    )
}

fn app() -> Element {
    rsx!(
        rect {
            width: "100%",
            height: "100%",
            background: "rgb(1, 116, 190)",
            main_align: "center",
            label {
                width: "100%",
                font_size: "35",
                text_align: "center",
                color: "white",
                "freya 🦀 & cargo-packager 📦"
            }
        }
    )
}
