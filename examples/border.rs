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
            width: "100%",
            height: "100%",
            background: "rgb(30, 30, 30)",
            color: "rgb(240, 240, 240)",
            direction: "horizontal",
            padding: "14",
            border: "3 inner rgb(242, 76, 61)",
            rect {
                width: "80",
                height: "80",
                corner_radius: "20",
                background: "rgb(0, 0, 0)",
                border: "1 inner rgb(242, 151, 39)",
            }
            rect {
                width: "80",
                height: "80",
                corner_radius: "2",
                background: "rgb(0, 0, 0)",
                border: "2 outer green",
                margin: "8"
            }
            rect {
                width: "80",
                height: "80",
                corner_radius: "2",
                background: "rgb(0, 0, 0)",
                border: "8 center rgb(34, 166, 153)",
                margin: "8"
            }
        }
    )
}
