use freya::prelude::*;
use freya_testing::prelude::*;

use crate::{
    use_editable,
    EditableMode,
    TextEditor,
};

#[tokio::test]
pub async fn multiple_lines_single_editor() {
    fn use_editable_app() -> Element {
        let mut editable = use_editable(
            || EditableConfig::new("Hello Rustaceans\nHello Rustaceans".to_string()),
            EditableMode::MultipleLinesSingleEditor,
        );
        let cursor_attr = editable.cursor_attr();
        let editor = editable.editor().read();
        let cursor_pos = editor.visible_cursor_pos();

        let onmousedown = move |e: MouseEvent| {
            editable.process_event(&EditableEvent::MouseDown(e.data, 0));
        };

        let onkeydown = move |e: Event<KeyboardData>| {
            editable.process_event(&EditableEvent::KeyDown(e.data));
        };

        rsx!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference: cursor_attr,
                onmousedown,
                paragraph {
                    height: "50%",
                    width: "100%",
                    cursor_id: "0",
                    cursor_index: "{cursor_pos}",
                    cursor_color: "black",
                    cursor_mode: "editable",
                    onkeydown,
                    text {
                        color: "black",
                        "{editor}"
                    }
                }
                label {
                    color: "black",
                    height: "50%",
                    "{editor.cursor_row()}:{editor.cursor_col()}"
                }
            }
        )
    }

    let mut utils = launch_test(use_editable_app);

    // Initial state
    let root = utils.root().get(0);
    let cursor = root.get(1).get(0);
    let content = root.get(0).get(0).get(0);
    assert_eq!(cursor.text(), Some("0:0"));
    assert_eq!(content.text(), Some("Hello Rustaceans\nHello Rustaceans"));

    // Move cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: (35.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;
    utils.wait_for_update().await;

    // Cursor has been moved
    let root = utils.root().get(0);
    let cursor = root.get(1).get(0);
    assert_eq!(cursor.text(), Some("0:5"));

    // Insert text
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        key: Key::Character("!".to_string()),
        code: Code::Unidentified,
        modifiers: Modifiers::empty(),
    });

    utils.wait_for_update().await;

    // Text and cursor have changed
    let cursor = root.get(1).get(0);
    let content = root.get(0).get(0).get(0);
    assert_eq!(content.text(), Some("Hello! Rustaceans\nHello Rustaceans"));
    assert_eq!(cursor.text(), Some("0:6"));

    // Move cursor to the begining
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: (3.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    utils.wait_for_update().await;
    let cursor = root.get(1).get(0);
    assert_eq!(cursor.text(), Some("0:0"));

    // Move cursor with arrow down
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowDown,
        key: Key::ArrowDown,
        modifiers: Modifiers::default(),
    });
    utils.wait_for_update().await;
    let cursor = root.get(1).get(0);
    assert_eq!(cursor.text(), Some("1:0"));

    // Move cursor with arrow right
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowRight,
        key: Key::ArrowRight,
        modifiers: Modifiers::default(),
    });
    utils.wait_for_update().await;
    let cursor = root.get(1).get(0);
    assert_eq!(cursor.text(), Some("1:1"));

    // Move cursor with arrow up
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowUp,
        key: Key::ArrowUp,
        modifiers: Modifiers::default(),
    });
    utils.wait_for_update().await;
    let cursor = root.get(1).get(0);
    assert_eq!(cursor.text(), Some("0:1"));

    // Move cursor with arrow left
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowLeft,
        key: Key::ArrowLeft,
        modifiers: Modifiers::default(),
    });
    utils.wait_for_update().await;
    let cursor = root.get(1).get(0);
    assert_eq!(cursor.text(), Some("0:0"));

    // Move cursor with arrow down, twice
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowDown,
        key: Key::ArrowDown,
        modifiers: Modifiers::default(),
    });
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowDown,
        key: Key::ArrowDown,
        modifiers: Modifiers::default(),
    });
    utils.wait_for_update().await;
    let cursor = root.get(1).get(0);
    // Because there is not a third line, the cursor will be moved to the max right
    assert_eq!(cursor.text(), Some("1:16"));

    // Move cursor with arrow up, twice
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowUp,
        key: Key::ArrowUp,
        modifiers: Modifiers::default(),
    });
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowUp,
        key: Key::ArrowUp,
        modifiers: Modifiers::default(),
    });
    utils.wait_for_update().await;
    let cursor = root.get(1).get(0);
    // Because there is not a line above the first one, the cursor will be moved to the begining
    assert_eq!(cursor.text(), Some("0:0"));
}

