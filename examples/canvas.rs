#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::dioxus_elements::MouseEvent;
use freya::prelude::*;

fn main() {
    launch_with_props(app, "Freya canvas experiment", (700, 570));
}

fn app(cx: Scope) -> Element {
    use_init_focus(cx);
    let hovering = use_state(cx, || false);
    let canvas_pos = use_state(cx, || (0.0f64, 0.0f64));
    let nodes = use_state(cx, || vec![(0.0f64, 0.0f64)]);
    let clicking = use_state::<Option<(f64, f64)>>(&cx, || None);
    let clicking_drag = use_state::<Option<(usize, (f64, f64))>>(&cx, || None);

    let onmouseleave = |_: MouseEvent| {
        if clicking.is_none() {
            hovering.set(false);
        }
    };

    let onmouseover = |e: MouseEvent| {
        hovering.set(true);
        if let Some(clicking_cords) = clicking.get() {
            let coordinates = e.get_screen_coordinates();
            canvas_pos.set((
                coordinates.x + clicking_cords.0,
                coordinates.y + clicking_cords.1,
            ));
        }
        if let Some((node_id, clicking_cords)) = clicking_drag.get() {
            let coordinates = e.get_screen_coordinates();

            nodes.with_mut(|nodes| {
                let node = nodes.get_mut(*node_id).unwrap();
                node.0 = coordinates.x - clicking_cords.0 - canvas_pos.0;
                node.1 = coordinates.y - clicking_cords.1 - canvas_pos.1 - 25.0;
                // The 25 is because of label from below.
            });
        }
    };

    let onmousedown = |e: MouseEvent| {
        let coordinates = e.get_screen_coordinates();
        clicking.set(Some((
            canvas_pos.0 - coordinates.x,
            canvas_pos.1 - coordinates.y,
        )));
    };

    let onclick = |_: MouseEvent| {
        clicking.set(None);
        clicking_drag.set(None);
    };

    let create_node = |_: MouseEvent| {
        nodes.with_mut(|nodes| {
            nodes.push((0.0f64, 0.0f64));
        });
    };

    render!(
        rect {
            width: "100%",
            height: "100%",
            color: "white",
            rect {
                background: "rgb(35, 35, 35)",
                width: "100%",
                height: "calc(100% - 100)",
                scroll_x: "{canvas_pos.0}",
                scroll_y: "{canvas_pos.1}",
                onmousedown: onmousedown,
                onclick: onclick,
                onmouseover: onmouseover,
                onmouseleave: onmouseleave,
                label {
                    font_size: "25",
                    "What is this even about? I have no idea, but it's cool"
                }
                nodes.get().iter().enumerate().map(|(id, node)| {
                    rsx! {
                        rect {
                            key: "{id}",
                            direction: "horizontal",
                            rect {
                                scroll_x: "{node.0}",
                                scroll_y: "{node.1}",
                                container {
                                    background: "rgb(20, 20, 20)",
                                    width: "600",
                                    height: "400",
                                    radius: "15",
                                    padding: "10",
                                    shadow: "0 0 60 35 white",
                                    onmousedown:  move |e: MouseEvent| {
                                        clicking_drag.set(Some((id, e.get_element_coordinates().to_tuple())));
                                    },
                                    onmouseleave: |_: MouseEvent| {
                                        if clicking.is_none() {
                                            hovering.set(false);
                                        }
                                    },
                                    Editor {

                                    }
                                }
                            }
                        }
                    }
                })
            }
            rect {
                background: "rgb(35, 35, 35)",
                height: "100",
                width: "100%",
                display: "center",
                direction: "horizontal",
                padding: "15",
                rect {
                    layer: "-100",
                    padding: "10",
                    radius: "7",
                    width: "170",
                    height: "100%",
                    radius: "15",
                    display: "center",
                    direction: "both",
                    background: "rgb(20, 20, 20)",
                    Button {
                        onclick: create_node,
                        label {
                            color: "white",
                            "Create new node"
                        }
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
fn Editor(cx: Scope) -> Element {
    let (focused, focus_id, focus) = use_raw_focus(cx);
    let (text_editor, process_keyevent, process_clickevent, cursor_ref) = use_editable(
        &cx,
        || {
            "Lorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet"
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

    use_effect(cx, (), move |_| {
        focus.map(|f| *f.write() = focus_id);
        async move {}
    });

    render!(
        rect {
            onclick: move |_| {
                focus.map(|f| *f.write() = focus_id);
            },
            width: "100%",
            height: "100%",
            rect {
                width: "100%",
                height: "50",
                padding: "10",
                direction: "horizontal",
                rect {
                    height: "100%",
                    width: "100%",
                    direction: "horizontal",
                    padding: "5",
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
                height: "calc(100% - 80)",
                padding: "5",
                onkeydown: move |e| {
                    if focused {
                        process_keyevent.send(e.data).unwrap();
                    }
                },
                cursor_reference: cursor_ref,
                direction: "horizontal",
                rect {
                    width: "100%",
                    height: "100%",
                    padding: "5",
                    ScrollView {
                        width: "100%",
                        height: "100%",
                        show_scrollbar: true,
                        text_editor.lines().map(move |l| {
                            let process_clickevent = process_clickevent.clone();

                            let is_line_selected = text_editor.cursor_row() == line_index;

                            // Only show the cursor in the active line
                            let character_index = if is_line_selected {
                                text_editor.cursor_col().to_string()
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
            }
        }
    )
}
