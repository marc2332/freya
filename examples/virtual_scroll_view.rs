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
        width: "100%",
        height: "100%",
        show_scrollbar: true,
        length: values.get().len(),
        item_size: 25.0,
        builder_values: values.get(),
        direction: "vertical",
        builder: Box::new(move |(key, index, values)| {
            let values = values.unwrap();
            let value = values[index];
            rsx! {
                label {
                    key: "{key}",
                    height: "25",
                    "{index} {value}"
                }
            }
        })
    })
}
