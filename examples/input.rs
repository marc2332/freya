#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let values = use_state(cx, || (String::new(), String::new()));

    render!(
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
                width: "100%",
                height: "100",
                value: values.0.clone(),
                onchange: |e| {
                    values.set((e, values.1.clone()))
                }
            },
            label {
                color: "black",
                "Your age:"
            }
            Input {
                value: values.1.clone(),
                onchange: |e| {
                    values.set((values.0.clone(), e))
                }
            },
            label {
                color: "black",
                "You are {values.0} and you are {values.1} years old."
            }
        }
    )
}
