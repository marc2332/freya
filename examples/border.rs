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
            padding: "14",
            border: "1 solid red",
            rect {
                width: "80",
                height: "80",
                radius: "20",
                background: "rgb(0, 0, 0)",
                border: "1 solid red inner"
            }
            rect {
                width: "80",
                height: "80",
                radius: "2",
                background: "rgb(0, 0, 0)",
                border: "8 solid green outer"
            } 
            rect {
                width: "80",
                height: "80",
                radius: "2",
                background: "rgb(0, 0, 0)",
                border: "8 solid blue center"
            }
        }
    )
}