#[tokio::test]
pub async fn single_line_multiple_editors() {
    fn use_editable_app() -> Element {
        let mut editable = use_editable(
            || EditableConfig::new("Hello Rustaceans\nHello World".to_string()),
            EditableMode::SingleLineMultipleEditors,
        );
        let cursor_attr = editable.cursor_attr();
        let editor = editable.editor().read();

        let onkeydown = move |e: Event<KeyboardData>| {
            editable.process_event(&EditableEvent::KeyDown(e.data));
        };

        rsx!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference: cursor_attr,
                onkeydown,
                {editor.lines().enumerate().map(move |(i, line)| {

                    let onmousedown = move |e: MouseEvent| {
                        editable.process_event(&EditableEvent::MouseDown(e.data, 0));
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
                            onmousedown,
                            text {
                                color: "black",
                                "{line}"
                            }
                        }
                    )
                })},
                label {
                    color: "black",
                    height: "50%",
                    "{editor.cursor_row()}:{editor.cursor_col()}"
                }
            }
        )
    }

    let mut utils = launch_test(use_editable_app);

    // Initial state
    let root = utils.root().get(0);
    let cursor = root.get(2).get(0);
    let content = root.get(0).get(0).get(0);
    assert_eq!(cursor.text(), Some("0:0"));
    assert_eq!(content.text(), Some("Hello Rustaceans\n"));

    // Move cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: (35.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;
    utils.wait_for_update().await;

    // Cursor has been moved
    let root = utils.root().get(0);
    let cursor = root.get(2).get(0);
    assert_eq!(cursor.text(), Some("0:5"));

    // Insert text
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        key: Key::Character("!".to_string()),
        code: Code::Unidentified,
        modifiers: Modifiers::empty(),
    });

    utils.wait_for_update().await;

    // Text and cursor have changed
    let cursor = root.get(2).get(0);
    let content = root.get(0).get(0).get(0);

    assert_eq!(content.text(), Some("Hello! Rustaceans\n"));
    assert_eq!(cursor.text(), Some("0:6"));

    // Second line
    let content = root.get(1).get(0).get(0);
    assert_eq!(content.text(), Some("Hello World"));
}

#[tokio::test]
pub async fn highlight_multiple_lines_single_editor() {
    fn use_editable_app() -> Element {
        let mut editable = use_editable(
            || EditableConfig::new("Hello Rustaceans\n".repeat(2)),
            EditableMode::MultipleLinesSingleEditor,
        );
        let editor = editable.editor().read();
        let cursor_pos = editor.visible_cursor_pos();
        let cursor_reference = editable.cursor_attr();
        let highlights = editable.highlights_attr(0);

        let onmousedown = move |e: MouseEvent| {
            editable.process_event(&EditableEvent::MouseDown(e.data, 0));
        };

        let onmouseover = move |e: MouseEvent| {
            editable.process_event(&EditableEvent::MouseOver(e.data, 0));
        };

        let onkeydown = move |e: Event<KeyboardData>| {
            editable.process_event(&EditableEvent::KeyDown(e.data));
        };

        rsx!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference,
                paragraph {
                    height: "50%",
                    width: "100%",
                    cursor_id: "0",
                    cursor_index: "{cursor_pos}",
                    cursor_color: "black",
                    cursor_mode: "editable",
                    highlights,
                    onkeydown,
                    onmousedown,
                    onmouseover,
                    text {
                        color: "black",
                        "{editor}"
                    }
                }
                label {
                    color: "black",
                    height: "50%",
                    "{editor.cursor_row()}:{editor.cursor_col()}"
                }
            }
        )
    }

    let mut utils = launch_test(use_editable_app);

    let root = utils.root().get(0);

    // Click cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: (35.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;

    // Move cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseOver,
        cursor: (80.0, 20.0).into(),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;
    utils.wait_for_update().await;

    let highlights = root.child(0).unwrap().state().cursor.highlights.clone();
    #[cfg(not(target_os = "macos"))]
    assert_eq!(highlights, Some(vec![(5, 28)]));

    #[cfg(target_os = "macos")]
    assert_eq!(highlights, Some(vec![(5, 27)]));
}

