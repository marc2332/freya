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
            border: "3 solid rgb(242, 76, 61)",
            rect {
                width: "80",
                height: "80",
                corner_radius: "20",
                background: "rgb(0, 0, 0)",
                border: "1 solid rgb(242, 151, 39)",
                border_align: "inner",
            }
            rect {
                width: "80",
                height: "80",
                corner_radius: "2",
                background: "rgb(0, 0, 0)",
                border: "8 solid green",
                border_align: "outer",
                margin: "8"
            }
            rect {
                width: "80",
                height: "80",
                corner_radius: "2",
                background: "rgb(0, 0, 0)",
                border: "8 solid rgb(34, 166, 153)",
                border_align: "center",
                margin: "4"
            }
        }
    )
}
