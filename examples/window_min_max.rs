#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
   
    let platform = use_platform();

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            Button {
                onclick: move |_| platform.toggle_maximize_window(),
                label {
                    "Maximize"
                }
            }
            Button {
                onclick: move |_| platform.toggle_minimize_window(),
                label {
                    "Minimize"
                }
            }
        }
    )
}