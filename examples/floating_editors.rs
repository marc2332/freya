#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    events::MouseEvent,
    prelude::*,
};

fn main() {
    launch_with_props(app, "Floating Editors", (700., 600.))
}

fn app() -> Element {
    use_init_theme(|| DARK_THEME);
    let mut nodes = use_signal(|| 1);
    let mut canvas_pos = use_signal(|| (0.0f64, 0.0f64));
    let mut clicking = use_signal::<Option<(f64, f64)>>(|| None);

    let onmousemove = move |e: MouseEvent| {
        if let Some(clicking_cords) = *clicking.peek() {
            let coordinates = e.get_screen_coordinates();
            canvas_pos.set((
                coordinates.x + clicking_cords.0,
                coordinates.y + clicking_cords.1,
            ));
        }
    };

    let onmousedown = move |e: MouseEvent| {
        let coordinates = e.get_screen_coordinates();
        clicking.set(Some((
            canvas_pos.peek().0 - coordinates.x,
            canvas_pos.peek().1 - coordinates.y,
        )));
    };

    let onclick = move |_: MouseEvent| {
        clicking.set(None);
    };

    let create_node = move |_| {
        nodes += 1;
    };

    rsx!(
        rect {
            color: "white",
            background: "rgb(35, 35, 35)",
            width: "fill",
            height: "calc(100% - 100)",
            offset_x: "{canvas_pos.read().0}",
            offset_y: "{canvas_pos.read().1}",
            onmousedown,
            onclick,
            onmousemove,
            DraggableCanvas {
                rect {
                    height: "0",
                    label {
                        font_size: "25",
                        "Floating Editors Example"
                    }
                }
                for i in 0..nodes() {
                    rect {
                        key: "{i}",
                        direction: "horizontal",
                        width: "0",
                        height: "0",
                        Draggable {
                            Editor { }
                        }
                    }
                }
             }
        }
        rect {
            color: "white",
            background: "rgb(25, 25, 25)",
            height: "fill",
            width: "fill",
            main_align: "center",
            cross_align: "center",
            padding: "15",
            layer: "-100",
            shadow: "0 -2 5 0 rgb(0, 0, 0, 0.1)",
            direction: "horizontal",
            spacing: "20",
            label {
                "Create as many editors you want!"
            }
            Button {
                onpress: create_node,
                label {
                    "New Editor"
                }
            }
        }
    )
}

#[allow(non_snake_case)]
fn Editor() -> Element {
    let mut focus_manager = use_focus();
    let mut editable = use_editable(
        || {
            EditableConfig::new("Lorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet".to_string())
        },
        EditableMode::SingleLineMultipleEditors,
    );
    let editor = editable.editor().read();
    let (cursor_row, cursor_col) = editor.cursor_row_and_col();

    let mut font_size_percentage = use_signal(|| 15.0);
    let mut line_height_percentage = use_signal(|| 0.0);
    let mut is_bold = use_signal(|| false);
    let mut is_italic = use_signal(|| false);

    // minimum font size is 5
    let font_size = font_size_percentage + 5.0;
    let line_height = (line_height_percentage / 25.0) + 1.2;
    let mut line_index = 0;
    let font_style = if *is_italic.read() {
        "italic"
    } else {
        "normal"
    };
    let font_weight = if *is_bold.read() { "bold" } else { "normal" };

    let onclick = move |_: MouseEvent| {
        if !focus_manager.is_focused() {
            focus_manager.request_focus();
        }
    };

    let onkeydown = move |e: KeyboardEvent| {
        e.stop_propagation();
        editable.process_event(&EditableEvent::KeyDown(e.data));
    };

    let onkeyup = move |e: KeyboardEvent| {
        e.stop_propagation();
        editable.process_event(&EditableEvent::KeyUp(e.data));
    };

    let a11y_id = focus_manager.attribute();

    rsx!(
        rect {
            onclick,
            a11y_id,
            a11y_auto_focus: "true",
            onkeydown,
            onkeyup,
            spacing: "15",
            overflow: "clip",
            background: "rgb(20, 20, 20)",
            width: "600",
            height: "400",
            corner_radius: "15",
            padding: "20",
            shadow: "0 0 30 0 rgb(0, 0, 0, 150)",
            rect {
                width: "fill",
                direction: "horizontal",
                cross_align: "center",
                spacing: "16",
                rect {
                    cross_align: "center",
                    Slider {
                        size: "130",
                        value: font_size_percentage(),
                        onmoved: move |p| {
                            font_size_percentage.set(p);
                        }
                    }
                    label {
                        "Font size"
                    }
                }
                rect {
                    cross_align: "center",
                    Slider {
                        size: "130",
                        value: line_height_percentage(),
                        onmoved: move |p| {
                            line_height_percentage.set(p);
                        }
                    }
                    label {
                        "Line height"
                    }
                }
                rect {
                    cross_align: "center",
                    Switch {
                        enabled: is_bold(),
                        ontoggled: move |_| {
                            is_bold.toggle();
                        }
                    }
                    label {
                        "Bold"
                    }
                }
                rect {
                    cross_align: "center",
                    Switch {
                        enabled: is_italic(),
                        ontoggled: move |_| {
                            is_italic.toggle();
                        }
                    }
                    label {
                        "Italic"
                    }
                }
            }
            rect {
                width: "fill",
                height: "fill",
                ScrollView {
                    scroll_with_arrows: false,
                    for l in editor.lines() {
                        {
                            let is_line_selected = cursor_row == line_index;
                            // Only show the cursor in the active line
                            let character_index = if is_line_selected {
                                cursor_col.to_string()
                            } else {
                                "none".to_string()
                            };

                            // Only highlight the active line
                            let line_background = if is_line_selected {
                                "rgb(37, 37, 37)"
                            } else {
                                "none"
                            };

                            let onmousedown = move |e: MouseEvent| {
                                e.stop_propagation();
                                editable.process_event(&EditableEvent::MouseDown(e.data, line_index));
                            };

                            let onmousemove = move |e: MouseEvent| {
                                editable.process_event(&EditableEvent::MouseMove(e.data, line_index));
                            };

                            let onglobalclick = move |_: MouseEvent| {
                                editable.process_event(&EditableEvent::Click);
                            };

                            let manual_line_height = font_size * line_height;

                            let cursor_id = line_index;
                            let highlights = editable.highlights_attr(cursor_id);

                            line_index += 1;
                            rsx!(
                                rect {
                                    key: "{line_index}",
                                    width: "100%",
                                    height: "{manual_line_height}",
                                    direction: "horizontal",
                                    background: "{line_background}",
                                    corner_radius: "7",
                                    label {
                                        width: "{font_size * 2.0}",
                                        height: "100%",
                                        main_align: "center",
                                        text_align: "center",
                                        font_size: "{font_size}",
                                        color: "rgb(200, 200, 200)",
                                        "{line_index} "
                                    }
                                    paragraph {
                                        height: "100%",
                                        width: "fill",
                                        main_align: "center",
                                        cursor_index: "{character_index}",
                                        cursor_color: "white",
                                        max_lines: "1",
                                        cursor_mode: "editable",
                                        cursor_id: "{cursor_id}",
                                        onmousedown,
                                        onmousemove,
                                        onglobalclick,
                                        cursor_reference: editable.cursor_attr(),
                                        highlights: highlights,
                                        text {
                                            color: "rgb(240, 240, 240)",
                                            font_size: "{font_size}",
                                            font_style: "{font_style}",
                                            font_weight: "{font_weight}",
                                            "{l}"
                                        }
                                    }
                                }
                            )
                        }
                    }
                }
            }
        }
    )
}
