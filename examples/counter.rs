#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Counter", (400.0, 350.0));
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx!(
        rect {
            height: "200",
            width: "200",
            direction: "horizontal",
            rect {
                onclick: move |_| count -= 1,
                height: "200",
                width: "200",
                background: "red"
            }
            paragraph {
                onclick: move |_| count += 1,
                text {
                    font_size: "75",
                    font_weight: "bold",
                    "{count}"
                }
            }
        }
    )
}
