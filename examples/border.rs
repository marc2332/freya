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
            main_align: "center",
            cross_align: "center",
            rect {
                width: "80",
                height: "80",
                corner_radius: "2",
                background: "rgb(0, 0, 0)",
                border: "6 outer red, 5 outer orange, 4 outer yellow, 3 outer green, 2 outer blue, 1 outer purple",
                margin: "8"
            }
        }
    )
}
