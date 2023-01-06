#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100%",
            height: "100%",
            background: "rgb(30, 30, 30)",
            color: "rgb(240, 240, 240)",
            paragraph {
                line_height: "2",
                width: "100%",
                text {
                    font_style: "normal",
                    font_size: "20",
                    "Normal\n"
                }
                text {
                    font_style: "bold",
                    font_size: "20",
                    "Bold\n"
                }
                text {
                    font_style: "italic",
                    font_size: "20",
                    "Italic\n"
                }
                text {
                    font_style: "bold-italic",
                    font_size: "20",
                    "Italic & Bold"
                }
            }
        }
    )
}