#[tokio::test]
pub async fn highlights_single_line_multiple_editors() {
    fn use_editable_app() -> Element {
        let mut editable = use_editable(
            || EditableConfig::new("Hello Rustaceans\n".repeat(2)),
            EditableMode::SingleLineMultipleEditors,
        );
        let cursor_attr = editable.cursor_attr();
        let editor = editable.editor().read();

        let onkeydown = move |e: Event<KeyboardData>| {
            editable.process_event(&EditableEvent::KeyDown(e.data));
        };

        rsx!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference: cursor_attr,
                onkeydown,
                direction: "vertical",
                {editor.lines().enumerate().map(move |(i, line)| {

                    let highlights = editable.highlights_attr(i);

                    let is_line_selected = editable.editor().read().cursor_row() == i;

                    // Only show the cursor in the active line
                    let character_index = if is_line_selected {
                        editable.editor().read().visible_cursor_col().to_string()
                    } else {
                        "none".to_string()
                    };

                    let onmouseover = move |e: MouseEvent| {
                        editable.process_event(&EditableEvent::MouseOver(e.data, i));
                    };

                    let onmousedown = move |e: MouseEvent| {
                        editable.process_event(&EditableEvent::MouseDown(e.data, i));
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
                            onmouseover,
                            onmousedown,
                            highlights,
                            text {
                                color: "black",
                                "{line}"
                            }
                        }
                    )
                })},
                label {
                    color: "black",
                    height: "50%",
                    "{editor.cursor_row()}:{editor.cursor_col()}"
                }
            }
        )
    }

    let mut utils = launch_test(use_editable_app);

    let root = utils.root().get(0);

    // Click cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: (35.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;

    // Move cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseOver,
        cursor: (35.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;
    utils.wait_for_update().await;

    // Move cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseOver,
        cursor: (80.0, 35.0).into(),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;
    utils.wait_for_update().await;

    let highlights_1 = root.child(0).unwrap().state().cursor.highlights.clone();
    assert_eq!(highlights_1, Some(vec![(5, 16)]));

    let highlights_2 = root.child(1).unwrap().state().cursor.highlights.clone();
    #[cfg(not(target_os = "macos"))]
    assert_eq!(highlights_2, Some(vec![(0, 11)]));

    #[cfg(target_os = "macos")]
    assert_eq!(highlights_2, Some(vec![(0, 10)]));
}

