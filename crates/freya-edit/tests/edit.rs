use std::collections::HashMap;

use freya::{
    helpers::*,
    prelude::*,
};
use freya_edit::*;
use freya_testing::prelude::*;

#[test]
fn multiple_lines_single_editor() {
    fn app() -> impl IntoElement {
        let mut editable = use_editable(
            || "Hello Rustaceans\nHello Rustaceans".to_string(),
            EditableConfig::new,
        );
        let holder = use_state(ParagraphHolder::default);
        let editor = editable.editor().read();
        let cursor_pos = editor.cursor_pos();

        let on_mouse_down = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Down {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        };
        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        rect()
            .font_family("NotoSans")
            .width(Size::fill())
            .height(Size::fill())
            .background((255, 255, 255))
            .on_mouse_down(on_mouse_down)
            .child(
                paragraph()
                    .holder(holder.read().clone())
                    .height(Size::percent(50.0))
                    .width(Size::fill())
                    .cursor_index(0)
                    .cursor_index(cursor_pos)
                    .cursor_color((0, 0, 0))
                    // .cursor_mode(CursorMode::Editable)
                    .on_global_key_down(on_global_key_down)
                    .span(Span::new(editor.to_string())),
            )
            .child(
                label()
                    .color((0, 0, 0))
                    .height(Size::percent(50.0))
                    .text(format!("{}:{}", editor.cursor_row(), editor.cursor_col())),
            )
    }
    let mut utils = launch_test(app);
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);

    let cursor = utils.find(|_, element| Some(Label::try_downcast(element)?.text.to_string()));
    let content = utils.find(|_, element| Some(Paragraph::try_downcast(element)?.to_string()));
    assert_eq!(cursor.as_deref(), Some("0:0"));
    assert_eq!(
        content.as_deref(),
        Some("Hello Rustaceans\nHello Rustaceans")
    );

    utils.click_cursor((35.0, 3.0));

    let cursor = utils.find(|_, element| Some(Label::try_downcast(element)?.text.to_string()));
    assert_eq!(cursor.as_deref(), Some("0:5"));

    utils.write_text("!");

    let cursor = utils.find(|_, element| Some(Label::try_downcast(element)?.text.to_string()));
    let content = utils.find(|_, element| Some(Paragraph::try_downcast(element)?.to_string()));
    assert_eq!(
        content.as_deref(),
        Some("Hello! Rustaceans\nHello Rustaceans")
    );
    assert_eq!(cursor.as_deref(), Some("0:6"));

    utils.click_cursor((3.0, 3.0));

    let cursor = utils.find(|_, element| Some(Label::try_downcast(element)?.text.to_string()));
    assert_eq!(cursor.as_deref(), Some("0:0"));

    utils.press_key(Key::Named(NamedKey::ArrowDown));

    let cursor = utils.find(|_, element| Some(Label::try_downcast(element)?.text.to_string()));
    assert_eq!(cursor.as_deref(), Some("1:0"));

    utils.press_key(Key::Named(NamedKey::ArrowRight));

    let cursor = utils.find(|_, element| Some(Label::try_downcast(element)?.text.to_string()));
    assert_eq!(cursor.as_deref(), Some("1:1"));

    utils.press_key(Key::Named(NamedKey::ArrowUp));

    let cursor = utils.find(|_, element| Some(Label::try_downcast(element)?.text.to_string()));
    assert_eq!(cursor.as_deref(), Some("0:1"));

    utils.press_key(Key::Named(NamedKey::ArrowUp));

    let cursor = utils.find(|_, element| Some(Label::try_downcast(element)?.text.to_string()));
    assert_eq!(cursor.as_deref(), Some("0:0"));

    utils.press_key(Key::Named(NamedKey::ArrowDown));
    utils.press_key(Key::Named(NamedKey::ArrowDown));

    let cursor = utils.find(|_, element| Some(Label::try_downcast(element)?.text.to_string()));
    assert_eq!(cursor.as_deref(), Some("1:16"));

    utils.press_key(Key::Named(NamedKey::ArrowUp));
    utils.press_key(Key::Named(NamedKey::ArrowUp));

    let cursor = utils.find(|_, element| Some(Label::try_downcast(element)?.text.to_string()));
    assert_eq!(cursor.as_deref(), Some("0:0"));
}

