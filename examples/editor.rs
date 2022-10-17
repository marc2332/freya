#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::events::MouseData;
use dioxus::{core::UiEvent, prelude::*};
use freya::{
    dioxus_elements::{self},
    *,
};

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let (content, cursor, process_keyevent, process_clickevent, cursor_ref) = use_editable(
        &cx,
        || {
            "Lorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet"
        },
        EditableMode::SingleLineMultipleEditor,
    );
    let font_size_percentage = use_state(&cx, || 15.0);
    let line_height_percentage = use_state(&cx, || 0.0);

    // minimum font size is 5
    let font_size = font_size_percentage + 5.0;
    let line_height = (line_height_percentage / 25.0) + 1.2;
    let mut line_index = 0;

    let mirror_process_clickevent = process_clickevent.clone();
    let cursor_char = content.offset_of_line(cursor.1) + cursor.0;

    render!(
        rect {
            width: "100%",
            height: "calc(100% - 20)",
            onkeydown: process_keyevent,
            cursor_reference: cursor_ref,
            direction: "horizontal",
            ScrollView {
                width: "33%",
                height: "100%",
                show_scrollbar: true,
                content.lines(0..).map(move |l| {
                    let process_clickevent = process_clickevent.clone();

                    // Only show the cursor in the active line
                    let character_index = if cursor.1 == line_index {
                        cursor.0.to_string()
                    } else {
                        "none".to_string()
                    };

                    let onmousedown = move |e: UiEvent<MouseData>| {
                        process_clickevent.send((e, line_index)).ok();
                    };

                    let manual_line_height = font_size * line_height;

                    line_index += 1;
                    rsx! {
                        rect {
                            key: "{line_index}",
                            width: "100%",
                            height: "{manual_line_height}",
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
                                cursor_mode: "editable",
                                onmousedown: onmousedown,
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
            ScrollView {
                width: "33%",
                height: "100%",
                show_scrollbar: true,
                content.lines(0..).map(move |l| {
                    let process_clickevent = mirror_process_clickevent.clone();

                    // Only show the cursor in the active line
                    let character_index = if cursor.1 == line_index {
                        cursor.0.to_string()
                    } else {
                        "none".to_string()
                    };

                    let onmousedown = move |e: UiEvent<MouseData>| {
                        process_clickevent.send((e, line_index)).ok();
                    };

                    let manual_line_height = font_size * line_height;

                    line_index += 1;
                    rsx! {
                        rect {
                            key: "{line_index}",
                            width: "100%",
                            height: "{manual_line_height}",
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
                                cursor_mode: "editable",
                                onmousedown: onmousedown,
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
            ScrollView {
                width: "33%",
                height: "100%",
                show_scrollbar: true,
                paragraph {
                    width: "100%",
                    cursor_index: "{cursor_char}",
                    cursor_color: "black",
                    line_height: "{line_height}",
                    text {
                        color: "rgb(25, 25, 25)",
                        font_size: "{font_size}",
                        "{content}"
                    }
                }
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
                value: *font_size_percentage.get(),
                onmoved: |p| {
                    font_size_percentage.set(p);
                }
            }
            Slider {
                width: 100.0,
                value: *line_height_percentage.get(),
                onmoved: |p| {
                    line_height_percentage.set(p);
                }
            }
        }
    )
}
