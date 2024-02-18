#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

const ICON: &[u8] = include_bytes!("./freya_icon.png");

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_width(400.)
            .with_height(300.)
            .with_icon(LaunchConfig::load_icon(ICON))
            .build(),
    )
}

fn app() -> Element {
    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "rgb(0, 119, 182)",
            color: "white",
            label {
                font_size: "35",
                text_align: "center",
                "This simply shows how to pass an icon to the Window"
            }
        }
    )
}
