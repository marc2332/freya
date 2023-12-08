#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let editable = use_editable(
        cx,
        || {
            EditableConfig::new("Hello Rustaceans Abcdefg12345 Hello Rustaceans Abcdefg12345 Hello Rustaceans Abcdefg12345\n".repeat(25).trim().to_string())
        },
        EditableMode::MultipleLinesSingleEditor,
    );
    let cursor = editable.editor().cursor();

    let cursor_attr = editable.cursor_attr(cx);
    let highlights_attr = editable.highlights_attr(cx, 0);
    let cursor_char = editable.editor().cursor_pos();

    let onmousedown = {
        to_owned![editable];
        move |e: MouseEvent| {
            editable.process_event(&EditableEvent::MouseDown(e.data, 0));
        }
    };

    let onmouseover = {
        to_owned![editable];
        move |e: MouseEvent| {
            editable.process_event(&EditableEvent::MouseOver(e.data, 0));
        }
    };

    let onclick = {
        to_owned![editable];
        move |_: MouseEvent| {
            editable.process_event(&EditableEvent::Click);
        }
    };

    let onkeydown = {
        to_owned![editable];
        move |e: KeyboardEvent| {
            editable.process_event(&EditableEvent::KeyDown(e.data));
        }
    };

    render!(
        rect { width: "100%", height: "100%", cursor_reference: cursor_attr,
            ScrollView { height: "calc(100% - 30)", width: "100%", scroll_with_arrows: false,
                paragraph {
                    width: "100%",
                    cursor_id: "0",
                    cursor_index: "{cursor_char}",
                    cursor_mode: "editable",
                    cursor_color: "black",
                    highlights: highlights_attr,
                    onclick: onclick,
                    onmouseover: onmouseover,
                    onmousedown: onmousedown,
                    onkeydown: onkeydown,
                    text { "{editable.editor()}" }
                }
            }
            label { color: "black", height: "30", "{cursor.col()}:{cursor.row()}" }
        }
    )
}
