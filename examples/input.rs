#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::signals::use_signal;
use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let values = use_signal(|| (String::new(), String::new()));

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
                onchange: move |t| {
                    values.with_mut(|v| v.0 = t)
                }
            },
            label {
                color: "black",
                "Your age:"
            }
            Input {
                value: values.read().1.clone(),
                onchange: move |t| {
                    values.with_mut(|v| v.1 = t)
                }
            },
            label {
                color: "black",
                "You are {values.read().0} and you are {values.read().1} years old."
            }
        }
    )
}
