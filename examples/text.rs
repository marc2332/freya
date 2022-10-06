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
    let font_size = use_state(&cx, || 20.0);

    render!(
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
            Slider {
                width: 100.0,
                onmoved: |e| {
                    font_size.set(e + 20.0); // Minimum is 20
                }
            }
        }
    )
}
