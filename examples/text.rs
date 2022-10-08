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
            Slider {
                width: 100.0,
                onmoved: |e| {
                    font_size.set(e + 20.0); // Minimum is 20
                }
            }
            ScrollView {
                show_scrollbar: true,
                height: "calc(100% - 20)",
                rect {
                    background: "red",
                    direction: "both",
                    label {
                        font_size: "{font_size}",
                        font_family: "Inter",
                        "Hello World 1"
                    }
                }
                label {
                    font_size: "{font_size / 2f64}",
                    font_family: "Inter",
                    "Hello World 2"
                }
                label {
                    font_size: "{font_size / 3f64}",
                    font_family: "Inter",
                    "Hello World 3"
                }
                label {
                    font_size: "{font_size / 2f64}",
                    font_family: "Inter",
                    "Hello World Hello World Hello World Hello World Hello World Hello World Hello World Hello World Hello World Hello World Hello World"
                }
            }
        }
    )
}
