#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(rect {
        width: "100%",
        height: "100%",
        main_align: "center",
        cross_align: "center",
        rect {
            direction: "horizontal",
            font_size: "56",
            spacing: "16",
            label {
                "Loading"
            }
            Loader {
                size: "64"
            }
        }
    })
}