#[test]
fn single_line_multiple_editors() {
    fn app() -> impl IntoElement {
        let mut editable = use_editable(
            || "Hello Rustaceans\nHello World".to_string(),
            EditableConfig::new,
        );

        let editor = editable.editor().read();

        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .background((255, 255, 255))
            .on_global_key_down(on_global_key_down)
            .children_iter(editor.lines().map(|line| {
                let holder = use_state(ParagraphHolder::default);
                let cursor_col = editor.cursor_col();
                let line = line.text.to_string();
                from_fn((), line, move |line| {
                    let on_mouse_down = move |e: Event<MouseEventData>| {
                        editable.process_event(EditableEvent::Down {
                            location: e.element_location,
                            editor_line: EditorLine::SingleParagraph,
                            holder: &holder.read(),
                        });
                    };
                    paragraph()
                        .holder(holder.read().clone())
                        .width(Size::fill())
                        .height(Size::px(30.0))
                        .max_lines(1)
                        .cursor_index(cursor_col)
                        .cursor_color((0, 0, 0))
                        .on_mouse_down(on_mouse_down)
                        .span(Span::new(line.to_string()))
                        .into()
                })
            }))
            .child(
                label()
                    .color((0, 0, 0))
                    .height(Size::percent(50.0))
                    .text(format!("{}:{}", editor.cursor_row(), editor.cursor_col())),
            )
    }

    let mut utils = launch_test(app);
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);

    let cursor = utils.find(|_, e| Some(Label::try_downcast(e)?.text.to_string()));
    let content = utils.find(|_, e| Some(Paragraph::try_downcast(e)?.to_string()));

    assert_eq!(cursor.as_deref(), Some("0:0"));
    assert_eq!(content.as_deref(), Some("Hello Rustaceans\n"));

    utils.click_cursor((35.0, 3.0));

    let cursor = utils.find(|_, e| Some(Label::try_downcast(e)?.text.to_string()));
    assert_eq!(cursor.as_deref(), Some("0:5"));

    utils.write_text("!");
    utils.sync_and_update();

    let cursor = utils.find(|_, e| Some(Label::try_downcast(e)?.text.to_string()));
    let content = utils.find(|_, e| Some(Paragraph::try_downcast(e)?.to_string()));
    assert_eq!(content.as_deref(), Some("Hello! Rustaceans\n"));
    assert_eq!(cursor.as_deref(), Some("0:6"));
}

