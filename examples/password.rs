#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut password = use_signal(|| String::new());
    let mut is_hidden = use_signal(|| true);

    rsx!(
        rect {
            overflow: "clip",
            padding: "7",
            width: "100%",
            height: "100%",
            label {
                color: "black",
                "Password:"
            }
            rect {
                direction: "horizontal",
                Input {
                    mode: if !*is_hidden.read() {
                        InputMode::Shown
                    } else {
                        InputMode::new_password()
                    },
                    value: password.read().clone(),
                    onchange: move |e| {
                        password.set(e)
                    }
                },
                Button {
                    onclick: move |_| is_hidden.toggle(),
                    label {
                        if *is_hidden.read() {
                            "Show password"
                        } else {
                            "Hide password"
                        }
                    }
                }
            }
        }
    )
}
