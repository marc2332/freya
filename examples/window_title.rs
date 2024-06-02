#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut text = use_signal(String::new);
    let platform = use_platform();

    let onpress = move |_| {
        let new_title = text.read().clone();
        platform.set_title(new_title);
    };

    rsx!(
        Input {
            value: text.read().clone(),
            onchange: move |txt| {
                text.set(txt);
            }
        },
        Button {
            onpress,
            label {
                "Update title"
            }
        }
    )
}
