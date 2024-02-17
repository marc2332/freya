#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut editable = use_editable(
        || {
            EditableConfig::new("Hello Rustaceans Abcdefg12345 Hello Rustaceans Abcdefg12345 Hello Rustaceans Abcdefg12345\n".repeat(25).trim().to_string())
        },
        EditableMode::MultipleLinesSingleEditor,
    );

    let cursor_reference = editable.cursor_attr();
    let highlights = editable.highlights_attr(0);
    let editor = editable.editor().read();
    let cursor = editor.cursor();
    let cursor_char = editor.cursor_pos();

    let onmousedown = move |e: MouseEvent| {
        editable.process_event(&EditableEvent::MouseDown(e.data, 0));
    };

    let onmouseover = move |e: MouseEvent| {
        editable.process_event(&EditableEvent::MouseOver(e.data, 0));
    };

    let onclick = move |_: MouseEvent| {
        editable.process_event(&EditableEvent::Click);
    };

    let onkeydown = move |e: KeyboardEvent| {
        editable.process_event(&EditableEvent::KeyDown(e.data));
    };

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            cursor_reference,
            ScrollView {
                theme: theme_with!(ScrollViewTheme {
                    width: "100%".into(),
                    height: "calc(100% - 30)".into(),
                }),
                scroll_with_arrows: false,
                paragraph {
                    width: "100%",
                    cursor_id: "0",
                    cursor_index: "{cursor_char}",
                    cursor_mode: "editable",
                    cursor_color: "black",
                    highlights,
                    onclick,
                    onmouseover,
                    onmousedown,
                    onkeydown,
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