#[test]
fn highlight_multiple_lines_single_editor() {
    let mut utils = launch_test(|| {
        let mut editable = use_editable(
            || "Hello Rustaceans\nHello Rustaceans".to_string(),
            EditableConfig::new,
        );
        let holder = use_state(ParagraphHolder::default);
        let editor = editable.editor().read();
        let cursor_pos = editor.cursor_pos();

        let on_mouse_down = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Down {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        };
        let on_mouse_move = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Move {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        };
        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .background((255, 255, 255))
            .child(
                paragraph()
                    .holder(holder.read().clone())
                    .highlights(
                        editor
                            .get_visible_selection(EditorLine::SingleParagraph)
                            .map(|h| vec![h]),
                    )
                    .width(Size::fill())
                    .height(Size::percent(50.0))
                    .cursor_index(cursor_pos)
                    .cursor_color((0, 0, 0))
                    .on_mouse_down(on_mouse_down)
                    .on_mouse_move(on_mouse_move)
                    .on_global_key_down(on_global_key_down)
                    .span(Span::new(editor.to_string())),
            )
    });
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);
    utils.send_event(PlatformEvent::Mouse {
        name: MouseEventName::MouseDown,
        cursor: (35.0, 3.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.sync_and_update();
    utils.move_cursor((80.0, 25.0));
    utils.send_event(PlatformEvent::Mouse {
        name: MouseEventName::MouseUp,
        cursor: (80.0, 25.0).into(),
        button: Some(MouseButton::Left),
    });
    utils.sync_and_update();

    let highlights = utils.find(|_, e| Some(Paragraph::try_downcast(e)?.highlights.clone()));
    assert_eq!(highlights, Some(vec![(5, 27)]));
}

#[test]
fn highlights_single_line_multiple_editors() {
    let mut utils = launch_test(|| {
        let mut editable = use_editable(
            || "Hello Rustaceans\nHello Rustaceans".to_string(),
            EditableConfig::new,
        );

        let editor = editable.editor().read();

        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .background((255, 255, 255))
            .on_global_key_down(on_global_key_down)
            .children_iter(editor.lines().enumerate().map(move |(i, line)| {
                let line = line.to_string();
                from_fn((), line, move |line| {
                    let holder = use_state(ParagraphHolder::default);

                    let editor = editable.editor().read();
                    let is_selected = editable.editor().read().cursor_row() == i;
                    let cursor_index = if is_selected {
                        Some(editable.editor().read().cursor_col())
                    } else {
                        None
                    };

                    let on_mouse_move = move |e: Event<MouseEventData>| {
                        editable.process_event(EditableEvent::Move {
                            location: e.element_location,
                            editor_line: EditorLine::Paragraph(i),
                            holder: &holder.read(),
                        });
                    };
                    let on_mouse_down = move |e: Event<MouseEventData>| {
                        editable.process_event(EditableEvent::Down {
                            location: e.element_location,
                            editor_line: EditorLine::Paragraph(i),
                            holder: &holder.read(),
                        });
                    };

                    paragraph()
                        .holder(holder.read().clone())
                        .width(Size::fill())
                        .height(Size::px(30.0))
                        .max_lines(1)
                        .cursor_index(cursor_index)
                        .cursor_color((0, 0, 0))
                        .on_mouse_down(on_mouse_down)
                        .on_mouse_move(on_mouse_move)
                        .highlights(
                            editor
                                .get_visible_selection(EditorLine::Paragraph(i))
                                .map(|h| vec![h]),
                        )
                        .span(Span::new(line.clone()))
                        .into()
                })
            }))
            .child(
                label()
                    .color((0, 0, 0))
                    .height(Size::percent(50.0))
                    .text(format!("{}:{}", editor.cursor_row(), editor.cursor_col())),
            )
    });
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);

    utils.press_cursor((35.0, 3.0));
    utils.sync_and_update();
    utils.move_cursor((35.0, 3.0));
    utils.move_cursor((80.0, 35.0));
    utils.release_cursor((80.0, 35.0));
    utils.sync_and_update();

    let highlights = utils.find_many(|_, e| {
        let para = Paragraph::try_downcast(e)?;
        Some(para.highlights.clone())
    });

    assert_eq!(highlights[0], vec![(5, 17)]);

    assert_eq!(highlights[1], vec![(0, 10)]);
}

#[test]
fn special_text_editing() {
    let mut utils = launch_test(|| {
        let mut editable = use_editable(|| "你好世界\n👋".to_string(), EditableConfig::new);
        let holder = use_state(ParagraphHolder::default);
        let editor = editable.editor().read();
        let cursor_pos = editor.cursor_pos();

        let on_mouse_down = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Down {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        };
        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .background((255, 255, 255))
            .on_mouse_down(on_mouse_down)
            .child(
                paragraph()
                    .holder(holder.read().clone())
                    .width(Size::fill())
                    .height(Size::percent(50.0))
                    .cursor_index(cursor_pos)
                    .cursor_color((0, 0, 0))
                    .on_global_key_down(on_global_key_down)
                    .span(Span::new(editor.to_string())),
            )
            .child(label().color((0, 0, 0)).text(format!(
                "{}:{}",
                editor.cursor_row(),
                editor.cursor_col()
            )))
    });
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);

    utils.click_cursor((35.0, 3.0));
    utils.write_text("🦀");

    let content = utils.find(|_, e| Some(Paragraph::try_downcast(e)?.to_string()));
    assert!(content.unwrap().contains("🦀"));
}