#[tokio::test]
pub async fn special_text_editing() {
    fn special_text_editing_app() -> Element {
        let mut editable = use_editable(
            || EditableConfig::new("你好世界\n👋".to_string()),
            EditableMode::MultipleLinesSingleEditor,
        );
        let cursor_attr = editable.cursor_attr();
        let editor = editable.editor().read();
        let cursor_pos = editor.visible_cursor_pos();

        let onmousedown = move |e: MouseEvent| {
            editable.process_event(&EditableEvent::MouseDown(e.data, 0));
        };

        let onkeydown = move |e: Event<KeyboardData>| {
            editable.process_event(&EditableEvent::KeyDown(e.data));
        };

        rsx!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference: cursor_attr,
                onmousedown,
                paragraph {
                    height: "50%",
                    width: "100%",
                    cursor_id: "0",
                    cursor_index: "{cursor_pos}",
                    cursor_color: "black",
                    cursor_mode: "editable",
                    onkeydown,
                    text {
                        color: "black",
                        "{editor}"
                    }
                }
                label {
                    color: "black",
                    height: "50%",
                    "{editor.cursor_row()}:{editor.cursor_col()}"
                }
            }
        )
    }

    let mut utils = launch_test(special_text_editing_app);

    // Initial state
    let root = utils.root().get(0);
    let cursor = root.get(1).get(0);
    let content = root.get(0).get(0).get(0);
    assert_eq!(cursor.text(), Some("0:0"));
    assert_eq!(content.text(), Some("你好世界\n👋"));

    // Move cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: (35.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;
    utils.wait_for_update().await;

    // Cursor has been moved
    let root = utils.root().get(0);
    let cursor = root.get(1).get(0);
    #[cfg(not(target_os = "linux"))]
    assert_eq!(cursor.text(), Some("0:2"));

    #[cfg(target_os = "linux")]
    assert_eq!(cursor.text(), Some("0:4"));

    // Insert text
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        key: Key::Character("🦀".to_string()),
        code: Code::Unidentified,
        modifiers: Modifiers::empty(),
    });

    utils.wait_for_update().await;

    // Text and cursor have changed
    let cursor = root.get(1).get(0);
    let content = root.get(0).get(0).get(0);
    #[cfg(not(target_os = "linux"))]
    {
        assert_eq!(content.text(), Some("你好🦀世界\n👋"));
        assert_eq!(cursor.text(), Some("0:3"));
    }

    #[cfg(target_os = "linux")]
    {
        assert_eq!(content.text(), Some("你好世界🦀\n👋"));
        assert_eq!(cursor.text(), Some("0:5"));
    }

    // Move cursor to the begining
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: (3.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    utils.wait_for_update().await;
    let cursor = root.get(1).get(0);
    assert_eq!(cursor.text(), Some("0:0"));

    // Move cursor with arrow down
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowDown,
        key: Key::ArrowDown,
        modifiers: Modifiers::default(),
    });
    utils.wait_for_update().await;
    let cursor = root.get(1).get(0);
    assert_eq!(cursor.text(), Some("1:0"));

    // Move cursor with arrow right
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowRight,
        key: Key::ArrowRight,
        modifiers: Modifiers::default(),
    });
    utils.wait_for_update().await;
    let cursor = root.get(1).get(0);
    assert_eq!(cursor.text(), Some("1:1"));

    // Move cursor with arrow up
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowUp,
        key: Key::ArrowUp,
        modifiers: Modifiers::default(),
    });
    utils.wait_for_update().await;
    let cursor = root.get(1).get(0);
    assert_eq!(cursor.text(), Some("0:1"));

    // Move cursor with arrow left
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowLeft,
        key: Key::ArrowLeft,
        modifiers: Modifiers::default(),
    });
    utils.wait_for_update().await;
    let cursor = root.get(1).get(0);
    assert_eq!(cursor.text(), Some("0:0"));

    // Move cursor with arrow down, twice
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowDown,
        key: Key::ArrowDown,
        modifiers: Modifiers::default(),
    });
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowDown,
        key: Key::ArrowDown,
        modifiers: Modifiers::default(),
    });
    utils.wait_for_update().await;
    let cursor = root.get(1).get(0);
    // Because there is not a third line, the cursor will be moved to the max right
    assert_eq!(cursor.text(), Some("1:1"));

    // Move cursor with arrow up, twice
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowUp,
        key: Key::ArrowUp,
        modifiers: Modifiers::default(),
    });
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        code: Code::ArrowUp,
        key: Key::ArrowUp,
        modifiers: Modifiers::default(),
    });
    utils.wait_for_update().await;
    let cursor = root.get(1).get(0);
    // Because there is not a line above the first one, the cursor will be moved to the begining
    assert_eq!(cursor.text(), Some("0:0"));
}

