#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

static COVER: &[u8] = include_bytes!("./cover.png");

fn app() -> Element {
    let image_data = static_bytes(COVER);

    rsx!(image {
        image_data,
        width: "fill",
        height: "fill",
        aspect_ratio: "max",
        cache_key: ":)"
    })
}