#[test]
fn backspace_remove() {
    let mut utils = launch_test(|| {
        let mut editable = use_editable(
            || "Hello Rustaceans\nHello Rustaceans".to_string(),
            EditableConfig::new,
        );
        let holder = use_state(ParagraphHolder::default);
        let editor = editable.editor().read();
        let cursor_pos = editor.cursor_pos();

        let on_mouse_down = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Down {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        };
        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .on_mouse_down(on_mouse_down)
            .background((255, 255, 255))
            .child(
                paragraph()
                    .holder(holder.read().clone())
                    .width(Size::fill())
                    .height(Size::percent(50.0))
                    .cursor_index(cursor_pos)
                    .cursor_color((0, 0, 0))
                    .on_global_key_down(on_global_key_down)
                    .span(Span::new(editor.to_string())),
            )
    });
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);

    utils.click_cursor((35.0, 3.0));
    utils.write_text("🦀");

    let content = utils.find(|_, e| Some(Paragraph::try_downcast(e)?.to_string()));
    assert!(content.unwrap().contains("🦀"));

    utils.press_key(Key::Named(NamedKey::Backspace));

    let content = utils.find(|_, e| Some(Paragraph::try_downcast(e)?.to_string()));
    assert_eq!(
        content.as_deref(),
        Some("Hello Rustaceans\nHello Rustaceans")
    );
}

#[test]
fn highlight_shift_click_multiple_lines_single_editor() {
    let mut utils = launch_test(|| {
        let mut editable = use_editable(
            || "Hello Rustaceans\nHello Rustaceans".to_string(),
            EditableConfig::new,
        );

        let holder = use_state(ParagraphHolder::default);
        let editor = editable.editor().read();
        let cursor_pos = editor.cursor_pos();

        let on_mouse_down = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Down {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        };

        let on_mouse_move = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Move {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        };

        let on_mouse_up = move |_e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Release);
        };

        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .background((255, 255, 255))
            .child(
                paragraph()
                    .holder(holder.read().clone())
                    .width(Size::fill())
                    .height(Size::percent(50.0))
                    .cursor_index(cursor_pos)
                    .cursor_color((0, 0, 0))
                    .highlights(
                        editor
                            .get_visible_selection(EditorLine::SingleParagraph)
                            .map(|h| vec![h]),
                    )
                    .on_mouse_down(on_mouse_down)
                    .on_mouse_move(on_mouse_move)
                    .on_global_key_down(on_global_key_down)
                    .on_mouse_up(on_mouse_up)
                    .span(Span::new(editor.to_string())),
            )
            .child(
                label()
                    .color((0, 0, 0))
                    .height(Size::percent(50.0))
                    .text(format!("{}:{}", editor.cursor_row(), editor.cursor_col())),
            )
    });
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);

    utils.press_cursor((35.0, 3.0));
    utils.move_cursor((35.0, 3.0));

    utils.press_key(Key::Named(NamedKey::Shift));

    utils.move_cursor((80.0, 40.0));
    utils.release_cursor((80.0, 40.0));

    let highlights = utils.find(|_, e| {
        let paragraph = Paragraph::try_downcast(e)?;
        Some(paragraph.highlights.clone())
    });

    assert_eq!(highlights, Some(vec![(5, 27)]));
}

