use freya::events::MouseEvent;
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
    use_init_default_theme(cx);
    render!(Body {})
}

#[allow(non_snake_case)]
fn Body(cx: Scope) -> Element {
    let theme = use_theme(cx);
    let theme = theme.read();

    let editable = use_editable(
        cx,
        || {
            "Lorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet".to_string()
        },
        EditableMode::SingleLineMultipleEditors,
    );
    let UseEditable {
        editor,
        keypress_notifier,
        click_notifier,
        ..
    } = editable.clone();
    let editor = editor.get();
    let cursor_char = editor.cursor_pos();
    let cursor_attr = editable.cursor_attr(cx);

    let font_size_percentage = use_state(cx, || 15.0);
    let line_height_percentage = use_state(cx, || 0.0);
    let is_bold = use_state(cx, || false);
    let is_italic = use_state(cx, || false);

    // minimum font size is 5
    let font_size = font_size_percentage + 5.0;
    let line_height = (line_height_percentage / 25.0) + 1.2;
    let mut line_index = 0;
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

    render!(
        rect {
            width: "100%",
            height: "100%",
            color: "white",
            rect {
                width: "100%",
                height: "60",
                padding: "10",
                direction: "horizontal",
                background: "rgb(20, 20, 20)",
                rect {
                    height: "100%",
                    width: "100%",
                    direction: "horizontal",
                    padding: "5",
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
                padding: "10",
                onkeydown: move |e| {
                    keypress_notifier.send(e.data).unwrap();
                },
                cursor_reference: cursor_attr,
                direction: "horizontal",
                background: "{theme.body.background}",
                rect {
                    width: "50%",
                    height: "100%",
                    padding: "15",
                    ScrollView {
                        width: "100%",
                        height: "100%",
                        show_scrollbar: true,
                        editor.lines().map(move |l| {
                            let click_notifier = click_notifier.clone();

                            let is_line_selected = editor.cursor_row() == line_index;

                            // Only show the cursor in the active line
                            let character_index = if is_line_selected {
                                editor.cursor_col().to_string()
                            } else {
                                "none".to_string()
                            };

                            // Only highlight the active line
                            let line_background = if is_line_selected {
                                "rgb(37, 37, 37)"
                            } else {
                                ""
                            };


                            let onmousedown = {
                                to_owned![click_notifier];
                                move |e: MouseEvent| {
                                    click_notifier.send(EditableEvent::MouseDown(e.data, line_index)).ok();
                                }
                            };

                            let onmouseover = {
                                to_owned![click_notifier];
                                move |e: MouseEvent| {
                                    click_notifier.send(EditableEvent::MouseOver(e.data, line_index)).ok();
                                }
                            };

                            let onclick = {
                                to_owned![click_notifier];
                                move |_: MouseEvent| {
                                    click_notifier.send(EditableEvent::Click).ok();
                                }
                            };

                            let manual_line_height = font_size * line_height;

                            let cursor_id = line_index;
                            let highlights = editable.highlights_attr(cx, cursor_id);

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
                                        onmouseover: onmouseover,
                                        onclick: onclick,
                                        highlights: highlights,
                                        text {
                                            color: "rgb(240, 240, 240)",
                                            font_size: "{font_size}",
                                            font_style: "{font_style}",
                                            "{l}"
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
                    padding: "15",
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
                                "{editor}"
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
                padding: "5",
                label {
                    color: "rgb(200, 200, 200)",
                    "Ln {editor.cursor_row() + 1}, Col {editor.cursor_col() + 1}"
                }
            }
        }
    )
}
