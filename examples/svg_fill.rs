#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

static SETTINGS: &[u8] = include_bytes!("./settings.svg");

fn app() -> Element {
    let svg_data = static_bytes(SETTINGS);

    rsx!(svg {
        fill: "red",
        width: "100%",
        height: "100%",
        svg_data,
    })
}
