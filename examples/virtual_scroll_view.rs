#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app)
}

fn app() -> Element {
    let values = use_signal(|| ["Hello, World!"].repeat(300));

    rsx!(VirtualScrollView {
        length: values.read().len(),
        item_size: 25.0,
        direction: "vertical",
        builder: move |index, _: &Option<()>| {
            let value = values.read()[index];
            let background = if index % 2 == 0 {
                "rgb(200, 200, 200)"
            } else {
                "white"
            };
            rsx! {
                rect {
                    key: "{index}",
                    background: "{background}",
                    width: "100%",
                    label {
                        height: "25",
                        "{index} {value}"
                    }
                }
            }
        }
    })
}
