use freya::dioxus_elements::MouseEvent;
use freya::prelude::*;

fn main() {
    launch_cfg(
        app,
        WindowConfig::<()>::builder()
            .with_width(900)
            .with_height(500)
            .with_decorations(true)
            .with_transparency(false)
            .with_title("Editor")
            .build(),
    );
}

fn app(cx: Scope) -> Element {
    use_init_default_theme(&cx);
    render!(Body {})
}

#[allow(non_snake_case)]
fn Body(cx: Scope) -> Element {
    let theme = use_theme(&cx);
    let theme = theme.read();
    let (content, cursor, process_keyevent, process_clickevent, cursor_ref) = use_editable(
        &cx,
        || {
            "AALorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet"
        },
        EditableMode::SingleLineMultipleEditors,
    );
    let font_size_percentage = use_state(cx, || 15.0);
    let line_height_percentage = use_state(cx, || 0.0);
    let is_bold = use_state(cx, || false);
    let is_italic = use_state(cx, || false);

    // minimum font size is 5
    let font_size = font_size_percentage + 5.0;
    let line_height = (line_height_percentage / 25.0) + 1.2;
    let mut line_index = 0;

    let cursor_char = content.offset_of_line(cursor.1) + cursor.0;

    let font_style = {
        if *is_bold.get() && *is_italic.get() {
            "bold-italic"
        } else if *is_italic.get() {
            "italic"
        } else if *is_bold.get() {
            "bold"
        } else {
            "normal"
        }
    };

    content.lines(0..);

    render!(
        rect {
            width: "100%",
            height: "100%",
            color: "white",
            rect {
                width: "100%",
                height: "60",
                padding: "20",
                direction: "horizontal",
                background: "rgb(20, 20, 20)",
                rect {
                    height: "100%",
                    width: "100%",
                    direction: "horizontal",
                    padding: "10",
                    label {
                        font_size: "30",
                        "Editor"
                    }
                    rect {
                        width: "20",
                    }
                    rect {
                        height: "40%",
                        display: "center",
                        width: "130",
                        Slider {
                            width: 100.0,
                            value: *font_size_percentage.get(),
                            onmoved: |p| {
                                font_size_percentage.set(p);
                            }
                        }
                        rect {
                            height: "auto",
                            width: "100%",
                            display: "center",
                            direction: "horizontal",
                            label {
                                "Font size"
                            }
                        }

                    }
                    rect {
                        height: "40%",
                        display: "center",
                        direction: "vertical",
                        width: "130",
                        Slider {
                            width: 100.0,
                            value: *line_height_percentage.get(),
                            onmoved: |p| {
                                line_height_percentage.set(p);
                            }
                        }
                        rect {
                            height: "auto",
                            width: "100%",
                            display: "center",
                            direction: "horizontal",
                            label {
                                "Line height"
                            }
                        }
                    }
                    rect {
                        height: "40%",
                        display: "center",
                        direction: "vertical",
                        width: "60",
                        Switch {
                            enabled: *is_bold.get(),
                            ontoggled: |_| {
                                is_bold.set(!is_bold.get());
                            }
                        }
                        rect {
                            height: "auto",
                            width: "100%",
                            display: "center",
                            direction: "horizontal",
                            label {
                                "Bold"
                            }
                        }
                    }
                    rect {
                        height: "40%",
                        display: "center",
                        direction: "vertical",
                        width: "60",
                        Switch {
                            enabled: *is_italic.get(),
                            ontoggled: |_| {
                                is_italic.set(!is_italic.get());
                            }
                        }
                        rect {
                            height: "auto",
                            width: "100%",
                            display: "center",
                            direction: "horizontal",
                            label {
                                "Italic"
                            }
                        }
                    }
                }
            }
            rect {
                width: "100%",
                height: "calc(100% - 90)",
                padding: "20",
                onkeydown: move |e| {
                   process_keyevent.send(e.data).unwrap();
                },
                cursor_reference: cursor_ref,
                direction: "horizontal",
                background: "{theme.body.background}",
                rect {
                    width: "50%",
                    height: "100%",
                    padding: "30",
                    ScrollView {
                        width: "100%",
                        height: "100%",
                        show_scrollbar: true,
                        content.lines(0..).map(move |l| {
                            let process_clickevent = process_clickevent.clone();

                            let is_line_selected = cursor.1 == line_index;

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
                                process_clickevent.send((e.data, line_index)).ok();
                            };

                            let manual_line_height = font_size * line_height;

                            let cursor_id = line_index;

                            line_index += 1;
                            rsx! {
                                rect {
                                    key: "{line_index}",
                                    width: "100%",
                                    height: "{manual_line_height}",
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
                                        cursor_id: "{cursor_id}",
                                        onmousedown: onmousedown,
                                        text {
                                            color: "rgb(240, 240, 240)",
                                            font_size: "{font_size}",
                                            font_style: "{font_style}",
                                            "{l} "
                                        }
                                    }
                                }
                            }
                        })
                    }
                }
                rect {
                    background: "{theme.body.background}",
                    radius: "15",
                    width: "50%",
                    height: "100%",
                    padding: "30",
                    shadow: "0 10 30 7 white",
                    ScrollView {
                        width: "100%",
                        height: "100%",
                        show_scrollbar: true,
                        paragraph {
                            width: "100%",
                            cursor_index: "{cursor_char}",
                            cursor_color: "white",
                            line_height: "{line_height}",
                            text {
                                color: "white",
                                font_size: "{font_size}",
                                "{content}"
                            }
                        }
                    }
                }
            }
            rect {
                width: "100%",
                height: "30",
                background: "rgb(20, 20, 20)",
                direction: "horizontal",
                padding: "10",
                label {
                    color: "rgb(200, 200, 200)",
                    "Ln {cursor.1 + 1}, Col {cursor.0 + 1}"
                }
            }
        }
    )
}