#[test]
fn highlights_shift_click_single_line_multiple_editors() {
    let mut utils = launch_test(|| {
        let mut editable = use_editable(
            || "Hello Rustaceans\nHello Rustaceans".to_string(),
            EditableConfig::new,
        );
        let editor = editable.editor().read();

        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .background((255, 255, 255))
            .on_global_key_down(on_global_key_down)
            .children(
                editor
                    .lines()
                    .enumerate()
                    .map(|(i, line)| {
                        let line = line.text.to_string();
                        from_fn((), line, move |line| {
                            let holder = use_state(ParagraphHolder::default);
                            let highlights = editable
                                .editor()
                                .read()
                                .get_visible_selection(EditorLine::Paragraph(i));

                            let is_line_selected = editable.editor().read().cursor_row() == i;
                            let character_index = if is_line_selected {
                                Some(editable.editor().read().cursor_col())
                            } else {
                                None
                            };

                            let on_mouse_down = move |e: Event<MouseEventData>| {
                                editable.process_event(EditableEvent::Down {
                                    location: e.element_location,
                                    editor_line: EditorLine::Paragraph(i),
                                    holder: &holder.read(),
                                });
                            };

                            let on_mouse_move = move |e: Event<MouseEventData>| {
                                editable.process_event(EditableEvent::Move {
                                    location: e.element_location,
                                    editor_line: EditorLine::Paragraph(i),
                                    holder: &holder.read(),
                                });
                            };

                            let on_mouse_up = move |_e: Event<MouseEventData>| {
                                editable.process_event(EditableEvent::Release);
                            };

                            paragraph()
                                .holder(holder.read().clone())
                                .width(Size::fill())
                                .height(Size::px(30.))
                                .max_lines(1)
                                .cursor_index(character_index)
                                .cursor_color((0, 0, 0))
                                .highlights(highlights.map(|h| vec![h]))
                                .on_mouse_down(on_mouse_down)
                                .on_mouse_move(on_mouse_move)
                                .on_mouse_up(on_mouse_up)
                                .span(Span::new(line.to_string()))
                                .into()
                        })
                    })
                    .collect::<Vec<_>>(),
            )
            .child(
                label()
                    .color((0, 0, 0))
                    .height(Size::percent(50.0))
                    .text(format!("{}:{}", editor.cursor_row(), editor.cursor_col())),
            )
    });
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);

    utils.press_cursor((35.0, 3.0));
    utils.move_cursor((35.0, 3.0));

    utils.press_key(Key::Named(NamedKey::Shift));

    utils.move_cursor((80.0, 35.0));
    utils.release_cursor((80.0, 35.0));

    let highlights = utils.find_many(|_, e| {
        let paragraph = Paragraph::try_downcast(e)?;
        Some(paragraph.highlights.clone())
    });

    assert_eq!(highlights[0], vec![(5, 17)]);

    assert_eq!(highlights[1], vec![(0, 10)]);
}

#[test]
fn double_click_select_word() {
    let mut utils = launch_test(|| {
        let mut editable = use_editable(
            || "Hello Rustaceans\nHello World".to_string(),
            EditableConfig::new,
        );
        let holder = use_state(ParagraphHolder::default);
        let editor = editable.editor().read();
        let cursor_pos = editor.cursor_pos();

        let on_mouse_down = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Down {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        };

        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        rect()
            .font_family("NotoSans")
            .width(Size::fill())
            .height(Size::fill())
            .background((255, 255, 255))
            .on_mouse_down(on_mouse_down)
            .child(
                paragraph()
                    .holder(holder.read().clone())
                    .height(Size::percent(50.0))
                    .width(Size::fill())
                    .cursor_index(cursor_pos)
                    .cursor_color((0, 0, 0))
                    .on_global_key_down(on_global_key_down)
                    .highlights(
                        editor
                            .get_visible_selection(EditorLine::SingleParagraph)
                            .map(|h| vec![h]),
                    )
                    .span(Span::new(editor.to_string())),
            )
            .child(
                label()
                    .color((0, 0, 0))
                    .height(Size::percent(50.0))
                    .text(format!("{}:{}", editor.cursor_row(), editor.cursor_col())),
            )
    });
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);

    // Double click on "Rustaceans" (around position 50, 3)
    utils.click_cursor((50.0, 3.0));
    utils.click_cursor((50.0, 3.0));

    let highlights = utils.find(|_, e| Some(Paragraph::try_downcast(e)?.highlights.clone()));

    // Should select "Rustaceans" (positions 6-16)
    assert_eq!(highlights, Some(vec![(6, 16)]));
}

