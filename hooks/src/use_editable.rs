use std::sync::{Arc, Mutex};

use dioxus::{core::UiEvent, events::MouseData, prelude::*};
use freya_elements::events::{KeyCode, KeyboardData};
use freya_node_state::CursorReference;
use tokio::sync::{mpsc::unbounded_channel, mpsc::UnboundedSender};
use xi_rope::Rope;

/// How the editable content must behave.
pub enum EditableMode {
    /// Multiple editors of only one line.
    /// 
    /// Useful for textarea-like editors that need more customization than a simple paragraph for example.
    SingleLineMultipleEditor,
    /// One editor of multiple lines.
    /// 
    /// A paragraph for example.
    MultipleLinesSingleEditors,
}

/// Create a cursor for some editable text.
pub fn use_editable<'a>(
    cx: &ScopeState,
    initializer: impl Fn() -> &'a str,
    mode: EditableMode,
) -> (
    &UseState<Rope>,
    &UseState<(usize, usize)>,
    impl Fn(UiEvent<KeyboardData>) + '_,
    UnboundedSender<(UiEvent<MouseData>, usize)>,
    &UseRef<CursorReference>,
) {
    // Hold the actual editable content
    let content = use_state(cx, || Rope::from(initializer()));
    let content_getter = content.current();

    // Holds the column and line where the cursor is
    let cursor = use_state(cx, || (0, 0));
    let cursor_getter = cursor.current();
    let cursor_setter = cursor.setter();

    let cursor_channels = use_ref(&cx, || {
        let (tx, rx) = unbounded_channel::<(usize, usize)>();
        (tx, Some(rx))
    });

    // Cursor reference passed to the layout engine
    let cursor_ref = use_ref(&cx, || CursorReference {
        agent: cursor_channels.read().0.clone(),
        positions: Arc::new(Mutex::new(None)),
        id: Arc::new(Mutex::new(None)),
    });

    // Single listener multiple triggers channel so the mouse can be changed from multiple elements
    let click_channel = use_ref(&cx, || {
        let (tx, rx) = unbounded_channel::<(UiEvent<MouseData>, usize)>();
        (tx, Some(rx))
    });

    // Update the new positions and ID from the cursor reference so the layout engine can make the proper calculations
    {
        let click_channel = click_channel.clone();
        let cursor_ref = cursor_ref.clone();
        use_effect(&cx, (), move |_| {
            let click_channel = click_channel.clone();
            async move {
                let rx = click_channel.write().1.take();
                let mut rx = rx.unwrap();

                loop {
                    if let Some((e, id)) = rx.recv().await {
                        let points = e.element_coordinates();
                        let cursor_ref = cursor_ref.clone();
                        cursor_ref.write().id.lock().unwrap().replace(id);
                        cursor_ref
                            .write()
                            .positions
                            .lock()
                            .unwrap()
                            .replace((points.x as f32, points.y as f32));
                    }
                }
            }
        });
    }

    // Listen for new calculations from the layout engine
    use_effect(&cx, (), move |_| {
        let cursor_ref = cursor_ref.clone();
        let getter = cursor_getter.clone();
        let cursor_channels = cursor_channels.clone();

        async move {
            let cursor_receiver = cursor_channels.write().1.take();
            let mut cursor_receiver = cursor_receiver.unwrap();
            let mut prev_cursor = (*getter).clone();
            let cursor_ref = cursor_ref.clone();

            loop {
                if let Some((new_index, editor_num)) = cursor_receiver.recv().await {
                    let row = content_getter.line_of_offset(new_index);
                    let col = new_index - row;

                    let new_cursor = match mode {
                        EditableMode::MultipleLinesSingleEditors => (col, row),
                        EditableMode::SingleLineMultipleEditor => (col, editor_num),
                    };

                    // Only update if it's actually different
                    if prev_cursor != new_cursor {
                        cursor_setter(new_cursor.clone());
                        prev_cursor = new_cursor;
                    }

                    // Remove the current calcutions so the layout engine doesn't try to calculate again
                    cursor_ref.write().positions.lock().unwrap().take();
                }
            }
        }
    });

    let process_keyevent = move |e: UiEvent<KeyboardData>| match &e.code {
        KeyCode::ArrowDown => {
            let total_lines = content.lines(..).count() - 1;
            if cursor.1 < total_lines {
                let next_line = content.get().lines(..).nth(cursor.1 + 1).unwrap();
                let cursor_indexolumn = if cursor.0 <= next_line.len() {
                    cursor.0
                } else {
                    next_line.len()
                };
                cursor.set((cursor_indexolumn, cursor.1 + 1))
            }
        }
        KeyCode::ArrowLeft => {
            if cursor.0 > 0 {
                cursor.set((cursor.0 - 1, cursor.1));
            } else {
                let prev_line = content.get().lines(..).nth(cursor.1 - 1);
                if let Some(prev_line) = prev_line {
                    if cursor.0 == 0 && cursor.1 > 0 {
                        let len = if prev_line.len() > 0 {
                            prev_line.len() - 1
                        } else {
                            0
                        };
                        cursor.set((len, cursor.1 - 1));
                    } else if cursor.0 > 0 {
                        cursor.set((cursor.0 - 1, cursor.1));
                    }
                }
            }
        }
        KeyCode::ArrowRight => {
            let total_lines = content.lines(..).count() - 1;
            let current_line = content.get().lines(..).nth(cursor.1).unwrap();
            if cursor.0 < current_line.len() {
                cursor.set((cursor.0 + 1, cursor.1));
            } else if cursor.0 == current_line.len() && cursor.1 < total_lines {
                cursor.set((0, cursor.1 + 1));
            }
        }
        KeyCode::ArrowUp => {
            if cursor.1 > 0 {
                let prev_line = content.get().lines(..).nth(cursor.1 - 1).unwrap();
                let cursor_indexolumn = if cursor.0 <= prev_line.len() {
                    cursor.0
                } else {
                    prev_line.len()
                };
                cursor.set((cursor_indexolumn, cursor.1 - 1))
            }
        }
        KeyCode::Space => {
            let char_idx = content.get().offset_of_line(cursor.1) + cursor.0;
            content.with_mut(|code| {
                code.edit(char_idx..char_idx, " ");
            });
            cursor.set((cursor.0 + 1, cursor.1))
        }
        KeyCode::Backspace => {
            if cursor.0 > 0 {
                let char_idx = content.get().offset_of_line(cursor.1) + cursor.0;
                content.with_mut(|code| {
                    code.edit(char_idx - 1..char_idx, "");
                });

                cursor.set((cursor.0 - 1, cursor.1))
            } else if cursor.1 > 0 {
                let prev_line = content.get().lines(..).nth(cursor.1 - 1).unwrap();
                let current_line = content.get().lines(..).nth(cursor.1);

                if let Some(current_line) = current_line {
                    let prev_char_idx =
                        content.get().offset_of_line(cursor.1 - 1) + prev_line.len();
                    let char_idx = content.get().offset_of_line(cursor.1) + current_line.len();

                    content.with_mut(|code| {
                        code.edit(prev_char_idx..prev_char_idx, current_line.clone());
                        code.edit(char_idx..char_idx + current_line.len(), "");
                    });
                }

                cursor.set((prev_line.len(), cursor.1 - 1));
            }
        }
        KeyCode::Enter => {
            let total_lines = content.lines(..).count();
            let char_idx = content.get().offset_of_line(cursor.1) + cursor.0;
            let current_line = content.get().lines(..).nth(cursor.1).unwrap();
            content.with_mut(|code| {
                let break_line = if cursor.1 == total_lines - 1 && current_line.len() > 0 {
                    "\n\n"
                } else {
                    "\n"
                };
                code.edit(char_idx..char_idx, break_line);
            });

            cursor.set((0, cursor.1 + 1))
        }
        character => {
            if let Some(character) = character.to_text() {
                let char_idx = content.get().offset_of_line(cursor.1) + cursor.0;

                content.with_mut(|code| {
                    code.edit(char_idx..char_idx, character);
                });

                cursor.set((cursor.0 + 1, cursor.1))
            }
        }
    };

    (
        content,
        cursor,
        process_keyevent,
        click_channel.read().0.clone(),
        cursor_ref,
    )
}
