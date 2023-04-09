use std::time::Duration;

use crate::{use_editable, EditableMode, TextEditor};
use freya::prelude::*;
use freya_elements::events::keyboard::{Code, Key, Modifiers};
use freya_testing::{launch_test, FreyaEvent, MouseButton};
use tokio::time::timeout;

#[tokio::test]
pub async fn multiple_lines_single_editor() {
    fn use_editable_app(cx: Scope) -> Element {
        let editable = use_editable(
            cx,
            || EditableConfig::new("Hello Rustaceans".to_string()),
            EditableMode::MultipleLinesSingleEditor,
        );
        let cursor_attr = editable.cursor_attr(cx);
        let editor = editable.editor();
        let cursor = editor.cursor();
        let cursor_pos = editor.cursor_pos();

        let onmousedown = {
            to_owned![editable];
            move |e: MouseEvent| {
                editable.process_event(&EditableEvent::MouseDown(e.data, 0));
            }
        };

        let onkeydown = {
            to_owned![editable];
            move |e: Event<KeyboardData>| {
                editable.process_event(&EditableEvent::KeyDown(e.data));
            }
        };

        render!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference: cursor_attr,
                onmousedown: onmousedown,
                paragraph {
                    height: "50%",
                    width: "100%",
                    cursor_id: "0",
                    cursor_index: "{cursor_pos}",
                    cursor_color: "black",
                    cursor_mode: "editable",
                    onkeydown: onkeydown,
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

    utils.wait_for_update().await;
    utils.wait_for_update().await;

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

    utils.wait_for_update().await;

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
            || EditableConfig::new("Hello Rustaceans\nHello World".to_string()),
            EditableMode::SingleLineMultipleEditors,
        );
        let cursor_attr = editable.cursor_attr(cx);
        let editor = editable.editor().clone();

        let onkeydown = {
            to_owned![editable];
            move |e: Event<KeyboardData>| {
                editable.process_event(&EditableEvent::KeyDown(e.data));
            }
        };

        render!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference: cursor_attr,
                onkeydown: onkeydown,
                editor.lines().enumerate().map(move |(i, line)| {

                    let onmousedown = {
                        to_owned![editable];
                        move |e: MouseEvent| {
                            editable.process_event(&EditableEvent::MouseDown(e.data, 0));
                        }
                    };

                    rsx!(
                        paragraph {
                            width: "100%",
                            height: "30",
                            max_lines: "1",
                            cursor_id: "0",
                            cursor_index: "{i}",
                            cursor_color: "black",
                            cursor_mode: "editable",
                            onmousedown:  onmousedown,
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

    utils.wait_for_update().await;
    utils.wait_for_update().await;

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

    utils.wait_for_update().await;

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

#[tokio::test]
pub async fn highlight_multiple_lines_single_editor() {
    fn use_editable_app(cx: Scope) -> Element {
        let editable = use_editable(
            cx,
            || EditableConfig::new("Hello Rustaceans\n".repeat(2)),
            EditableMode::MultipleLinesSingleEditor,
        );
        let cursor_attr = editable.cursor_attr(cx);
        let editor = editable.editor();
        let cursor = editor.cursor();
        let cursor_pos = editor.cursor_pos();
        let highlights_attr = editable.highlights_attr(cx, 0);

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

        let onkeydown = {
            to_owned![editable];
            move |e: Event<KeyboardData>| {
                editable.process_event(&EditableEvent::KeyDown(e.data));
            }
        };

        render!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference: cursor_attr,
                paragraph {
                    height: "50%",
                    width: "100%",
                    cursor_id: "0",
                    cursor_index: "{cursor_pos}",
                    cursor_color: "black",
                    cursor_mode: "editable",
                    highlights: highlights_attr,
                    onkeydown: onkeydown,
                    onmousedown: onmousedown,
                    onmouseover: onmouseover,
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

    let root = utils.root().child(0).unwrap();

    // Click cursor
    utils.push_event(FreyaEvent::Mouse {
        name: "mousedown",
        cursor: (35.0, 3.0),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update((500.0, 500.0)).await;

    // Move cursor
    utils.push_event(FreyaEvent::Mouse {
        name: "mouseover",
        cursor: (80.0, 20.0),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update((500.0, 500.0)).await;
    utils.wait_for_update((500.0, 500.0)).await;

    let highlights = root
        .child(0)
        .unwrap()
        .state()
        .cursor_settings
        .highlights
        .clone();

    #[cfg(not(target_os = "linux"))]
    let start = 5;
    #[cfg(not(target_os = "linux"))]
    let end = 28;

    #[cfg(target_os = "linux")]
    let start = 4;
    #[cfg(target_os = "linux")]
    let end = 27;

    assert_eq!(highlights, Some(vec![(start, end)]))
}

#[tokio::test]
pub async fn highlights_single_line_mulitple_editors() {
    fn use_editable_app(cx: Scope) -> Element {
        let editable = use_editable(
            cx,
            || EditableConfig::new("Hello Rustaceans\n".repeat(2)),
            EditableMode::SingleLineMultipleEditors,
        );
        let cursor_attr = editable.cursor_attr(cx);
        let editor = editable.editor().clone();

        let onkeydown = {
            to_owned![editable];
            move |e: Event<KeyboardData>| {
                editable.process_event(&EditableEvent::KeyDown(e.data));
            }
        };

        render!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference: cursor_attr,
                onkeydown: onkeydown,
                direction: "vertical",
                editor.lines().enumerate().map(move |(i, line)| {

                    let highlights_attr = editable.highlights_attr(cx, i);

                    let is_line_selected = editable.editor().cursor_row() == i;

                    // Only show the cursor in the active line
                    let character_index = if is_line_selected {
                        editable.editor().cursor_col().to_string()
                    } else {
                        "none".to_string()
                    };

                    let onmouseover = {
                        to_owned![editable];
                        move |e: MouseEvent| {
                            editable.process_event(&EditableEvent::MouseOver(e.data, i));
                        }
                    };

                    let onmousedown = {
                        to_owned![editable];
                        move |e: MouseEvent| {
                            editable.process_event(&EditableEvent::MouseDown(e.data, i));
                        }
                    };

                    rsx!(
                        paragraph {
                            width: "100%",
                            height: "30",
                            max_lines: "1",
                            cursor_id: "{i}",
                            cursor_index: "{character_index}",
                            cursor_color: "black",
                            cursor_mode: "editable",
                            onmouseover:  onmouseover,
                            onmousedown:  onmousedown,
                            highlights: highlights_attr,
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

    let root = utils.root().child(0).unwrap();

    // Click cursor
    utils.push_event(FreyaEvent::Mouse {
        name: "mousedown",
        cursor: (35.0, 3.0),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update((500.0, 500.0)).await;

    // Move cursor
    utils.push_event(FreyaEvent::Mouse {
        name: "mouseover",
        cursor: (35.0, 3.0),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update((500.0, 500.0)).await;
    utils.wait_for_update((500.0, 500.0)).await;

    // Move cursor
    utils.push_event(FreyaEvent::Mouse {
        name: "mouseover",
        cursor: (80.0, 35.0),
        button: Some(MouseButton::Left),
    });

    timeout(
        Duration::from_millis(100),
        utils.wait_for_update((500.0, 500.0)),
    )
    .await
    .ok();
    utils.wait_for_update((500.0, 500.0)).await;

    let highlights_1 = root
        .child(0)
        .unwrap()
        .state()
        .cursor_settings
        .highlights
        .clone();

    #[cfg(not(target_os = "linux"))]
    let start = 5;
    #[cfg(not(target_os = "linux"))]
    let end = 17;

    #[cfg(target_os = "linux")]
    let start = 4;
    #[cfg(target_os = "linux")]
    let end = 17;

    assert_eq!(highlights_1, Some(vec![(start, end)]));

    let highlights_2 = root
        .child(1)
        .unwrap()
        .state()
        .cursor_settings
        .highlights
        .clone();

    #[cfg(not(target_os = "linux"))]
    let start = 0;
    #[cfg(not(target_os = "linux"))]
    let end = 11;

    #[cfg(target_os = "linux")]
    let start = 0;
    #[cfg(target_os = "linux")]
    let end = 10;

    assert_eq!(highlights_2, Some(vec![(start, end)]));
}