#[test]
fn triple_click_select_line() {
    let mut utils = launch_test(|| {
        let mut editable = use_editable(
            || "Hello Rustaceans\nHello World".to_string(),
            EditableConfig::new,
        );
        let holder = use_state(ParagraphHolder::default);
        let editor = editable.editor().read();
        let cursor_pos = editor.cursor_pos();

        let on_mouse_down = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Down {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        };

        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        rect()
            .font_family("NotoSans")
            .width(Size::fill())
            .height(Size::fill())
            .background((255, 255, 255))
            .on_mouse_down(on_mouse_down)
            .child(
                paragraph()
                    .holder(holder.read().clone())
                    .height(Size::percent(50.0))
                    .width(Size::fill())
                    .cursor_index(cursor_pos)
                    .cursor_color((0, 0, 0))
                    .on_global_key_down(on_global_key_down)
                    .highlights(
                        editor
                            .get_visible_selection(EditorLine::SingleParagraph)
                            .map(|h| vec![h]),
                    )
                    .span(Span::new(editor.to_string())),
            )
            .child(
                label()
                    .color((0, 0, 0))
                    .height(Size::percent(50.0))
                    .text(format!("{}:{}", editor.cursor_row(), editor.cursor_col())),
            )
    });
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);

    // Triple click on first line
    utils.click_cursor((50.0, 3.0));
    utils.click_cursor((50.0, 3.0));
    utils.click_cursor((50.0, 3.0));

    let highlights = utils.find(|_, e| Some(Paragraph::try_downcast(e)?.highlights.clone()));

    // Should select entire first line "Hello Rustaceans\n" (positions 0-17)
    assert_eq!(highlights, Some(vec![(0, 17)]));
}

#[test]
fn double_click_select_word_single_line_multiple_editors() {
    let mut utils = launch_test(|| {
        let mut editable = use_editable(
            || "Hello Rustaceans\nHello World".to_string(),
            EditableConfig::new,
        );

        let editor = editable.editor().read();

        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        rect()
            .font_family("NotoSans")
            .width(Size::fill())
            .height(Size::fill())
            .background((255, 255, 255))
            .on_global_key_down(on_global_key_down)
            .children_iter(editor.lines().enumerate().map(move |(i, line)| {
                let line = line.text.to_string();
                from_fn((), line, move |line| {
                    let holder = use_state(ParagraphHolder::default);
                    let editor = editable.editor().read();

                    let is_selected = editor.cursor_row() == i;
                    let cursor_index = if is_selected {
                        Some(editor.cursor_col())
                    } else {
                        None
                    };

                    let on_mouse_down = move |e: Event<MouseEventData>| {
                        editable.process_event(EditableEvent::Down {
                            location: e.element_location,
                            editor_line: EditorLine::Paragraph(i),
                            holder: &holder.read(),
                        });
                    };

                    paragraph()
                        .holder(holder.read().clone())
                        .width(Size::fill())
                        .height(Size::px(30.0))
                        .max_lines(1)
                        .cursor_index(cursor_index)
                        .cursor_color((0, 0, 0))
                        .on_mouse_down(on_mouse_down)
                        .highlights(
                            editor
                                .get_visible_selection(EditorLine::Paragraph(i))
                                .map(|h| vec![h]),
                        )
                        .span(Span::new(line.clone()))
                        .into()
                })
            }))
            .child(
                label()
                    .color((0, 0, 0))
                    .height(Size::percent(50.0))
                    .text(format!("{}:{}", editor.cursor_row(), editor.cursor_col())),
            )
    });
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);

    // Double click on "World" in second line
    utils.click_cursor((50.0, 35.0));
    utils.click_cursor((50.0, 35.0));

    let highlights = utils.find_many(|_, e| {
        let para = Paragraph::try_downcast(e)?;
        Some(para.highlights.clone())
    });

    // Second line should have "World" selected (positions 6-11)
    assert_eq!(highlights[1], vec![(6, 11)]);
}

