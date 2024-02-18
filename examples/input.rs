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
            label {
                color: "black",
                "Your name:"
            }
            Input {
                value: values.read().0.clone(),
                onchange: move |txt| {
                    values.write().0 = txt;
                }
            },
            label {
                color: "black",
                "Your age:"
            }
            Input {
                value: values.read().1.clone(),
                onchange: move |txt| {
                    values.write().1 = txt;
                }
            },
            label {
                color: "black",
                "You are {values.read().0} and you are {values.read().1} years old."
            }
        }
    )
}
