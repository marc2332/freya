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
            direction: "horizontal",
            rect {
                height: "100",
                width: "100",
                main_align: "center",
                cross_align: "center",
                background: "rgb(0, 119, 182)",
                label {
                    blend_mode: "soft-light",
                    color: "red",
                    "Hello, World!"
                }
            }
            rect {
                height: "100",
                width: "100",
                main_align: "center",
                cross_align: "center",
                background: "rgb(112, 241, 129)",
                label {
                    blend_mode: "luminosity",
                    color: "red",
                    "Hello, World!"
                }
            }
            rect {
                height: "100",
                width: "100",
                main_align: "center",
                cross_align: "center",
                background: "rgb(255, 121, 44)",
                label {
                    blend_mode: "exclusion",
                    color: "white",
                    "Hello, World!"
                }
            }
            rect {
                height: "100",
                width: "100",
                main_align: "center",
                cross_align: "center",
                background: "rgb(255, 44, 132)",
                label {
                    blend_mode: "overlay",
                    color: "blue",
                    "Hello, World!"
                }
            }
        }
    )
}
