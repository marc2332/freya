#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::{core::UiEvent, prelude::*};
use freya::{
    dioxus_elements::{self, events::KeyboardData},
    *,
};

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let (content, cursor, process_keyevent) = use_editable(&cx);
    let percentage = use_state(&cx, || 15.0);

    // minimum font size is 5
    let font_size = percentage + 5.0;
    let mut line_index = 0;
    let content_lines = content.lines(0..);

    render!(
        rect {
            width: "100%",
            height: "calc(100% - 20)",
            onkeydown: move |e: UiEvent<KeyboardData>| process_keyevent(e),
            ScrollView {
                show_scrollbar: true,
                content_lines.into_iter().map(|l| {
                    // Only show the cursor in the active line
                    let character_index = if cursor.1 == line_index {
                        cursor.0.to_string()
                    } else {
                        "none".to_string()
                    };
                    line_index += 1;
                    rsx! {
                        rect {
                            key: "{line_index}",
                            width: "100%",
                            height: "{font_size}",
                            direction: "horizontal",
                            rect {
                                background: "rgb(220, 220, 220)",
                                width: "{font_size * 2.0}",
                                height: "100%",
                                display: "center",
                                direction: "horizontal",
                                label {
                                    font_size: "{font_size}",
                                    color: "rgb(80, 80, 80)",
                                    "{line_index} "
                                }
                            }
                            paragraph {
                                width: "100%",
                                cursor_index: "{character_index}",
                                cursor_color: "black",
                                max_lines: "1",
                                text {
                                    color: "rgb(25, 25, 25)",
                                    font_size: "{font_size}",
                                    "{l} "
                                }
                            }
                        }
                    }
                })
            }
        }
        rect {
            width: "100%",
            height: "20",
            background: "rgb(190, 190, 190)",
            direction: "horizontal",
            label {
                color: "rgb(25, 25, 25)",
                width: "100",
                "Ln {cursor.1 + 1}, Col {cursor.0 + 1}"
            }
            Slider {
                width: 100.0,
                value: *percentage.get(),
                onmoved: |p| {
                    percentage.set(p);
                }
            }
        }
    )
}
