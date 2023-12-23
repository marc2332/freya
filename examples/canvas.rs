#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::events::MouseEvent;
use freya::prelude::*;

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_title("canvas")
            .with_width(700.)
            .with_height(500.)
            .with_plugin(PerformanceOverlayPlugin::default())
            .build(),
    )
}

fn app(cx: Scope) -> Element {
    use_init_theme(cx, DARK_THEME);
    let hovering = use_state(cx, || false);
    let canvas_pos = use_state(cx, || (0.0f64, 0.0f64));
    let nodes = use_state(cx, || vec![(0.0f64, 0.0f64)]);
    let clicking = use_state::<Option<(f64, f64)>>(cx, || None);
    let clicking_drag = use_state::<Option<(usize, (f64, f64))>>(cx, || None);

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
                offset_x: "{canvas_pos.0}",
                offset_y: "{canvas_pos.1}",
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
                                offset_x: "{node.0}",
                                offset_y: "{node.1}",
                                width: "0",
                                height: "0",
                                rect {
                                    overflow: "clip",
                                    background: "rgb(20, 20, 20)",
                                    width: "600",
                                    height: "400",
                                    corner_radius: "15",
                                    padding: "10",
                                    shadow: "0 0 30 0 rgb(0, 0, 0, 150)",
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
                background: "rgb(25, 25, 25)",
                height: "100",
                width: "100%",
                main_align: "center",
                cross_align: "center",
                padding: "15",
                layer: "-100",
                shadow: "0 -2 5 0 rgb(0, 0, 0, 0.1)",
                direction: "horizontal",
                label {
                    "Create as many editors you want!"
                }
                Button {
                    theme: theme_with!(ButtonTheme {
                        margin: "0 20".into(),
                    }),
                    onclick: create_node,
                    label {
                        "New Editor"
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
fn Editor(cx: Scope) -> Element {
    let focus_manager = use_focus(cx);
    let editable = use_editable(
        cx,
        || {
            EditableConfig::new("Lorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet".to_string())
        },
        EditableMode::SingleLineMultipleEditors,
    );
    let cursor_attr = editable.cursor_attr(cx);
    let editor = editable.editor().clone();
    let cursor = editor.cursor().clone();

    let font_size_percentage = use_state(cx, || 15.0);
    let line_height_percentage = use_state(cx, || 0.0);
    let is_bold = use_state(cx, || false);
    let is_italic = use_state(cx, || false);

    // minimum font size is 5
    let font_size = font_size_percentage + 5.0;
    let line_height = (line_height_percentage / 25.0) + 1.2;
    let mut line_index = 0;
    let font_style = if *is_italic.get() { "italic" } else { "normal" };
    let font_weight = if *is_bold.get() { "bold" } else { "normal" };

    use_on_create(cx, move || {
        focus_manager.focus();
        async move {}
    });

    let onclick = {
        to_owned![focus_manager];
        move |_: MouseEvent| {
            if !focus_manager.is_focused() {
                focus_manager.focus();
            }
        }
    };

    let onkeydown = {
        to_owned![editable];
        move |e: KeyboardEvent| {
            if focus_manager.is_focused() {
                editable.process_event(&EditableEvent::KeyDown(e.data));
            }
        }
    };

    render!(
        rect {
            onclick: onclick,
            width: "100%",
            height: "100%",
            rect {
                width: "100%",
                height: "70",
                padding: "5",
                direction: "horizontal",
                cross_align: "center",
                rect {
                    width: "130",
                    cross_align: "center",
                    Slider {
                        width: 100.0,
                        value: *font_size_percentage.get(),
                        onmoved: |p| {
                            font_size_percentage.set(p);
                        }
                    }
                    label {
                        "Font size"
                    }
                }
                rect {
                    width: "130",
                    cross_align: "center",
                    Slider {
                        width: 100.0,
                        value: *line_height_percentage.get(),
                        onmoved: |p| {
                            line_height_percentage.set(p);
                        }
                    }
                    label {
                        "Line height"
                    }
                }
                rect {
                    width: "80",
                    cross_align: "center",
                    Switch {
                        enabled: *is_bold.get(),
                        ontoggled: |_| {
                            is_bold.set(!is_bold.get());
                        }
                    }
                    label {
                        "Bold"
                    }
                }
                rect {
                    width: "80",
                    cross_align: "center",
                    Switch {
                        enabled: *is_italic.get(),
                        ontoggled: |_| {
                            is_italic.set(!is_italic.get());
                        }
                    }
                    label {
                        "Italic"
                    }
                }
            }
            rect {
                width: "100%",
                height: "calc(100% - 80)",
                padding: "5",
                onkeydown: onkeydown,
                cursor_reference: cursor_attr,
                direction: "horizontal",
                rect {
                    width: "100%",
                    height: "100%",
                    padding: "5",
                    ScrollView {
                        scroll_with_arrows: false,
                        editor.lines().map(move |l| {
                            let editable = editable.clone();

                            let is_line_selected = cursor.row() == line_index;

                            // Only show the cursor in the active line
                            let character_index = if is_line_selected {
                                cursor.col().to_string()
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
                                to_owned![editable];
                                move |e: MouseEvent| {
                                    editable.process_event(&EditableEvent::MouseDown(e.data, line_index));
                                }
                            };

                            let onmouseover = {
                                to_owned![editable];
                                move |e: MouseEvent| {
                                    editable.process_event(&EditableEvent::MouseOver(e.data, line_index));
                                }
                            };

                            let onclick = {
                                to_owned![editable];
                                move |_: MouseEvent| {
                                    editable.process_event(&EditableEvent::Click);
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
                                    corner_radius: "7",
                                    rect {
                                        width: "{font_size * 2.0}",
                                        height: "100%",
                                        main_align: "center",
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
                                            font_weight: "{font_weight}",
                                            "{l}"
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
