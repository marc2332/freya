use std::rc::Rc;

use dioxus_core::{prelude::spawn, use_hook, AttributeValue};
use dioxus_sdk::clipboard::use_clipboard;
use dioxus_signals::{Readable, Signal, Writable};
use freya_common::{CursorLayoutResponse, EventMessage, TextGroupMeasurement};
use freya_elements::events::{Code, KeyboardData, MouseData};
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
    KeyUp(Rc<KeyboardData>),
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

/// Indicates the type of text dragging being done.
#[derive(Debug, PartialEq, Clone)]
pub enum TextDragging {
    None,
    FromPointToPoint {
        src: CursorPoint,
    },
    FromCursorToPoint {
        shift: bool,
        clicked: bool,
        cursor: usize,
        dist: Option<CursorPoint>,
    },
}

impl TextDragging {
    pub fn has_cursor_coords(&self) -> bool {
        match self {
            Self::None => false,
            Self::FromPointToPoint { .. } => true,
            Self::FromCursorToPoint { dist, .. } => dist.is_some(),
        }
    }

    pub fn set_cursor_coords(&mut self, cursor: CursorPoint) {
        match self {
            Self::FromPointToPoint { src } => *src = cursor,
            Self::FromCursorToPoint {
                dist, shift: true, ..
            } => *dist = Some(cursor),
            _ => *self = Self::FromPointToPoint { src: cursor },
        }
    }

    pub fn get_cursor_coords(&self) -> Option<CursorPoint> {
        match self {
            Self::None => None,
            Self::FromPointToPoint { src } => Some(*src),
            Self::FromCursorToPoint { dist, clicked, .. } => {
                if *clicked {
                    *dist
                } else {
                    None
                }
            }
        }
    }
}

