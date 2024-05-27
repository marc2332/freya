#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {

    rsx!(
        rect {
            width: "100%",
            height: "100%",

            main_align: "center",
            cross_align: "center",
            direction: "horizontal",

            Button {
                onclick: |_| use_platform().maximize_window(),
                label {
                    "Maximize"
                }
            }

            Button {
                onclick: |_| use_platform().minimize_window(),
                label {
                    "Minimize"
                }
            }
        }
    )
}