#[tokio::test]
pub async fn backspace_remove() {
    fn backspace_remove_app() -> Element {
        let mut editable = use_editable(
            || EditableConfig::new("Hello Rustaceans\nHello Rustaceans".to_string()),
            EditableMode::MultipleLinesSingleEditor,
        );
        let cursor_attr = editable.cursor_attr();
        let editor = editable.editor().read();
        let cursor_pos = editor.visible_cursor_pos();

        let onmousedown = move |e: MouseEvent| {
            editable.process_event(&EditableEvent::MouseDown(e.data, 0));
        };

        let onkeydown = move |e: Event<KeyboardData>| {
            editable.process_event(&EditableEvent::KeyDown(e.data));
        };

        rsx!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference: cursor_attr,
                onmousedown,
                paragraph {
                    height: "50%",
                    width: "100%",
                    cursor_id: "0",
                    cursor_index: "{cursor_pos}",
                    cursor_color: "black",
                    cursor_mode: "editable",
                    onkeydown,
                    text {
                        color: "black",
                        "{editor}"
                    }
                }
                label {
                    color: "black",
                    height: "50%",
                    "{editor.cursor_row()}:{editor.cursor_col()}"
                }
            }
        )
    }

    let mut utils = launch_test(backspace_remove_app);

    // Initial state
    let root = utils.root().get(0);
    let cursor = root.get(1).get(0);
    let content = root.get(0).get(0).get(0);
    assert_eq!(cursor.text(), Some("0:0"));
    assert_eq!(content.text(), Some("Hello Rustaceans\nHello Rustaceans"));

    // Move cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: (35.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;
    utils.wait_for_update().await;

    // Cursor has been moved
    let root = utils.root().get(0);
    let cursor = root.get(1).get(0);
    assert_eq!(cursor.text(), Some("0:5"));

    // Insert text
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        key: Key::Character("🦀".to_string()),
        code: Code::Unidentified,
        modifiers: Modifiers::empty(),
    });

    utils.wait_for_update().await;
    utils.wait_for_update().await;

    // Text and cursor have changed
    let cursor = root.get(1).get(0);
    let content = root.get(0).get(0).get(0);
    assert_eq!(content.text(), Some("Hello🦀 Rustaceans\nHello Rustaceans"));
    assert_eq!(cursor.text(), Some("0:6"));

    // Remove text
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        key: Key::Backspace,
        code: Code::Unidentified,
        modifiers: Modifiers::empty(),
    });

    utils.wait_for_update().await;
    utils.wait_for_update().await;

    // Text and cursor have changed
    let cursor = root.get(1).get(0);
    let content = root.get(0).get(0).get(0);
    assert_eq!(content.text(), Some("Hello Rustaceans\nHello Rustaceans"));
    assert_eq!(cursor.text(), Some("0:5"));
}

#[tokio::test]
pub async fn highlight_shift_click_multiple_lines_single_editor() {
    fn use_editable_app() -> Element {
        let mut editable = use_editable(
            || EditableConfig::new("Hello Rustaceans\n".repeat(2)),
            EditableMode::MultipleLinesSingleEditor,
        );
        let editor = editable.editor().read();
        let cursor_pos = editor.visible_cursor_pos();
        let cursor_reference = editable.cursor_attr();
        let highlights = editable.highlights_attr(0);

        let onmousedown = move |e: MouseEvent| {
            editable.process_event(&EditableEvent::MouseDown(e.data, 0));
        };

        let onmouseover = move |e: MouseEvent| {
            editable.process_event(&EditableEvent::MouseOver(e.data, 0));
        };

        let onkeydown = move |e: Event<KeyboardData>| {
            editable.process_event(&EditableEvent::KeyDown(e.data));
        };

        let onclick = move |_: MouseEvent| {
            editable.process_event(&EditableEvent::Click);
        };

        rsx!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference,
                paragraph {
                    height: "50%",
                    width: "100%",
                    cursor_id: "0",
                    cursor_index: "{cursor_pos}",
                    cursor_color: "black",
                    cursor_mode: "editable",
                    highlights,
                    onkeydown,
                    onclick,
                    onmousedown,
                    onmouseover,
                    text {
                        color: "black",
                        "{editor}"
                    }
                }
                label {
                    color: "black",
                    height: "50%",
                    "{editor.cursor_row()}:{editor.cursor_col()}"
                }
            }
        )
    }

    let mut utils = launch_test(use_editable_app);

    let root = utils.root().get(0);

    // Click cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: (35.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (35.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;

    // Press shift
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        key: Key::Shift,
        code: Code::ShiftLeft,
        modifiers: Modifiers::default(),
    });
    utils.wait_for_update().await;

    // Move cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: (80.0, 20.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;

    // Click cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (80.0, 20.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    utils.wait_for_update().await;

    let highlights = root.child(0).unwrap().state().cursor.highlights.clone();
    #[cfg(not(target_os = "macos"))]
    assert_eq!(highlights, Some(vec![(5, 28)]));

    #[cfg(target_os = "macos")]
    assert_eq!(highlights, Some(vec![(5, 27)]));
}