/// Manage an editable content.
#[derive(Clone, Copy)]
pub struct UseEditable {
    pub(crate) editor: Signal<RopeEditor>,
    pub(crate) cursor_reference: Signal<CursorReference>,
    pub(crate) dragging: Signal<TextDragging>,
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
                .get_visible_selection(editor_id)
                .map(|v| vec![v])
                .unwrap_or_default(),
        ))
    }

    /// Process a [`EditableEvent`] event.
    pub fn process_event(&mut self, edit_event: &EditableEvent) {
        let res = match edit_event {
            EditableEvent::MouseDown(e, id) => {
                let coords = e.get_element_coordinates();

                self.dragging.write().set_cursor_coords(coords);
                self.editor.write().clear_selection();

                Some((*id, Some(coords), None))
            }
            EditableEvent::MouseOver(e, id) => {
                if let Some(src) = self.dragging.peek().get_cursor_coords() {
                    let new_dist = e.get_element_coordinates();

                    Some((*id, None, Some((src, new_dist))))
                } else {
                    None
                }
            }
            EditableEvent::Click => {
                let selection = &mut *self.dragging.write();
                match selection {
                    TextDragging::FromCursorToPoint { shift, clicked, .. } if *shift => {
                        *clicked = false;
                    }
                    _ => {
                        *selection = TextDragging::None;
                    }
                }
                None
            }
            EditableEvent::KeyDown(e) => {
                if e.code == Code::ShiftLeft {
                    let dragging = &mut *self.dragging.write();
                    match dragging {
                        TextDragging::FromCursorToPoint {
                            shift: shift_pressed,
                            ..
                        } => {
                            *shift_pressed = true;
                        }
                        TextDragging::None => {
                            *dragging = TextDragging::FromCursorToPoint {
                                shift: true,
                                clicked: false,
                                cursor: self.editor.peek().cursor_pos(),
                                dist: None,
                            }
                        }
                        _ => {}
                    }
                }
                let event = self
                    .editor
                    .write()
                    .process_key(&e.key, &e.code, &e.modifiers);
                if event.contains(TextEvent::TEXT_CHANGED) {
                    *self.dragging.write() = TextDragging::None;
                }

                None
            }
            EditableEvent::KeyUp(e) => {
                if e.code == Code::ShiftLeft {
                    if let TextDragging::FromCursorToPoint { shift, .. } =
                        &mut *self.dragging.write()
                    {
                        *shift = false;
                    }
                } else {
                    *self.dragging.write() = TextDragging::None;
                }

                None
            }
        };

        if let Some((cursor_id, cursor_position, cursor_selection)) = res {
            if self.dragging.peek().has_cursor_coords() {
                self.platform
                    .send(EventMessage::RemeasureTextGroup(TextGroupMeasurement {
                        text_id: self.cursor_reference.peek().text_id,
                        cursor_id,
                        cursor_position,
                        cursor_selection,
                    }))
                    .unwrap()
            }
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
        let dragging = Signal::new(TextDragging::None);
        let (cursor_sender, mut cursor_receiver) = unbounded_channel::<CursorLayoutResponse>();
        let cursor_reference = CursorReference {
            text_id,
            cursor_sender,
        };

        spawn(async move {
            while let Some(message) = cursor_receiver.recv().await {
                match message {
                    // Update the cursor position calculated by the layout
                    CursorLayoutResponse::CursorPosition { position, id } => {
                        let mut text_editor = editor.write();

                        let new_cursor_row = match mode {
                            EditableMode::MultipleLinesSingleEditor => {
                                text_editor.char_to_line(text_editor.utf16_cu_to_char(position))
                            }
                            EditableMode::SingleLineMultipleEditors => id,
                        };

                        let new_cursor_col = match mode {
                            EditableMode::MultipleLinesSingleEditor => text_editor
                                .utf16_cu_to_char(
                                    position
                                        - text_editor.char_to_utf16_cu(
                                            text_editor.line_to_char(new_cursor_row),
                                        ),
                                ),
                            EditableMode::SingleLineMultipleEditors => {
                                text_editor.utf16_cu_to_char(position)
                            }
                        };

                        let new_current_line = text_editor.line(new_cursor_row).unwrap();

                        // Use the line length as new column if the clicked column surpases the length
                        let new_cursor = if new_cursor_col >= new_current_line.utf16_len_chars() {
                            (
                                text_editor.utf16_cu_to_char(new_current_line.utf16_len_chars()),
                                new_cursor_row,
                            )
                        } else {
                            (new_cursor_col, new_cursor_row)
                        };

                        // Only update and clear the selection if the cursor has changed
                        if text_editor.cursor().as_tuple() != new_cursor {
                            text_editor.cursor_mut().set_col(new_cursor.0);
                            text_editor.cursor_mut().set_row(new_cursor.1);
                            if let TextDragging::FromCursorToPoint { cursor: from, .. } = dragging()
                            {
                                let to = text_editor.cursor_pos();
                                let maybe_new_selection =
                                    text_editor.measure_new_selection(from, to, id);
                                text_editor.set_selection(maybe_new_selection);
                            } else {
                                text_editor.clear_selection();
                            }
                        }
                    }
                    // Update the text selections calculated by the layout
                    CursorLayoutResponse::TextSelection { from, to, id } => {
                        let current_cursor = editor.peek().cursor().clone();
                        let current_selection = editor.peek().get_selection();

                        let (from, to) = (
                            editor.peek().utf16_cu_to_char(from),
                            editor.peek().utf16_cu_to_char(to),
                        );
                        let maybe_new_cursor = editor.peek().measure_new_cursor(to, id);
                        let maybe_new_selection = editor.peek().measure_new_selection(from, to, id);

                        // Update the text selection if it has changed
                        if let Some(current_selection) = current_selection {
                            if current_selection != maybe_new_selection {
                                let mut text_editor = editor.write();
                                text_editor.set_selection(maybe_new_selection);
                            }
                        } else {
                            let mut text_editor = editor.write();
                            text_editor.set_selection(maybe_new_selection);
                        }

                        // Update the cursor if it has changed
                        if current_cursor != maybe_new_cursor {
                            let mut text_editor = editor.write();
                            *text_editor.cursor_mut() = maybe_new_cursor;
                        }
                    }
                }
            }
        });

        UseEditable {
            editor,
            cursor_reference: Signal::new(cursor_reference.clone()),
            dragging,
            platform,
        }
    })
}
