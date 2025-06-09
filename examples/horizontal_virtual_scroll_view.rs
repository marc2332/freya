#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app)
}

fn app() -> Element {
    rsx!(VirtualScrollView {
        length: 100,
        item_size: 60.0,
        direction: "horizontal",
        builder: move |index, _: &Option<()>| {
            let background = if index % 2 == 0 {
                "rgb(200, 200, 200)"
            } else {
                "white"
            };
            rsx! {
                rect {
                    key: "{index}",
                    width: "60",
                    height: "100%",
                    background,
                    corner_radius: "16",
                    padding: "6",
                    label {
                        "Item {index}"
                    }
                }
            }
        }
    })
}