#[test]
fn triple_click_select_line_single_line_multiple_editors() {
    let mut utils = launch_test(|| {
        let mut editable = use_editable(
            || "Hello Rustaceans\nHello World".to_string(),
            EditableConfig::new,
        );

        let editor = editable.editor().read();

        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        rect()
            .font_family("NotoSans")
            .width(Size::fill())
            .height(Size::fill())
            .background((255, 255, 255))
            .on_global_key_down(on_global_key_down)
            .children_iter(editor.lines().enumerate().map(move |(i, line)| {
                let line = line.text.to_string();
                from_fn((), line, move |line| {
                    let holder = use_state(ParagraphHolder::default);
                    let editor = editable.editor().read();

                    let is_selected = editor.cursor_row() == i;
                    let cursor_index = if is_selected {
                        Some(editor.cursor_col())
                    } else {
                        None
                    };

                    let on_mouse_down = move |e: Event<MouseEventData>| {
                        editable.process_event(EditableEvent::Down {
                            location: e.element_location,
                            editor_line: EditorLine::Paragraph(i),
                            holder: &holder.read(),
                        });
                    };

                    paragraph()
                        .holder(holder.read().clone())
                        .width(Size::fill())
                        .height(Size::px(30.0))
                        .max_lines(1)
                        .cursor_index(cursor_index)
                        .cursor_color((0, 0, 0))
                        .on_mouse_down(on_mouse_down)
                        .highlights(
                            editor
                                .get_visible_selection(EditorLine::Paragraph(i))
                                .map(|h| vec![h]),
                        )
                        .span(Span::new(line.clone()))
                        .into()
                })
            }))
            .child(
                label()
                    .color((0, 0, 0))
                    .height(Size::percent(50.0))
                    .text(format!("{}:{}", editor.cursor_row(), editor.cursor_col())),
            )
    });
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);

    // Triple click on second line
    utils.click_cursor((50.0, 35.0));
    utils.click_cursor((50.0, 35.0));
    utils.click_cursor((50.0, 35.0));

    let highlights = utils.find_many(|_, e| {
        let para = Paragraph::try_downcast(e)?;
        Some(para.highlights.clone())
    });

    // Second line should be fully selected "Hello World" (positions 0-11)
    assert_eq!(highlights[1], vec![(0, 11)]);
}

#[test]
fn highlight_all_text() {
    let mut utils = launch_test(|| {
        let mut editable = use_editable(
            || "Hello Rustaceans\nHello Rustaceans".to_string(),
            EditableConfig::new,
        );

        let holder = use_state(ParagraphHolder::default);
        let editor = editable.editor().read();
        let cursor_pos = editor.cursor_pos();

        let on_mouse_down = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Down {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        };

        let on_mouse_move = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Move {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        };

        let on_mouse_up = move |_e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Release);
        };

        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .background((255, 255, 255))
            .child(
                paragraph()
                    .holder(holder.read().clone())
                    .width(Size::fill())
                    .height(Size::percent(50.0))
                    .cursor_index(cursor_pos)
                    .cursor_color((0, 0, 0))
                    .highlights(
                        editor
                            .get_visible_selection(EditorLine::SingleParagraph)
                            .map(|h| vec![h]),
                    )
                    .on_mouse_down(on_mouse_down)
                    .on_mouse_move(on_mouse_move)
                    .on_global_key_down(on_global_key_down)
                    .on_mouse_up(on_mouse_up)
                    .span(Span::new(editor.to_string())),
            )
            .child(
                label()
                    .color((0, 0, 0))
                    .height(Size::percent(50.0))
                    .text(format!("{}:{}", editor.cursor_row(), editor.cursor_col())),
            )
    });
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);

    utils.send_event(PlatformEvent::Keyboard {
        name: KeyboardEventName::KeyDown,
        key: Key::Character("a".to_string()),
        code: Code::KeyA,
        modifiers: if cfg!(target_os = "macos") {
            Modifiers::META
        } else {
            Modifiers::CONTROL
        },
    });

    utils.sync_and_update();

    let highlights = utils.find(|_, e| Some(Paragraph::try_downcast(e)?.highlights.clone()));

    let expected = Some(vec![(0, 33)]);
    assert_eq!(highlights, expected);
}

