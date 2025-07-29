#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

static SANSITA_SWASHED: &[u8] = include_bytes!("./SansitaSwashed-Regular.ttf");

fn main() {
    launch_cfg(
        LaunchConfig::<()>::new()
            .with_font("Sansita Swashed", SANSITA_SWASHED)
            .with_window(WindowConfig::default().with_app(app).with_size(200.0, 200.)),
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
