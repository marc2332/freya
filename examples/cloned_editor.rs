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
    let click_notifier = editable.click_notifier().clone();
    let keypress_notifier = editable.keypress_notifier().clone();
    let cursor_attr = editable.cursor_attr(cx);
    let editor = editable.editor().get().clone();

    let onclick = {
        to_owned![click_notifier];
        move |_: MouseEvent| {
            click_notifier.send(EditableEvent::Click).ok();
        }
    };

    render!(
        rect {
            width: "100%",
            height: "100%",
            padding: "10",
            onkeydown: move |e| {
                keypress_notifier.send(e.data).unwrap();
            },
            cursor_reference: cursor_attr,
            direction: "horizontal",
            onglobalclick: onclick,
            background: "{theme.body.background}",
            VirtualScrollView {
                width: "50%",
                height: "100%",
                show_scrollbar: true,
                length: editor.len_lines(),
                item_size: 35.0,
                builder_values: (editor.clone(), click_notifier.clone(), editable.clone()),
                builder: Box::new(move |(key, line_index, cx, values)| {
                    let (editor, click_notifier, editable) = values.as_ref().unwrap();
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

                    let highlights = editable.highlights_attr(&cx, line_index);

                    rsx! {
                        rect {
                            key: "{key}",
                            width: "100%",
                            height: "35",
                            direction: "horizontal",
                            background: "{line_background}",
                            rect {
                                width: "30",
                                height: "100%",
                                display: "center",
                                direction: "horizontal",
                                label {
                                    font_size: "15",
                                    color: "rgb(200, 200, 200)",
                                    "{line_index + 1} "
                                }
                            }
                            paragraph {
                                height: "100%",
                                width: "100%",
                                cursor_index: "{character_index}",
                                cursor_color: "white",
                                max_lines: "1",
                                cursor_mode: "editable",
                                cursor_id: "{line_index}",
                                onmousedown: onmousedown,
                                onmouseover: onmouseover,
                                highlights: highlights,
                                text {
                                    color: "rgb(240, 240, 240)",
                                    font_size: "15",
                                    "{line}"
                                }
                            }
                        }
                    }
                })
            }
            VirtualScrollView {
                width: "50%",
                height: "100%",
                show_scrollbar: true,
                length: editor.len_lines(),
                item_size: 35.0,
                builder_values: (editor.clone(), click_notifier.clone(), editable.clone()),
                builder: Box::new(move |(key, line_index, cx, values)| {
                    let (editor, click_notifier, editable) = values.as_ref().unwrap();
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

                    let highlights = editable.highlights_attr(&cx, line_index);

                    rsx! {
                        rect {
                            key: "{key}",
                            width: "100%",
                            height: "35",
                            direction: "horizontal",
                            background: "{line_background}",
                            rect {
                                width: "30",
                                height: "100%",
                                display: "center",
                                direction: "horizontal",
                                label {
                                    font_size: "15",
                                    color: "rgb(200, 200, 200)",
                                    "{line_index + 1} "
                                }
                            }
                            paragraph {
                                height: "100%",
                                width: "100%",
                                cursor_index: "{character_index}",
                                cursor_color: "white",
                                max_lines: "1",
                                cursor_mode: "editable",
                                cursor_id: "{line_index}",
                                onmousedown: onmousedown,
                                onmouseover: onmouseover,
                                highlights: highlights,
                                text {
                                    color: "rgb(240, 240, 240)",
                                    font_size: "15",
                                    "{line}"
                                }
                            }
                        }
                    }
                })
            }
        }
    )
}
