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
            overflow: "clip",
            height: "100%",
            width: "100%",
            padding: "60",
            background: "rgb(224, 224, 224)",
            rect {
                direction: "horizontal",
                rect {
                    shadow: "inset 0 0 8 red",
                    height: "80",
                    width: "80",
                    background: "black",
                }
                rect { width: "40" }
                rect {
                    shadow: "24 24 8 0 rgb(0, 0, 0, 128), -24 -24 8 0 rgb(0, 255, 0, 128)",
                    height: "80",
                    width: "80",
                    background: "black",
                }
                rect { width: "40" }
                rect {
                    shadow: "0 0 60 3 red, 0 0 50 3 orange, 0 0 40 3 yellow, 0 0 30 3 green, 0 0 20 3 blue, 0 0 10 3 rgb(255,0,255)",
                    height: "80",
                    width: "80",
                    background: "black",
                }
                rect { width: "40" }
                rect {
                    shadow: "5 5 10 rgb(190, 190, 190), -5 -5 10 rgb(255, 255, 255)",
                    height: "80",
                    width: "80",
                    corner_radius: "8",
                    background: "rgb(224, 224, 224)",
                }
            }
            rect { height: "40" }
            label {
                text_shadow: "0 18 12 rgb(0, 0, 0), 0 0 20 orange",
                font_size: "80",
                font_weight: "bold",
                "Text!"
            }
        }
    )
}
