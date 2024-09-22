#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

static SANSITA_SWASHED: &[u8] = include_bytes!("./SansitaSwashed-Regular.ttf");

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::new()
            .with_size(200.0, 200.)
            .with_font("Sansita Swashed", SANSITA_SWASHED),
    );
}

fn app() -> Element {
    rsx!(
        rect {
            main_align: "center",
            cross_align: "center",
            height: "100%",
            width: "100%",
            label {
                width: "100%",
                font_size: "20",
                font_family: "Sansita Swashed",
                text_align: "center",
                "This font is called Sansita Swashed"
            }
        }
    )
}