#[tokio::test]
pub async fn highlights_shift_click_single_line_multiple_editors() {
    fn use_editable_app() -> Element {
        let mut editable = use_editable(
            || EditableConfig::new("Hello Rustaceans\n".repeat(2)),
            EditableMode::SingleLineMultipleEditors,
        );
        let cursor_attr = editable.cursor_attr();
        let editor = editable.editor().read();

        let onkeydown = move |e: Event<KeyboardData>| {
            editable.process_event(&EditableEvent::KeyDown(e.data));
        };

        rsx!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference: cursor_attr,
                onkeydown,
                direction: "vertical",
                {editor.lines().enumerate().map(move |(i, line)| {

                    let highlights = editable.highlights_attr(i);

                    let is_line_selected = editable.editor().read().cursor_row() == i;

                    // Only show the cursor in the active line
                    let character_index = if is_line_selected {
                        editable.editor().read().visible_cursor_col().to_string()
                    } else {
                        "none".to_string()
                    };

                    let onmouseover = move |e: MouseEvent| {
                        editable.process_event(&EditableEvent::MouseOver(e.data, i));
                    };

                    let onmousedown = move |e: MouseEvent| {
                        editable.process_event(&EditableEvent::MouseDown(e.data, i));
                    };

                    let onclick = move |_: MouseEvent| {
                        editable.process_event(&EditableEvent::Click);
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
                            onclick,
                            onmouseover,
                            onmousedown,
                            highlights,
                            text {
                                color: "black",
                                "{line}"
                            }
                        }
                    )
                })},
                label {
                    color: "black",
                    height: "50%",
                    "{editor.cursor_row()}:{editor.cursor_col()}"
                }
            }
        )
    }

    let mut utils = launch_test(use_editable_app);

    let root = utils.root().get(0);

    // Click cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: (35.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (35.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;

    // Press shift
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        key: Key::Shift,
        code: Code::ShiftLeft,
        modifiers: Modifiers::default(),
    });
    utils.wait_for_update().await;

    // Click cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: (80.0, 35.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (80.0, 35.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;

    utils.wait_for_update().await;

    let highlights_1 = root.child(0).unwrap().state().cursor.highlights.clone();

    assert_eq!(highlights_1, Some(vec![(5, 16)]));

    let highlights_2 = root.child(1).unwrap().state().cursor.highlights.clone();

    #[cfg(not(target_os = "macos"))]
    assert_eq!(highlights_2, Some(vec![(0, 11)]));

    #[cfg(target_os = "macos")]
    assert_eq!(highlights_2, Some(vec![(0, 10)]));
}

#[tokio::test]
pub async fn highlight_all_text() {
    fn use_editable_app() -> Element {
        let mut editable = use_editable(
            || EditableConfig::new("Hello Rustaceans\n".repeat(2)),
            EditableMode::MultipleLinesSingleEditor,
        );
        let editor = editable.editor().read();
        let cursor_pos = editor.visible_cursor_pos();
        let cursor_reference = editable.cursor_attr();
        let highlights = editable.highlights_attr(0);

        let onmousedown = move |e: MouseEvent| {
            editable.process_event(&EditableEvent::MouseDown(e.data, 0));
        };

        let onmouseover = move |e: MouseEvent| {
            editable.process_event(&EditableEvent::MouseOver(e.data, 0));
        };

        let onkeydown = move |e: Event<KeyboardData>| {
            editable.process_event(&EditableEvent::KeyDown(e.data));
        };

        let onclick = move |_: MouseEvent| {
            editable.process_event(&EditableEvent::Click);
        };

        rsx!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference,
                paragraph {
                    height: "50%",
                    width: "100%",
                    cursor_id: "0",
                    cursor_index: "{cursor_pos}",
                    cursor_color: "black",
                    cursor_mode: "editable",
                    highlights,
                    onkeydown,
                    onclick,
                    onmousedown,
                    onmouseover,
                    text {
                        color: "black",
                        "{editor}"
                    }
                }
                label {
                    color: "black",
                    height: "50%",
                    "{editor.cursor_row()}:{editor.cursor_col()}"
                }
            }
        )
    }

    let mut utils = launch_test(use_editable_app);

    let root = utils.root().get(0);

    #[cfg(target_os = "macos")]
    let modifiers = Modifiers::META;

    #[cfg(not(target_os = "macos"))]
    let modifiers = Modifiers::CONTROL;

    // Select all text
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        key: Key::Character("a".to_string()),
        code: Code::KeyA,
        modifiers,
    });
    utils.wait_for_update().await;
    utils.wait_for_update().await;

    let highlights = root.child(0).unwrap().state().cursor.highlights.clone();

    let start = 0;
    let end = 34;

    assert_eq!(highlights, Some(vec![(start, end)]))
}

