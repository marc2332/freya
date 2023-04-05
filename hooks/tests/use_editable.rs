use crate::{use_editable, EditableMode, TextEditor};
use freya::prelude::*;
use freya_elements::events::keyboard::{Code, Key, Modifiers};
use freya_testing::{launch_test, FreyaEvent, MouseButton};

#[tokio::test]
pub async fn multiple_lines_single_editor() {
    fn use_editable_app(cx: Scope) -> Element {
        let editable = use_editable(
            cx,
            || "Hello Rustaceans".to_string(),
            EditableMode::MultipleLinesSingleEditor,
        );
        let keypress_notifier = editable.keypress_notifier().clone();
        let click_notifier = editable.click_notifier().clone();
        let cursor_attr = editable.cursor_attr(cx);
        let editor = editable.editor();
        let cursor = editor.cursor();
        let cursor_pos = editor.cursor_pos();
        render!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference: cursor_attr,
                onmousedown:  move |e: MouseEvent| {
                    click_notifier.send(EditableEvent::MouseDown(e.data, 0)).ok();
                },
                paragraph {
                    height: "50%",
                    width: "100%",
                    cursor_id: "0",
                    cursor_index: "{cursor_pos}",
                    cursor_color: "black",
                    cursor_mode: "editable",
                    onkeydown: move |e| {
                        keypress_notifier.send(e.data).unwrap();
                    },
                    text {
                        color: "black",
                        "{editor}"
                    }
                }
                label {
                    color: "black",
                    height: "50%",
                    "{cursor.col()}:{cursor.row()}"
                }
            }
        )
    }

    let mut utils = launch_test(use_editable_app);

    // Initial state
    let root = utils.root().child(0).unwrap();
    let cursor = root.child(1).unwrap().child(0).unwrap();
    let content = root.child(0).unwrap().child(0).unwrap().child(0).unwrap();
    assert_eq!(cursor.text(), Some("0:0"));
    assert_eq!(content.text(), Some("Hello Rustaceans"));

    // Move cursor
    utils.push_event(FreyaEvent::Mouse {
        name: "mousedown",
        cursor: (35.0, 3.0),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update((500.0, 500.0)).await;
    utils.wait_for_update((500.0, 500.0)).await;

    // Cursor has been moved
    let root = utils.root().child(0).unwrap();
    let cursor = root.child(1).unwrap().child(0).unwrap();
    #[cfg(not(target_os = "linux"))]
    assert_eq!(cursor.text(), Some("5:0"));

    #[cfg(target_os = "linux")]
    assert_eq!(cursor.text(), Some("4:0"));

    // Insert text
    utils.push_event(FreyaEvent::Keyboard {
        name: "keydown",
        key: Key::Character("!".to_string()),
        code: Code::Unidentified,
        modifiers: Modifiers::empty(),
    });

    utils.wait_for_update((500.0, 500.0)).await;

    // Text and cursor have changed
    let cursor = root.child(1).unwrap().child(0).unwrap();
    let content = root.child(0).unwrap().child(0).unwrap().child(0).unwrap();
    #[cfg(not(target_os = "linux"))]
    {
        assert_eq!(content.text(), Some("Hello! Rustaceans"));
        assert_eq!(cursor.text(), Some("6:0"));
    }

    #[cfg(target_os = "linux")]
    {
        assert_eq!(content.text(), Some("Hell!o Rustaceans"));
        assert_eq!(cursor.text(), Some("5:0"));
    }
}

#[tokio::test]
pub async fn single_line_mulitple_editors() {
    fn use_editable_app(cx: Scope) -> Element {
        let editable = use_editable(
            cx,
            || "Hello Rustaceans\nHello World".to_string(),
            EditableMode::SingleLineMultipleEditors,
        );
        let keypress_notifier = editable.keypress_notifier().clone();
        let click_notifier = editable.click_notifier().clone();
        let cursor_attr = editable.cursor_attr(cx);
        let editor = editable.editor();
        render!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference: cursor_attr,
                onkeydown: move |e| {
                    keypress_notifier.send(e.data).unwrap();
                },
                editor.lines().enumerate().map(move |(i, line)| {
                    let click_notifier = click_notifier.clone();
                    rsx!(
                        paragraph {
                            width: "100%",
                            height: "30",
                            max_lines: "1",
                            cursor_id: "0",
                            cursor_index: "{i}",
                            cursor_color: "black",
                            cursor_mode: "editable",
                            onmousedown:  move |e: MouseEvent| {
                                click_notifier.send(EditableEvent::MouseDown(e.data, i)).ok();
                            },
                            text {
                                color: "black",
                                "{line}"
                            }
                        }
                    )
                })
                label {
                    color: "black",
                    height: "50%",
                    "{editor.cursor_col()}:{editor.cursor_row()}"
                }
            }
        )
    }

    let mut utils = launch_test(use_editable_app);

    // Initial state
    let root = utils.root().child(0).unwrap();
    let cursor = root.child(2).unwrap().child(0).unwrap();
    let content = root.child(0).unwrap().child(0).unwrap().child(0).unwrap();
    assert_eq!(cursor.text(), Some("0:0"));
    assert_eq!(content.text(), Some("Hello Rustaceans\n"));

    // Move cursor
    utils.push_event(FreyaEvent::Mouse {
        name: "mousedown",
        cursor: (35.0, 3.0),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update((500.0, 500.0)).await;
    utils.wait_for_update((500.0, 500.0)).await;

    // Cursor has been moved
    let root = utils.root().child(0).unwrap();
    let cursor = root.child(2).unwrap().child(0).unwrap();
    #[cfg(not(target_os = "linux"))]
    assert_eq!(cursor.text(), Some("5:0"));

    #[cfg(target_os = "linux")]
    assert_eq!(cursor.text(), Some("4:0"));

    // Insert text
    utils.push_event(FreyaEvent::Keyboard {
        name: "keydown",
        key: Key::Character("!".to_string()),
        code: Code::Unidentified,
        modifiers: Modifiers::empty(),
    });

    utils.wait_for_update((500.0, 500.0)).await;

    // Text and cursor have changed
    let cursor = root.child(2).unwrap().child(0).unwrap();
    let content = root.child(0).unwrap().child(0).unwrap().child(0).unwrap();

    #[cfg(not(target_os = "linux"))]
    {
        assert_eq!(content.text(), Some("Hello! Rustaceans\n"));
        assert_eq!(cursor.text(), Some("6:0"));
    }

    #[cfg(target_os = "linux")]
    {
        assert_eq!(content.text(), Some("Hell!o Rustaceans\n"));
        assert_eq!(cursor.text(), Some("5:0"));
    }

    // Second line
    let content = root.child(1).unwrap().child(0).unwrap().child(0).unwrap();
    assert_eq!(content.text(), Some("Hello World"));
}
