#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let percentage = use_state(cx, || 20.0);
    let font_size = percentage + 20.0;

    render!(
        rect {
            width: "100%",
            height: "100%",
            background: "black",
            color: "white",
            container {
                width: "100%",
                height: "60",
                Button {
                    onclick: move |_| {
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
            ScrollView {
                show_scrollbar: true,
                height: "calc(100% - 60)",
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
                label {
                    font_size: "18",
                    font_family: "Inter",
                    align: "right",
                    width: "100%",
                    "Right align"
                }
                label {
                    font_size: "18",
                    font_family: "Inter",
                    align: "center",
                    width: "100%",
                    "Center align"
                }
                label {
                    font_size: "18",
                    font_family: "Inter",
                    align: "justify",
                    width: "100%",
                    "Justify align"
                }
                label {
                    font_size: "18",
                    font_family: "Inter",
                    align: "end",
                    width: "100%",
                    "End align"
                }
                label {
                    font_size: "18",
                    font_family: "Inter",
                    align: "start",
                    width: "100%",
                    "Start align"
                }
                label {
                    font_size: "18",
                    font_family: "Inter",
                    align: "left",
                    width: "100%",
                    "Left align"
                }
            }
        }
    )
}