#[test]
fn replace_text() {
    let mut utils = launch_test(|| {
        let mut editable = use_editable(
            || "Hello Rustaceans\nHello Rustaceans".to_string(),
            EditableConfig::new,
        );

        let holder = use_state(ParagraphHolder::default);
        let editor = editable.editor().read();
        let cursor_pos = editor.cursor_pos();

        let on_mouse_down = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Down {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        };

        let on_mouse_move = move |e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Move {
                location: e.element_location,
                editor_line: EditorLine::SingleParagraph,
                holder: &holder.read(),
            });
        };

        let on_mouse_up = move |_e: Event<MouseEventData>| {
            editable.process_event(EditableEvent::Release);
        };

        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .background((255, 255, 255))
            .on_mouse_move(on_mouse_move)
            .on_mouse_down(on_mouse_down)
            .on_mouse_up(on_mouse_up)
            .child(
                paragraph()
                    .holder(holder.read().clone())
                    .width(Size::fill())
                    .height(Size::percent(50.0))
                    .cursor_index(cursor_pos)
                    .cursor_color((0, 0, 0))
                    .on_global_key_down(on_global_key_down)
                    .highlights(
                        editor
                            .get_visible_selection(EditorLine::SingleParagraph)
                            .map(|h| vec![h]),
                    )
                    .span(Span::new(editor.to_string())),
            )
            .child(
                label()
                    .color((0, 0, 0))
                    .height(Size::percent(50.0))
                    .text(format!("{}:{}", editor.cursor_row(), editor.cursor_col())),
            )
    });
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);

    let cursor = utils.find(|_, e| Some(Label::try_downcast(e)?.text.to_string()));
    let content = utils.find(|_, e| Some(Paragraph::try_downcast(e)?.to_string()));
    assert_eq!(cursor.as_deref(), Some("0:0"));
    assert_eq!(
        content.as_deref(),
        Some("Hello Rustaceans\nHello Rustaceans")
    );

    utils.press_cursor((35.0, 3.0));
    utils.move_cursor((35.0, 3.0));

    let cursor = utils.find(|_, e| Some(Label::try_downcast(e)?.text.to_string()));
    assert_eq!(cursor.as_deref(), Some("0:5"));

    utils.press_key(Key::Named(NamedKey::Shift));
    utils.move_cursor((80.0, 3.0));
    utils.release_cursor((80.0, 3.0));

    utils.write_text("🦀");

    let cursor = utils.find(|_, e| Some(Label::try_downcast(e)?.text.to_string()));
    let content = utils.find(|_, e| Some(Paragraph::try_downcast(e)?.to_string()));
    assert_eq!(content.as_deref(), Some("Hello🦀aceans\nHello Rustaceans"));
    assert_eq!(cursor.as_deref(), Some("0:7"));
}

#[test]
fn navigate_empty_lines() {
    let mut utils = launch_test(|| {
        let mut editable = use_editable(|| "".to_string(), EditableConfig::new);
        let holder = use_state(ParagraphHolder::default);
        let editor = editable.editor().read();
        let cursor_pos = editor.cursor_pos();

        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            editable.process_event(EditableEvent::KeyDown {
                key: &e.key,
                modifiers: e.modifiers,
            });
        };

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .background((255, 255, 255))
            .child(
                paragraph()
                    .holder(holder.read().clone())
                    .width(Size::fill())
                    .height(Size::percent(50.0))
                    .cursor_index(cursor_pos)
                    .cursor_color((0, 0, 0))
                    .on_global_key_down(on_global_key_down)
                    .span(Span::new(editor.to_string())),
            )
            .child(label().color((0, 0, 0)).text(format!(
                "{}:{}",
                editor.cursor_row(),
                editor.cursor_col()
            )))
    });
    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("./NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);

    utils.write_text("\n");

    let content = utils.find(|_, e| Some(Paragraph::try_downcast(e)?.to_string()));
    assert_eq!(content.as_deref(), Some("\n"));

    utils.press_key(Key::Named(NamedKey::ArrowUp));
    utils.press_key(Key::Named(NamedKey::ArrowDown));

    let cursor = utils.find(|_, e| Some(Label::try_downcast(e)?.text.to_string()));
    assert!(cursor.is_some());
}
