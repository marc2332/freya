use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use dioxus_core::{AttributeValue, Event, ScopeState};
use dioxus_hooks::{use_effect, use_ref, use_state, UseRef, UseState};
use freya_elements::events_data::{KeyCode, KeyboardData, MouseData};
use freya_node_state::{CursorReference, CustomAttributeValues};
use tokio::sync::{mpsc::unbounded_channel, mpsc::UnboundedSender};
pub use xi_rope::Rope;

/// How the editable content must behave.
pub enum EditableMode {
    /// Multiple editors of only one line.
    ///
    /// Useful for textarea-like editors that need more customization than a simple paragraph for example.
    SingleLineMultipleEditors,
    /// One editor of multiple lines.
    ///
    /// A paragraph for example.
    MultipleLinesSingleEditor,
}

pub type KeypressNotifier = UnboundedSender<Rc<KeyboardData>>;
pub type ClickNotifier = UnboundedSender<(Rc<MouseData>, usize)>;
pub type EditableText = UseState<Rope>;
pub type CursorPosition = UseState<(usize, usize)>;
pub type KeyboardEvent = Event<KeyboardData>;
pub type CursorRef = UseRef<CursorReference>;

/// Create a cursor for some editable text.
pub fn use_editable<'a>(
    cx: &ScopeState,
    initializer: impl Fn() -> &'a str,
    mode: EditableMode,
) -> (
    &EditableText,
    &CursorPosition,
    KeypressNotifier,
    ClickNotifier,
    AttributeValue,
) {
    // Hold the actual editable content
    let content = use_state(cx, || Rope::from(initializer()));

    // Holds the column and line where the cursor is
    let cursor = use_state(cx, || (0, 0));

    let cursor_channels = cx.use_hook(|| {
        let (tx, rx) = unbounded_channel::<(usize, usize)>();
        (tx, Some(rx))
    });

    // Cursor reference passed to the layout engine
    let cursor_ref = use_ref(cx, || CursorReference {
        agent: cursor_channels.0.clone(),
        positions: Arc::new(Mutex::new(None)),
        id: Arc::new(Mutex::new(None)),
    });

    let cursor_ref_attr = cx.any_value(CustomAttributeValues::CursorReference(
        cursor_ref.read().clone(),
    ));

    // Single listener multiple triggers channel so the mouse can be changed from multiple elements
    let click_channel = cx.use_hook(|| {
        let (tx, rx) = unbounded_channel::<(Rc<MouseData>, usize)>();
        (tx, Some(rx))
    });

    // Single listener multiple triggers channel to write from different sources
    let keypress_channel = cx.use_hook(|| {
        let (tx, rx) = unbounded_channel::<Rc<KeyboardData>>();
        (tx, Some(rx))
    });

    let keypress_channel_sender = keypress_channel.0.clone();
    let click_channel_sender = click_channel.0.clone();

    // Update the new positions and ID from the cursor reference so the layout engine can make the proper calculations
    {
        let cursor_ref = cursor_ref.clone();
        use_effect(cx, (), move |_| {
            let rx = click_channel.1.take();
            async move {
                let mut rx = rx.unwrap();

                while let Some((e, id)) = rx.recv().await {
                    let points = e.get_element_coordinates();
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
        });
    }

    // Listen for new calculations from the layout engine
    use_effect(cx, (), move |_| {
        let cursor_ref = cursor_ref.clone();
        let cursor_getter = cursor.current();
        let cursor_receiver = cursor_channels.1.take();
        let content = content.clone();
        let cursor_setter = cursor.setter();

        async move {
            let mut cursor_receiver = cursor_receiver.unwrap();
            let mut prev_cursor = *cursor_getter;
            let cursor_ref = cursor_ref.clone();

            while let Some((new_index, editor_num)) = cursor_receiver.recv().await {
                let content = content.current();

                let new_cursor_row = match mode {
                    EditableMode::MultipleLinesSingleEditor => content.line_of_offset(new_index),
                    EditableMode::SingleLineMultipleEditors => editor_num,
                };

                let new_cursor_col = match mode {
                    EditableMode::MultipleLinesSingleEditor => {
                        new_index - content.offset_of_line(new_cursor_row)
                    }
                    EditableMode::SingleLineMultipleEditors => new_index,
                };

                let new_current_line = content.lines(..).nth(new_cursor_row).unwrap();

                // Use the line lenght as new column if the clicked column surpases the length
                let new_cursor = if new_cursor_col >= new_current_line.len() {
                    (new_current_line.len(), new_cursor_row)
                } else {
                    (new_cursor_col, new_cursor_row)
                };

                // Only update if it's actually different
                if prev_cursor != new_cursor {
                    cursor_setter(new_cursor);
                    prev_cursor = new_cursor;
                }

                // Remove the current calcutions so the layout engine doesn't try to calculate again
                cursor_ref.write().positions.lock().unwrap().take();
            }
        }
    });

    use_effect(cx, (), move |_| {
        let cursor_getter = cursor.to_owned();
        let rx = keypress_channel.1.take();
        let content = content.clone();
        let cursor_setter = cursor.setter();
        async move {
            let mut rx = rx.unwrap();

            while let Some(e) = rx.recv().await {
                let rope = content.current();
                let cursor = cursor_getter.current();
                println!("{:?}", e);

                match &e.code {
                    KeyCode::ArrowDown => {
                        let total_lines = rope.lines(..).count() - 1;
                        // Go one line down
                        if cursor.1 < total_lines {
                            let next_line = rope.lines(..).nth(cursor.1 + 1).unwrap();

                            // Try to use the current cursor column, otherwise use the new line length
                            let cursor_index = if cursor.0 <= next_line.len() {
                                cursor.0
                            } else {
                                next_line.len()
                            };

                            cursor_setter((cursor_index, cursor.1 + 1));
                        }
                    }
                    KeyCode::ArrowLeft => {
                        // Go one character to the left
                        if cursor.0 > 0 {
                            cursor_setter((cursor.0 - 1, cursor.1));
                        } else if cursor.1 > 0 {
                            // Go one line up if there is no more characters on the left
                            let prev_line = rope.lines(..).nth(cursor.1 - 1);
                            if let Some(prev_line) = prev_line {
                                // Use the new line length as new cursor column, otherwise just set it to 0
                                let len = if prev_line.len() > 0 {
                                    prev_line.len()
                                } else {
                                    0
                                };
                                cursor_setter((len, cursor.1 - 1));
                            }
                        }
                    }
                    KeyCode::ArrowRight => {
                        let total_lines = rope.lines(..).count() - 1;
                        let current_line = rope.lines(..).nth(cursor.1).unwrap();

                        // Go one line down if there isn't more characters on the right
                        if cursor.1 < total_lines && cursor.0 == current_line.len() {
                            cursor_setter((0, cursor.1 + 1));
                        } else if cursor.0 < current_line.len() {
                            // Go one character to the right if possible
                            cursor_setter((cursor.0 + 1, cursor.1));
                        }
                    }
                    KeyCode::ArrowUp => {
                        // Go one line up if there is any
                        if cursor.1 > 0 {
                            let prev_line = rope.lines(..).nth(cursor.1 - 1).unwrap();

                            // Try to use the current cursor column, otherwise use the new line length
                            let cursor_column = if cursor.0 <= prev_line.len() {
                                cursor.0
                            } else {
                                prev_line.len()
                            };

                            cursor_setter((cursor_column, cursor.1 - 1));
                        }
                    }
                    KeyCode::Space => {
                        // Simply adds an space
                        let char_idx = rope.offset_of_line(cursor.1) + cursor.0;
                        content.with_mut(|code| {
                            code.edit(char_idx..char_idx, " ");
                        });
                        cursor_setter((cursor.0 + 1, cursor.1));
                    }
                    KeyCode::Backspace => {
                        if cursor.0 > 0 {
                            // Remove the character to the left if there is any
                            let char_idx = rope.offset_of_line(cursor.1) + cursor.0;
                            content.with_mut(|code| {
                                code.edit(char_idx - 1..char_idx, "");
                            });

                            cursor_setter((cursor.0 - 1, cursor.1));
                        } else if cursor.1 > 0 {
                            // Moves the whole current line to the end of the line above.
                            let prev_line = rope.lines(..).nth(cursor.1 - 1).unwrap();
                            let current_line = rope.lines(..).nth(cursor.1);

                            if let Some(current_line) = current_line {
                                let prev_char_idx =
                                    rope.offset_of_line(cursor.1 - 1) + prev_line.len();
                                let char_idx = rope.offset_of_line(cursor.1) + current_line.len();

                                content.with_mut(|code| {
                                    code.edit(prev_char_idx..prev_char_idx, current_line.clone());
                                    code.edit(char_idx..char_idx + current_line.len() + 1, "");
                                });
                            }

                            cursor_setter((prev_line.len(), cursor.1 - 1));
                        }
                    }
                    KeyCode::Enter => {
                        // Breaks the line
                        let total_lines = rope.lines(..).count();
                        let char_idx = rope.offset_of_line(cursor.1) + cursor.0;
                        let current_line = rope.lines(..).nth(cursor.1).unwrap();
                        content.with_mut(|code| {
                            let break_line =
                                if cursor.1 == total_lines - 1 && current_line.len() > 0 {
                                    "\n\n"
                                } else {
                                    "\n"
                                };
                            code.edit(char_idx..char_idx, break_line);
                        });

                        cursor_setter((0, cursor.1 + 1));
                    }
                    character => {
                        // Adds a new character to the right
                        if let Some(character) = character.to_text() {
                            let char_idx = rope.offset_of_line(cursor.1) + cursor.0;

                            content.with_mut(|code| {
                                code.edit(char_idx..char_idx, character);
                            });

                            cursor_setter((cursor.0 + 1, cursor.1));
                        }
                    }
                }
            }
        }
    });

    (
        content,
        cursor,
        keypress_channel_sender,
        click_channel_sender,
        cursor_ref_attr,
    )
}
