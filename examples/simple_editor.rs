#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Simple editor", (900.0, 650.0));
}

fn app() -> Element {
    let mut editable = use_editable(
        || {
            EditableConfig::new(
                "你好世界 👋| Hello World! 🙍‍♂️| Hola Mundo! 🚀| Hola Món! 🦀\n"
                    .repeat(15)
                    .trim()
                    .to_string(),
            )
            .with_allow_tabs(true)
        },
        EditableMode::MultipleLinesSingleEditor,
    );

    let cursor_reference = editable.cursor_attr();
    let highlights = editable.highlights_attr(0);
    let editor = editable.editor().read();
    let cursor_char = editor.cursor_pos();

    let onmousedown = move |e: MouseEvent| {
        editable.process_event(&EditableEvent::MouseDown(e.data, 0));
    };

    let onmousemove = move |e: MouseEvent| {
        editable.process_event(&EditableEvent::MouseMove(e.data, 0));
    };

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
            ScrollView {
                width: "100%",
                height: "calc(100% - 30)",
                scroll_with_arrows: false,
                paragraph {
                    width: "100%",
                    height: "100%",
                    main_align: "center",
                    cursor_id: "0",
                    cursor_index: "{cursor_char}",
                    cursor_mode: "editable",
                    cursor_color: "black",
                    highlights,
                    cursor_reference,
                    onclick,
                    onmousemove,
                    onmousedown,
                    onglobalkeydown,
                    onglobalkeyup,
                    text {
                        "{editable.editor()}"
                    }
                }
            }
            label {
                color: "black",
                height: "30",
                "{editor.cursor_row()}:{editor.cursor_col()}"
            }
        }
    )
}
