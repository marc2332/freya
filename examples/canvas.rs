#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use freya::{dioxus_elements, *};

fn main() {
    launch_cfg(vec![
        (app, WindowConfig {
            width: 500,
            height: 500,
            title: "Cool experiment",
            decorations: true,
            transparent: false
        })
    ]);
}

fn app(cx: Scope) -> Element {
    let hovering = use_state(&cx, || false);
    let canvas_pos = use_state(&cx, || (0.0f64, 0.0f64));
    let nodes = use_state(&cx, || vec![(0.0f64, 0.0f64)]);
    let clicking = use_state::<Option<(f64, f64)>>(&cx, || None);
    let clicking_drag = use_state::<Option<usize>>(&cx, || None);

    let onmouseleave = |_: UiEvent<MouseData>| {
        if clicking.is_none() {
            hovering.set(false);
        }
    };

    let onmouseover = |e: UiEvent<MouseData>| {
        hovering.set(true);
        if let Some(clicking) = clicking.get() {
            let coordinates = e.coordinates().screen();
            canvas_pos.set((coordinates.x + clicking.0, coordinates.y + clicking.1));
        }
        if let Some(node_id) = clicking_drag.get() {
            let coordinates = e.coordinates().screen();

            nodes.with_mut(|nodes| {
                let node = nodes.get_mut(*node_id).unwrap();
                node.0 = coordinates.x - 50.0 - canvas_pos.0;
                node.1 = coordinates.y - 50.0 - canvas_pos.1;
            });
        }
    };

    let onmousedown = |e: UiEvent<MouseData>| {
        let coordinates = e.coordinates().screen();
        clicking.set(Some(( canvas_pos.0 - coordinates.x, canvas_pos.1 - coordinates.y)));
    };

    let onclick = |_: UiEvent<MouseData>| {
        clicking.set(None);
        clicking_drag.set(None);
    };

    let create_node = |_: UiEvent<MouseData>| {
        nodes.with_mut(|nodes| {
            nodes.push((0.0f64, 0.0f64));
        });
    };

    render!(
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
                                width: "400",
                                height: "250",
                                radius: "15",
                                padding: "10",
                                shadow: "0 0 60 35 white",
                                onmousedown:  move |_: UiEvent<MouseData>| {
                                    clicking_drag.set(Some(id));
                                },
                                onmouseleave: |_: UiEvent<MouseData>| {
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
            padding: "30",
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
                    on_click: create_node,
                    label {
                        color: "white",
                        "Create new node"
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
fn Editor(cx: Scope) -> Element {
    let (content, cursor, process_keyevent, process_clickevent, cursor_ref) = use_editable(
        &cx,
        || {
            "Lorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet"
        },
        EditableMode::SingleLineMultipleEditors,
    );
    let font_size_percentage = use_state(&cx, || 15.0);
    let line_height_percentage = use_state(&cx, || 0.0);
    let is_bold = use_state(&cx, || false);
    let is_italic = use_state(&cx, || false);

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
            onkeydown: process_keyevent,
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

                        let onmousedown = move |e: UiEvent<MouseData>| {
                            process_clickevent.send((e, line_index)).ok();
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
    )
}
