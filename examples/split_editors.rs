use dioxus_clipboard::hooks::use_clipboard;
use dioxus_radio::hooks::{
    use_init_radio_station,
    use_radio,
    RadioChannel,
};
use freya::prelude::*;

fn main() {
    launch_with_params(app, "Editor", (900., 500.));
}

fn app() -> Element {
    let platform = use_platform();
    let clipboard = use_clipboard();
    use_init_radio_station::<State, Channel>(|| State {
        editors: vec![UseEditable::new_in_hook(
            clipboard,
            platform,
            EditableConfig::new("Hello, World!".to_string()),
            EditableMode::SingleLineMultipleEditors,
        )],
    });
    use_init_theme(|| DARK_THEME);
    let mut radio = use_radio(Channel::Editors);

    let split = move |_| {
        radio.write().editors.push(UseEditable::new_in_hook(
            clipboard,
            platform,
            EditableConfig::new("Hello, World!".to_string()),
            EditableMode::SingleLineMultipleEditors,
        ));
    };

    rsx!(
        Body {
            rect {
                cross_align: "center",
                width: "fill",
                Button {
                    onpress: split,
                    label {
                        "Split"
                    }
                }
            }
            rect {
                direction: "horizontal",
                content: "flex",
                for (editor_id, _) in radio.read().editors.iter().enumerate() {
                    Editor {
                        key: "{editor_id}",
                        editor_id
                    }
                }
            }
        }
    )
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
enum Channel {
    Editors,
    UpdatedEditor(usize),
}

impl RadioChannel<State> for Channel {}

struct State {
    editors: Vec<UseEditable>,
}

#[component]
fn Editor(editor_id: usize) -> Element {
    let mut focus = use_focus();
    let mut radio = use_radio(Channel::UpdatedEditor(editor_id));

    let editable = radio.read().editors[editor_id];
    let editor = editable.editor();

    let onclick = move |_: MouseEvent| {
        focus.request_focus();
        radio.write().editors[editor_id].process_event(&EditableEvent::Click);
    };

    let onkeydown = move |e: KeyboardEvent| {
        radio.write().editors[editor_id].process_event(&EditableEvent::KeyDown(e.data));
    };

    let onglobalkeyup = move |e: KeyboardEvent| {
        radio.write().editors[editor_id].process_event(&EditableEvent::KeyUp(e.data));
    };

    let border = if focus.is_focused() {
        "2 inner rgb(100, 100, 100)"
    } else {
        "none"
    };

    rsx!(
        rect {
            border,
            a11y_id: focus.attribute(),
            width: "flex(1)",
            height: "fill",
            padding: "10",
            onkeydown,
            onglobalkeyup,
            onclick,
            VirtualScrollView {
                length: editor.read().len_lines(),
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
                        radio.write().editors[editor_id].process_event(&EditableEvent::MouseDown(e.data, line_index));
                    };

                    let onmousemove = move |e: MouseEvent| {
                        radio.write().editors[editor_id].process_event(&EditableEvent::MouseMove(e.data, line_index));
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
                                cursor_reference: editable.cursor_attr(),
                                main_align: "center",
                                height: "100%",
                                width: "100%",
                                cursor_index: "{character_index}",
                                cursor_color: "white",
                                cursor_mode: "editable",
                                cursor_id: "{line_index}",
                                max_lines: "1",
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
        }
    )
}
