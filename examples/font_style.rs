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
        container {
            width: "100%",
            height: "100%",
            background: "rgb(30, 30, 30)",
            color: "rgb(240, 240, 240)",
            direction: "horizontal",
            paragraph {
                width: "20%",
                line_height: "2",

                text {
                    font_size: "20",
                    font_weight: "thin",
                    "Invisible\n"
                }
                text {
                    font_size: "20",
                    font_weight: "thin",
                    "Thin\n"
                }
                text {
                    font_size: "20",
                    font_weight: "extra-light",
                    "Extra Light\n"
                }
                text {
                    font_size: "20",
                    font_weight: "light",
                    "Light\n"
                }
                text {
                    font_size: "20",
                    font_weight: "normal",
                    "Normal\n"
                }
                text {
                    font_size: "20",
                    font_weight: "medium",
                    "Medium\n"
                }
                text {
                    font_size: "20",
                    font_weight: "semi-bold",
                    "Semi Bold\n"
                }
                text {
                    font_size: "20",
                    font_weight: "extra-bold",
                    "Extra Bold\n"
                }
                text {
                    font_size: "20",
                    font_weight: "black",
                    "Black\n"
                }
                text {
                    font_size: "20",
                    font_weight: "extra-black",
                    "Extra Black\n"
                }
            }
            paragraph {
                width: "20%",
                line_height: "2",

                text {
                    font_size: "20",
                    font_slant: "normal",
                    "Normal\n"
                }
                text {
                    font_size: "20",
                    font_slant: "italic",
                    "Italic\n"
                }
                text {
                    font_size: "20",
                    font_slant: "oblique",
                    "Oblique\n"
                }
            }
            paragraph {
                width: "40%",
                line_height: "2",

                text {
                    font_size: "20",
                    font_family: "Arial",
                    font_width: "extra-condensed",
                    "Extra Condensed\n"
                }
                text {
                    font_size: "20",
                    font_family: "Arial",
                    font_width: "condensed",
                    "Condensed\n"
                }
                text {
                    font_size: "20",
                    font_family: "Arial",
                    font_width: "semi-condensed",
                    "Semi Condensed\n"
                }
                text {
                    font_size: "20",
                    font_family: "Arial",
                    font_width: "normal",
                    "Normal\n"
                }
                text {
                    font_size: "20",
                    font_family: "Arial",
                    font_width: "semi-expanded",
                    "Semi Expanded\n"
                }
                text {
                    font_size: "20",
                    font_family: "Arial",
                    font_width: "extra-expanded",
                    "Extra Expanded\n"
                }
                text {
                    font_size: "20",
                    font_family: "Arial",
                    font_width: "ultra-expanded",
                    "Ultra Expanded\n"
                }
            }
        }
    )
}
