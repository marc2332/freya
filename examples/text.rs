#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::prelude::*;
use freya::{dioxus_elements, *};

const MAX_FONT_SIZE: f64 = 100.0;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let percentage = use_state(&cx, || 0.2);
    let font_size = percentage * MAX_FONT_SIZE + 20.0;

    cx.render(rsx!(
        rect {
            width: "100%",
            height: "100%",
            background: "black",
            padding: "20",
            label {
                font_size: "{font_size}",
                font_family: "Inter",
                height: "150",
                "Hello World"
            }
            Button {
                on_click: move |_| {
                    percentage.set(0.2);
                },
                label { "Reset size" }
            }
            Slider {
                width: 100.0,
                state: percentage,
            }
        }
    ))
}
