use freya::{
    events::MouseEvent,
    prelude::*,
};

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::new()
            .with_size(900.0, 500.0)
            .with_decorations(true)
            .with_transparency(false)
            .with_title("Editor"),
    );
}

fn app() -> Element {
    use_init_theme(|| DARK_THEME);
    rsx!(Body {})
}

#[allow(non_snake_case)]
fn Body() -> Element {
    let theme = use_applied_theme!(None, body);

    let mut editable = use_editable(
        || {
            EditableConfig::new("Lorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet\nLorem ipsum dolor sit amet".to_string())
        },
        EditableMode::SingleLineMultipleEditors,
    );
    let cursor_reference = editable.cursor_attr();
    let editor = editable.editor().read();

    let onclick = move |_: MouseEvent| {
        editable.process_event(&EditableEvent::Click);
    };

    let onglobalkeydown = move |e: KeyboardEvent| {
        editable.process_event(&EditableEvent::KeyDown(e.data));
    };

    let onglobalkeyup = move |e: KeyboardEvent| {
        editable.process_event(&EditableEvent::KeyUp(e.data));
    };

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            padding: "10",
            onglobalkeydown,
            onglobalkeyup,
            cursor_reference,
            direction: "horizontal",
            onglobalclick: onclick,
            background: "{theme.background}",
            VirtualScrollView {
                width: "50%",
                length: editor.len_lines(),
                item_size: 35.0,
                scroll_with_arrows: false,
                cache_elements: false,
                builder: move |line_index, _: &Option<()>| {
                    let editor = editable.editor().read();
                    let line = editor.line(line_index).unwrap();

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
                        "none"
                    };

                    let onmousedown = move |e: MouseEvent| {
                        editable.process_event(&EditableEvent::MouseDown(e.data, line_index));
                    };

                    let onmousemove = move |e: MouseEvent| {
                        editable.process_event(&EditableEvent::MouseMove(e.data, line_index));
                    };

                    let highlights = editable.highlights_attr(line_index);

                    rsx! {
                        rect {
                            key: "{line_index}",
                            width: "100%",
                            height: "35",
                            direction: "horizontal",
                            background: "{line_background}",
                            label {
                                main_align: "center",
                                width: "30",
                                height: "100%",
                                text_align: "center",
                                font_size: "15",
                                color: "rgb(200, 200, 200)",
                                "{line_index + 1} "
                            }
                            paragraph {
                                main_align: "center",
                                height: "100%",
                                width: "100%",
                                cursor_index: "{character_index}",
                                cursor_color: "white",
                                max_lines: "1",
                                cursor_mode: "editable",
                                cursor_id: "{line_index}",
                                onmousedown,
                                onmousemove,
                                highlights,
                                text {
                                    color: "rgb(240, 240, 240)",
                                    font_size: "15",
                                    "{line}"
                                }
                            }
                        }
                    }
                }
            }
            VirtualScrollView {
                width: "50%",
                length: editor.len_lines(),
                item_size: 60.0,
                scroll_with_arrows: false,
                cache_elements: false,
                builder: move |line_index, _: &Option<()>| {
                    let editor = editable.editor().read();
                    let line = editor.line(line_index).unwrap();

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
                        "none"
                    };

                    let onmousedown = move |e: MouseEvent| {
                        editable.process_event(&EditableEvent::MouseDown(e.data, line_index));
                    };

                    let onmousemove = move |e: MouseEvent| {
                        editable.process_event(&EditableEvent::MouseMove(e.data, line_index));
                    };

                    let highlights = editable.highlights_attr(line_index);

                    rsx! {
                        rect {
                            key: "{line_index}",
                            width: "100%",
                            height: "60",
                            direction: "horizontal",
                            background: "{line_background}",
                            label {
                                main_align: "center",
                                width: "30",
                                height: "100%",
                                text_align: "center",
                                font_size: "15",
                                color: "rgb(200, 200, 200)",
                                "{line_index + 1} "
                            }
                            paragraph {
                                main_align: "center",
                                height: "100%",
                                width: "100%",
                                cursor_index: "{character_index}",
                                cursor_color: "white",
                                max_lines: "1",
                                cursor_mode: "editable",
                                cursor_id: "{line_index}",
                                onmousedown,
                                onmousemove,
                                highlights,
                                highlight_mode: "expanded",
                                text {
                                    color: "rgb(240, 240, 240)",
                                    font_size: "15",
                                    "{line}"
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
