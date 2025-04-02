#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut value = use_signal(String::new);
    let platform = use_platform();

    let onpress = move |_| {
        platform.set_title(value());
    };

    rsx!(
        Input {
            value,
            onchange: move |txt| {
                value.set(txt);
            }
        }
        Button {
            onpress,
            label {
                "Update title"
            }
        }
    )
}
