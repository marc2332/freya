#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_clipboard::prelude::use_clipboard;
use freya::prelude::*;

fn main() {
    launch_with_props(app, "Simple editor", (900.0, 650.0));
}

fn app() -> Element {
    let platform = use_platform();
    let clipboard = use_clipboard();

    let text_editors = use_hook(|| {
        vec![
            UseEditable::new_in_hook(
                clipboard,
                platform,
                EditableConfig::new("Editor 1 -------------\n".repeat(4).trim().to_string()),
                EditableMode::MultipleLinesSingleEditor,
            ),
            UseEditable::new_in_hook(
                clipboard,
                platform,
                EditableConfig::new("Editor 2 -------------\n".repeat(4).trim().to_string()),
                EditableMode::MultipleLinesSingleEditor,
            ),
            UseEditable::new_in_hook(
                clipboard,
                platform,
                EditableConfig::new("Editor 3 -------------\n".repeat(4).trim().to_string()),
                EditableMode::MultipleLinesSingleEditor,
            ),
            UseEditable::new_in_hook(
                clipboard,
                platform,
                EditableConfig::new("Editor 4 -------------\n".repeat(4).trim().to_string()),
                EditableMode::MultipleLinesSingleEditor,
            ),
            UseEditable::new_in_hook(
                clipboard,
                platform,
                EditableConfig::new("Editor 5 -------------\n".repeat(4).trim().to_string()),
                EditableMode::MultipleLinesSingleEditor,
            ),
        ]
    });

    rsx!(
        rect {
            spacing: "10",
            for (i, editable) in text_editors.into_iter().enumerate() {
                TextEditor {
                    key: "{i}",
                    editable
                }
            }
        }
    )
}

#[component]
fn TextEditor(mut editable: UseEditable) -> Element {
    let mut focus = use_focus();
    let cursor_reference = editable.cursor_attr();
    let highlights = editable.highlights_attr(0);
    let editor = editable.editor().read();
    let cursor_char = editor.cursor_pos();

    let onmousedown = move |e: MouseEvent| {
        focus.focus();
        editable.process_event(&EditableEvent::MouseDown(e.data, 0));
    };

    let onmousemove = move |e: MouseEvent| {
        editable.process_event(&EditableEvent::MouseMove(e.data, 0));
    };

    let onglobalclick = move |_: MouseEvent| {
        editable.process_event(&EditableEvent::Click);
    };

    let onkeydown = move |e: KeyboardEvent| {
        editable.process_event(&EditableEvent::KeyDown(e.data));
    };

    let onglobalkeyup = move |e: KeyboardEvent| {
        editable.process_event(&EditableEvent::KeyUp(e.data));
    };

    rsx!(
        rect {
            background: "rgb(235, 235, 235)",
            cursor_reference,
            paragraph {
                a11y_id: focus.attribute(),
                width: "100%",
                cursor_id: "0",
                cursor_index: "{cursor_char}",
                cursor_mode: "editable",
                cursor_color: "black",
                highlights,
                onglobalclick,
                onmousemove,
                onmousedown,
                onkeydown,
                onglobalkeyup,
                text {
                    "{editable.editor()}"
                }
            }
        }
    )
}
