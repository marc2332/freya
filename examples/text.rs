#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::prelude::*;
use freya::{dioxus_elements, *};

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let percentage = use_state(&cx, || 20.0);
    let font_size = percentage + 20.0;

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
                    percentage.set(20.0);
                },
                label {
                    width: "80", 
                    "Reset size"
                }
            }
            Slider {
                width: 100.0,
                value: *percentage.get(),
                onmoved: |p| {
                    percentage.set(p);
                }
            }
        }
    ))
}
