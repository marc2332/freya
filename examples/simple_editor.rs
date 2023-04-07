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
            "Hello Rustaceans Abcdefg12345 Hello Rustaceans Abcdefg12345 Hello Rustaceans Abcdefg12345\n".repeat(25).trim().to_string()
        },
        EditableMode::MultipleLinesSingleEditor,
    );
    let keypress_notifier = editable.keypress_notifier().clone();
    let click_notifier = editable.click_notifier().clone();
    let cursor = editable.editor().cursor();

    let cursor_attr = editable.cursor_attr(cx);
    let highlights_attr = editable.highlights_attr(cx, 0);

    let onmousedown = {
        to_owned![click_notifier];
        move |e: MouseEvent| {
            click_notifier
                .send(EditableEvent::MouseDown(e.data, 0))
                .ok();
        }
    };

    let onmouseover = {
        to_owned![click_notifier];
        move |e: MouseEvent| {
            click_notifier
                .send(EditableEvent::MouseOver(e.data, 0))
                .ok();
        }
    };

    let onclick = move |_: MouseEvent| {
        click_notifier.send(EditableEvent::Click).ok();
    };

    let onkeydown = move |e: KeyboardEvent| {
        keypress_notifier.send(e.data).unwrap();
    };

    let cursor_char = editable.editor().cursor_pos();
    render!(
        rect {
            width: "100%",
            height: "100%",
            cursor_reference: cursor_attr,
            ScrollView {
                show_scrollbar: true,
                height: "calc(100% - 30)",
                width: "100%",
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
                    text {
                        "{editable.editor()}"
                    }
                }
            }
            label {
                color: "black",
                height: "30",
                "{cursor.col()}:{cursor.row()}"
            }
        }
    )
}
