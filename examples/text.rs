#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    use_init_theme(DARK_THEME);
    let mut percentage = use_signal(|| 20.0);
    let font_size = percentage + 20.0;

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            background: "black",
            color: "white",
            rect {
                height: "80",
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
                    value: *percentage.read(),
                    onmoved: move |p| {
                        percentage.set(p);
                    }
                }
            }
            ScrollView {
                show_scrollbar: true,
                theme: theme_with!(ScrollViewTheme {
                    height: "calc(100% - 60)".into(),
                }),
                rect {
                    background: "red",
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
                    max_lines: "3",
                    text_overflow: "❌",
                    "Looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong text"
                }
                label {
                    font_size: "18",
                    font_family: "Inter",
                    text_align: "right",
                    width: "100%",
                    "Right align"
                }
                label {
                    font_size: "18",
                    font_family: "Inter",
                    text_align: "center",
                    width: "100%",
                    "Center align"
                }
                label {
                    font_size: "18",
                    font_family: "Inter",
                    text_align: "justify",
                    width: "100%",
                    "Justify align"
                }
                label {
                    font_size: "18",
                    font_family: "Inter",
                    text_align: "end",
                    width: "100%",
                    "End align"
                }
                label {
                    font_size: "18",
                    font_family: "Inter",
                    text_align: "start",
                    width: "100%",
                    "Start align"
                }
                label {
                    font_size: "18",
                    font_family: "Inter",
                    text_align: "left",
                    width: "100%",
                    "Left align"
                }
            }
        }
    )
}
