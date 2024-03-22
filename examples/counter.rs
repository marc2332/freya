#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Counter", (400.0, 350.0));
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            padding: "30",
            background: "red",
            rect {
                height: "100%",
                width: "100%",
                padding: "30",
                background: "green",
                rect {
                    height: "100%",
                    width: "100%",
                    padding: "30",
                    background: "blue",

                }
            }
        }
    )
}
