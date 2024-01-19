#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let values = use_state(cx, || vec!["Hello World"].repeat(400));

    render!(VirtualScrollView {
        length: values.get().len(),
        item_size: 25.0,
        builder_values: values.get(),
        direction: "vertical",
        builder: Box::new(move |(key, index, _, values)| {
            let values = values.unwrap();
            let value = values[index];
            let background = if index % 2 == 0 {
                "rgb(200, 200, 200)"
            } else {
                "white"
            };
            rsx! {
                rect {
                    key: "{key}",
                    background: "{background}",
                    label {
                        height: "25",
                        width: "100%",
                        "{index} {value}"
                    }
                }
            }
        })
    })
}
