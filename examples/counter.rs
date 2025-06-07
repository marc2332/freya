#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Counter", (400.0, 350.0));
}

fn app() -> Element {
    rsx!(

        rect {
            height: "100%",
            width: "100%",
            background: "red",
            onclick: |_| println!("clicked 1"),
            rect {
                height: "100",
                width: "100",
                background: "rgb(0, 119, 182)",
                onpointerdown: |e| {
                    e.prevent_default();
                    println!("clicked 2");
                },
            }
            Button {
                Button {
                    label {
                        "hi"
                    }
                }
            }
        }
    )
}