#[tokio::test]
pub async fn replace_text() {
    fn replace_text_app() -> Element {
        let mut editable = use_editable(
            || EditableConfig::new("Hello Rustaceans\nHello Rustaceans".to_string()),
            EditableMode::MultipleLinesSingleEditor,
        );
        let cursor_attr = editable.cursor_attr();
        let editor = editable.editor().read();
        let cursor_pos = editor.visible_cursor_pos();
        let highlights = editable.highlights_attr(0);

        let onmousedown = move |e: MouseEvent| {
            editable.process_event(&EditableEvent::MouseDown(e.data, 0));
        };

        let onkeydown = move |e: Event<KeyboardData>| {
            editable.process_event(&EditableEvent::KeyDown(e.data));
        };

        let onclick = move |_: MouseEvent| {
            editable.process_event(&EditableEvent::Click);
        };

        rsx!(
            rect {
                width: "100%",
                height: "100%",
                background: "white",
                cursor_reference: cursor_attr,
                onmousedown,
                onclick,
                paragraph {
                    height: "50%",
                    width: "100%",
                    cursor_id: "0",
                    cursor_index: "{cursor_pos}",
                    cursor_color: "black",
                    cursor_mode: "editable",
                    onkeydown,
                    highlights,
                    text {
                        color: "black",
                        "{editor}"
                    }
                }
                label {
                    color: "black",
                    height: "50%",
                    "{editor.cursor_row()}:{editor.cursor_col()}"
                }
            }
        )
    }

    let mut utils = launch_test(replace_text_app);

    // Initial state
    let root = utils.root().get(0);
    let cursor = root.get(1).get(0);
    let content = root.get(0).get(0).get(0);
    assert_eq!(cursor.text(), Some("0:0"));
    assert_eq!(content.text(), Some("Hello Rustaceans\nHello Rustaceans"));

    // Move cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: (35.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;
    utils.wait_for_update().await;

    // Cursor has been moved
    let root = utils.root().get(0);
    let cursor = root.get(1).get(0);
    assert_eq!(cursor.text(), Some("0:5"));

    // Click cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: (35.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (35.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;

    // Press shift
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        key: Key::Shift,
        code: Code::ShiftLeft,
        modifiers: Modifiers::default(),
    });
    utils.wait_for_update().await;

    // Move cursor
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: (80.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (80.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;

    // Insert text
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        key: Key::Character("🦀".to_string()),
        code: Code::Unidentified,
        modifiers: Modifiers::empty(),
    });

    utils.wait_for_update().await;
    utils.wait_for_update().await;

    // Text and cursor have changed
    let cursor = root.get(1).get(0);
    let content = root.get(0).get(0).get(0);
    #[cfg(not(target_os = "macos"))]
    {
        assert_eq!(content.text(), Some("Hello🦀ceans\nHello Rustaceans"));
        assert_eq!(cursor.text(), Some("0:6"));
    }

    #[cfg(target_os = "macos")]
    {
        assert_eq!(content.text(), Some("Hello🦀aceans\nHello Rustaceans"));
        assert_eq!(cursor.text(), Some("0:6"));
    }
}
