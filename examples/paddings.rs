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
            height: "33%",
            width: "100%",
            padding: "15",
            background: "black",
            rect {
                height: "100%",
                width: "100%",
                background: "yellow"
            }
        }
        container {
            height: "33%",
            width: "100%",
            padding: "10 30 50 70",
            background: "gray",
            rect {
                height: "100%",
                width: "100%",
                background: "yellow"
            }
        }
        container {
            height: "33%",
            width: "100%",
            padding: "25 125",
            background: "white",
            rect {
                height: "100%",
                width: "100%",
                background: "yellow"
            }
        }
    )
}
