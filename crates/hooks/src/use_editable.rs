use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use dioxus_core::{prelude::spawn, use_hook, AttributeValue};
use dioxus_hooks::to_owned;
use dioxus_sdk::clipboard::use_clipboard;
use dioxus_signals::{Readable, Signal, Writable};
use freya_common::{CursorLayoutResponse, EventMessage};
use freya_elements::events::{KeyboardData, MouseData};
use freya_node_state::{CursorReference, CustomAttributeValues};
use tokio::sync::mpsc::unbounded_channel;
use torin::geometry::CursorPoint;
use uuid::Uuid;

use crate::{
    use_platform, EditorHistory, RopeEditor, TextCursor, TextEditor, TextEvent, UsePlatform,
};

/// Events emitted to the [`UseEditable`].
pub enum EditableEvent {
    Click,
    MouseOver(Rc<MouseData>, usize),
    MouseDown(Rc<MouseData>, usize),
    KeyDown(Rc<KeyboardData>),
}

/// How the editable content must behave.
#[derive(PartialEq, Eq, Clone, Copy)]
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

impl Default for EditableMode {
    fn default() -> Self {
        Self::MultipleLinesSingleEditor
    }
}

/// Manage an editable content.
#[derive(Clone, Copy)]
pub struct UseEditable {
    pub(crate) editor: Signal<RopeEditor>,
    pub(crate) cursor_reference: Signal<CursorReference>,
    pub(crate) selecting_text_with_mouse: Signal<Option<CursorPoint>>,
    pub(crate) platform: UsePlatform,
}

impl UseEditable {
    /// Reference to the editor.
    pub fn editor(&self) -> &Signal<RopeEditor> {
        &self.editor
    }

    /// Mutable reference to the editor.
    pub fn editor_mut(&mut self) -> &mut Signal<RopeEditor> {
        &mut self.editor
    }

    /// Create a cursor attribute.
    pub fn cursor_attr(&self) -> AttributeValue {
        AttributeValue::any_value(CustomAttributeValues::CursorReference(
            self.cursor_reference.peek().clone(),
        ))
    }

    /// Create a highlights attribute.
    pub fn highlights_attr(&self, editor_id: usize) -> AttributeValue {
        AttributeValue::any_value(CustomAttributeValues::TextHighlights(
            self.editor
                .read()
                .highlights(editor_id)
                .map(|v| vec![v])
                .unwrap_or_default(),
        ))
    }

    /// Process a [`EditableEvent`] event.
    pub fn process_event(&mut self, edit_event: &EditableEvent) {
        match edit_event {
            EditableEvent::MouseDown(e, id) => {
                let coords = e.get_element_coordinates();
                *self.selecting_text_with_mouse.write() = Some(coords);

                self.cursor_reference.peek().set_id(Some(*id));
                self.cursor_reference
                    .peek()
                    .set_cursor_position(Some(coords));

                self.editor.write().unhighlight();
            }
            EditableEvent::MouseOver(e, id) => {
                self.selecting_text_with_mouse.with(|selecting_text| {
                    if let Some(current_dragging) = selecting_text {
                        let coords = e.get_element_coordinates();

                        self.cursor_reference.peek().set_id(Some(*id));
                        self.cursor_reference
                            .peek()
                            .set_cursor_selections(Some((*current_dragging, coords)));
                    }
                });
            }
            EditableEvent::Click => {
                *self.selecting_text_with_mouse.write() = None;
            }
            EditableEvent::KeyDown(e) => {
                let event = self
                    .editor
                    .write()
                    .process_key(&e.key, &e.code, &e.modifiers);
                if event.contains(TextEvent::TEXT_CHANGED) {
                    *self.selecting_text_with_mouse.write() = None;
                }
            }
        }

        if self.selecting_text_with_mouse.peek().is_some() {
            self.platform
                .send(EventMessage::RemeasureTextGroup(
                    self.cursor_reference.peek().text_id,
                ))
                .unwrap()
        }
    }
}

/// Create a configuration for a [`UseEditable`].
pub struct EditableConfig {
    pub(crate) content: String,
    pub(crate) cursor: TextCursor,
}

impl EditableConfig {
    /// Create a [`EditableConfig`].
    pub fn new(content: String) -> Self {
        Self {
            content,
            cursor: TextCursor::default(),
        }
    }

    /// Specify a custom initial cursor positions.
    pub fn with_cursor(mut self, (row, col): (usize, usize)) -> Self {
        self.cursor = TextCursor::new(row, col);
        self
    }
}

/// Create a virtual text editor with it's own cursor and rope.
pub fn use_editable(initializer: impl Fn() -> EditableConfig, mode: EditableMode) -> UseEditable {
    let platform = use_platform();
    let clipboard = use_clipboard();

    use_hook(|| {
        let text_id = Uuid::new_v4();
        let config = initializer();
        let mut editor = Signal::new(RopeEditor::new(
            config.content,
            config.cursor,
            mode,
            clipboard,
            EditorHistory::new(),
        ));
        let selecting_text_with_mouse = Signal::new(None);
        let (cursor_sender, mut cursor_receiver) = unbounded_channel::<CursorLayoutResponse>();
        let cursor_reference = CursorReference {
            text_id,
            cursor_sender,
            cursor_position: Arc::new(Mutex::new(None)),
            cursor_id: Arc::new(Mutex::new(None)),
            cursor_selections: Arc::new(Mutex::new(None)),
        };

        spawn({
            to_owned![cursor_reference];
            async move {
                while let Some(message) = cursor_receiver.recv().await {
                    match message {
                        // Update the cursor position calculated by the layout
                        CursorLayoutResponse::CursorPosition { position, id } => {
                            let mut text_editor = editor.write();

                            let new_cursor_row = match mode {
                                EditableMode::MultipleLinesSingleEditor => {
                                    text_editor.char_to_line(position)
                                }
                                EditableMode::SingleLineMultipleEditors => id,
                            };

                            let new_cursor_col = match mode {
                                EditableMode::MultipleLinesSingleEditor => {
                                    position - text_editor.line_to_char(new_cursor_row)
                                }
                                EditableMode::SingleLineMultipleEditors => position,
                            };

                            let new_current_line = text_editor.line(new_cursor_row).unwrap();

                            // Use the line length as new column if the clicked column surpases the length
                            let new_cursor = if new_cursor_col >= new_current_line.len_chars() {
                                (new_current_line.len_chars(), new_cursor_row)
                            } else {
                                (new_cursor_col, new_cursor_row)
                            };

                            // Only update if it's actually different
                            if text_editor.cursor().as_tuple() != new_cursor {
                                text_editor.cursor_mut().set_col(new_cursor.0);
                                text_editor.cursor_mut().set_row(new_cursor.1);
                                text_editor.unhighlight();
                            }

                            // Remove the current calcutions so the layout engine doesn't try to calculate again
                            cursor_reference.set_cursor_position(None);
                        }
                        // Update the text selections calculated by the layout
                        CursorLayoutResponse::TextSelection { from, to, id } => {
                            editor.write().highlight_text(from, to, id);
                            cursor_reference.set_cursor_selections(None);
                        }
                    }
                }
            }
        });

        UseEditable {
            editor,
            cursor_reference: Signal::new(cursor_reference.clone()),
            selecting_text_with_mouse,
            platform,
        }
    })
}
