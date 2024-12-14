#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app)
}

fn app() -> Element {
    let values = use_signal(|| ["Hello, World!"].repeat(128));

    rsx!(VirtualScrollView {
        length: values.read().len(),
        item_size: 5.0,
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
                    width: "200",
                    label {
                        height: "5",
                        width: "200",
                        "{index} {value}"
                    }
                }
            }
        }
    })
}
