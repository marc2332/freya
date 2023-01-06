#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let (content, cursor, process_keyevent, process_clickevent, cursor_ref) = use_editable(
        &cx,
        || {
            "Lorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet"
        },
        EditableMode::SingleLineMultipleEditors,
    );

    let font_size = 20.0;
    let line_height = 1.2;

    let real_line_height = font_size * line_height;

    render!(
        container {
            background: "rgb(15, 15, 15)",
            padding: "50",
            direction: "both",
            width: "auto",
            height: "50%",
            onkeydown: move |e| {
                process_keyevent.send(e.data).unwrap();
            },
            cursor_reference: cursor_ref,
            VirtualScrollView {
                width: "100%",
                height: "100%",
                show_scrollbar: true,
                length: content.lines(..).count() as i32,
                item_size: real_line_height,
                builder_values: (cursor, process_clickevent, content),
                builder: Box::new(move |(k, line_index, vals)| {
                    let (cursor, process_clickevent, content) = vals.as_ref().unwrap();
                    let line_content = content.lines(0..).nth(line_index  as usize).unwrap();

                    let is_line_selected = cursor.1 == line_index as usize;

                    // Only show the cursor in the active line
                    let character_index = if is_line_selected {
                        cursor.0.to_string()
                    } else {
                        "none".to_string()
                    };

                     // Only highlight the active line
                     let line_background = if is_line_selected {
                        "rgb(37, 37, 37)"
                    } else {
                        ""
                    };

                    let onmousedown = move |e: MouseEvent| {
                        process_clickevent.send((e.data, line_index as usize)).ok();
                    };

                    rsx! {
                        rect {
                            key: "{k}",
                            width: "100%",
                            height: "{real_line_height}",
                            direction: "horizontal",
                            background: "{line_background}",
                            radius: "7",
                            rect {
                                width: "{font_size * 2.0}",
                                height: "100%",
                                display: "center",
                                direction: "horizontal",
                                label {
                                    font_size: "{font_size}",
                                    color: "rgb(200, 200, 200)",
                                    "{line_index} "
                                }
                            }
                            paragraph {
                                width: "100%",
                                cursor_index: "{character_index}",
                                cursor_color: "white",
                                max_lines: "1",
                                cursor_mode: "editable",
                                cursor_id: "{line_index}",
                                onmousedown: onmousedown,
                                text {
                                    color: "rgb(240, 240, 240)",
                                    font_size: "{font_size}",
                                    font_style: "normal",
                                    "{line_content} "
                                }
                            }
                        }
                    }
                })
            }
        }
        container {
            background: "rgb(15, 15, 15)",
            padding: "50",
            direction: "both",
            width: "auto",
            height: "50%",
            VirtualScrollView {
                width: "100%",
                height: "100%",
                show_scrollbar: true,
                length: 5,
                item_size: 80.0,
                builder_values: (),
                direction: "horizontal",
                builder: Box::new(move |(k, i, _)| {
                    rsx! {
                        label {
                            key: "{k}",
                            width: "80",
                            "Number {i}"
                        }
                    }
                })
            }
        }
    )
}
