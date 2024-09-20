use std::rc::Rc;

use dioxus_core::{
    prelude::spawn,
    use_hook,
    AttributeValue,
};
use dioxus_sdk::clipboard::use_clipboard;
use dioxus_signals::{
    Readable,
    Signal,
    Writable,
};
use freya_common::CursorLayoutResponse;
use freya_core::prelude::{
    EventMessage,
    TextGroupMeasurement,
};
use freya_elements::events::{
    Code,
    KeyboardData,
    MouseData,
};
use freya_node_state::{
    CursorReference,
    CustomAttributeValues,
};
use tokio::sync::mpsc::unbounded_channel;
use torin::geometry::CursorPoint;
use uuid::Uuid;

use crate::{
    use_platform,
    EditorHistory,
    RopeEditor,
    TextCursor,
    TextEditor,
    TextEvent,
    UsePlatform,
};

/// Events emitted to the [`UseEditable`].
pub enum EditableEvent {
    Click,
    MouseMove(Rc<MouseData>, usize),
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
#[derive(Clone, Copy, PartialEq)]
pub struct UseEditable {
    pub(crate) editor: Signal<RopeEditor>,
    pub(crate) cursor_reference: Signal<CursorReference>,
    pub(crate) dragging: Signal<TextDragging>,
    pub(crate) platform: UsePlatform,
    pub(crate) allow_tabs: bool,
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
            EditableEvent::MouseMove(e, id) => {
                if let Some(src) = self.dragging.peek().get_cursor_coords() {
                    let new_dist = e.get_element_coordinates();

                    Some((*id, None, Some((src, new_dist))))
                } else {
                    None
                }
            }
            EditableEvent::Click => {
                let dragging = &mut *self.dragging.write();
                match dragging {
                    TextDragging::FromCursorToPoint { shift, clicked, .. } if *shift => {
                        *clicked = false;
                    }
                    _ => {
                        *dragging = TextDragging::None;
                    }
                }
                None
            }
            EditableEvent::KeyDown(e) => {
                match e.code {
                    // Handle dragging
                    Code::ShiftLeft => {
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
                    // Do not write Tabs
                    Code::Tab if !self.allow_tabs => {}
                    // Handle editing
                    _ => {
                        let event = self
                            .editor
                            .write()
                            .process_key(&e.key, &e.code, &e.modifiers);
                        if event.contains(TextEvent::TEXT_CHANGED) {
                            *self.dragging.write() = TextDragging::None;
                        }
                    }
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
    pub(crate) identation: u8,
    pub(crate) allow_tabs: bool,
}

impl EditableConfig {
    /// Create a [`EditableConfig`].
    pub fn new(content: String) -> Self {
        Self {
            content,
            cursor: TextCursor::default(),
            identation: 4,
            allow_tabs: false,
        }
    }

    /// Specify a custom initial cursor position.
    pub fn with_cursor(mut self, pos: usize) -> Self {
        self.cursor = TextCursor::new(pos);
        self
    }

    /// Specify a custom identation
    pub fn with_identation(mut self, identation: u8) -> Self {
        self.identation = identation;
        self
    }

    /// Specify whether you want to allow tabs to be inserted
    pub fn with_allow_tabs(mut self, allow_tabs: bool) -> Self {
        self.allow_tabs = allow_tabs;
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
            config.identation,
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
                        let new_cursor = text_editor.measure_new_cursor(position, id);

                        // Only update and clear the selection if the cursor has changed
                        if *text_editor.cursor() != new_cursor {
                            *text_editor.cursor_mut() = new_cursor;
                            if let TextDragging::FromCursorToPoint { cursor: from, .. } =
                                &*dragging.read()
                            {
                                let to = text_editor.cursor_pos();
                                text_editor.set_selection((*from, to));
                            } else {
                                text_editor.clear_selection();
                            }
                        }
                    }
                    // Update the text selections calculated by the layout
                    CursorLayoutResponse::TextSelection { from, to, id } => {
                        let current_cursor = editor.peek().cursor().clone();
                        let current_selection = editor.peek().get_selection();

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
            allow_tabs: config.allow_tabs,
        }
    })
}
