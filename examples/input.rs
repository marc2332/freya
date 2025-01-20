#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut values = use_signal(|| (String::new(), String::new()));

    rsx!(
        rect {
            overflow: "clip",
            padding: "7",
            width: "100%",
            height: "100%",
            spacing: "4",
            label {
                color: "black",
                "Your name:"
            }
            Input {
                value: values().0,
                placeholder: "Name",
                onchange: move |txt| {
                    values.write().0 = txt;
                }
            },
            label {
                color: "black",
                "Your age:"
            }
            Input {
                value: values().1,
                placeholder: "Age",
                onvalidate: |validator: InputValidator| {
                    if validator.text().parse::<u8>().is_err() {
                        validator.set_valid(false)
                    }
                },
                onchange: move |txt| {
                    values.write().1 = txt;
                }
            },
        }
    )
}
