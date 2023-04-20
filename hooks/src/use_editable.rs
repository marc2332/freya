use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use dioxus_core::{AttributeValue, Scope, ScopeState};
use dioxus_hooks::{use_effect, use_ref, use_state, UseRef, UseState};
use freya_common::{CursorLayoutResponse, EventMessage, Point2D};
use freya_elements::events::{KeyboardData, MouseData};
use freya_node_state::{CursorReference, CustomAttributeValues};
pub use ropey::Rope;
use tokio::sync::{mpsc::unbounded_channel, mpsc::UnboundedSender};
use uuid::Uuid;
use winit::event_loop::EventLoopProxy;

use crate::{RopeEditor, TextCursor, TextEditor, TextEvent};

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

pub type ClickNotifier = UnboundedSender<EditableEvent>;
pub type EditorState = UseState<RopeEditor>;

/// Manage an editable content.
#[derive(Clone)]
pub struct UseEditable {
    pub(crate) editor: EditorState,
    pub(crate) cursor_reference: CursorReference,
    pub(crate) selecting_text_with_mouse: UseRef<Option<Point2D>>,
    pub(crate) event_loop_proxy: Option<EventLoopProxy<EventMessage>>,
}

impl UseEditable {
    /// Reference to the editor.
    pub fn editor(&self) -> &EditorState {
        &self.editor
    }

    /// Create a cursor attribute.
    pub fn cursor_attr<'a, T>(&self, cx: Scope<'a, T>) -> AttributeValue<'a> {
        cx.any_value(CustomAttributeValues::CursorReference(
            self.cursor_reference.clone(),
        ))
    }

    /// Create a highlights attribute.
    pub fn highlights_attr<'a, T>(&self, cx: Scope<'a, T>, editor_id: usize) -> AttributeValue<'a> {
        cx.any_value(CustomAttributeValues::TextHighlights(
            self.editor
                .get()
                .highlights(editor_id)
                .map(|v| vec![v])
                .unwrap_or_default(),
        ))
    }

    /// Process a [`EditableEvent`] event.
    pub fn process_event(&self, edit_event: &EditableEvent) {
        match edit_event {
            EditableEvent::MouseDown(e, id) => {
                let coords = e.get_element_coordinates();
                *self.selecting_text_with_mouse.write_silent() = Some(coords);

                self.cursor_reference.set_id(Some(*id));
                self.cursor_reference.set_cursor_position(Some(coords));

                self.editor.with_mut(|editor| {
                    editor.unhighlight();
                });
            }
            EditableEvent::MouseOver(e, id) => {
                self.selecting_text_with_mouse.with(|selecting_text| {
                    if let Some(current_dragging) = selecting_text {
                        let coords = e.get_element_coordinates();

                        self.cursor_reference.set_id(Some(*id));
                        self.cursor_reference
                            .set_cursor_selections(Some((*current_dragging, coords)));
                    }
                });
            }
            EditableEvent::Click => {
                *self.selecting_text_with_mouse.write_silent() = None;
            }
            EditableEvent::KeyDown(e) => {
                self.editor.with_mut(|editor| {
                    let event = editor.process_key(&e.key, &e.code, &e.modifiers);

                    if event == TextEvent::TextChanged {
                        *self.selecting_text_with_mouse.write_silent() = None;
                    }
                });
            }
        }

        if self.selecting_text_with_mouse.read().is_some() {
            if let Some(event_loop_proxy) = &self.event_loop_proxy {
                event_loop_proxy
                    .send_event(EventMessage::RemeasureTextGroup(
                        self.cursor_reference.text_id,
                    ))
                    .unwrap();
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
pub fn use_editable(
    cx: &ScopeState,
    initializer: impl Fn() -> EditableConfig,
    mode: EditableMode,
) -> UseEditable {
    let id = cx.use_hook(Uuid::new_v4);
    let event_loop_proxy = cx.consume_context::<EventLoopProxy<EventMessage>>();

    // Hold the text editor
    let text_editor = use_state(cx, || {
        let config = initializer();
        RopeEditor::new(config.content, config.cursor, mode)
    });

    let cursor_channels = cx.use_hook(|| {
        let (tx, rx) = unbounded_channel::<CursorLayoutResponse>();
        (tx, Some(rx))
    });

    let selecting_text_with_mouse = use_ref(cx, || None);

    // Cursor reference passed to the layout engine
    let cursor_reference = cx.use_hook(|| CursorReference {
        text_id: *id,
        agent: cursor_channels.0.clone(),
        cursor_position: Arc::new(Mutex::new(None)),
        id: Arc::new(Mutex::new(None)),
        cursor_selections: Arc::new(Mutex::new(None)),
    });

    let use_editable = UseEditable {
        editor: text_editor.clone(),
        cursor_reference: cursor_reference.clone(),
        selecting_text_with_mouse: selecting_text_with_mouse.clone(),
        event_loop_proxy,
    };

    // Listen for new calculations from the layout engine
    use_effect(cx, (), move |_| {
        let cursor_reference = cursor_reference.clone();
        let cursor_receiver = cursor_channels.1.take();
        let editor = text_editor.clone();

        async move {
            let mut cursor_receiver = cursor_receiver.unwrap();

            while let Some(message) = cursor_receiver.recv().await {
                match message {
                    // Update the cursor position calculated by the layout
                    CursorLayoutResponse::CursorPosition { position, id } => {
                        let text_editor = editor.current();

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
                            editor.with_mut(|text_editor| {
                                text_editor.cursor_mut().set_col(new_cursor.0);
                                text_editor.cursor_mut().set_row(new_cursor.1);
                                text_editor.unhighlight();
                            })
                        }

                        // Remove the current calcutions so the layout engine doesn't try to calculate again
                        cursor_reference.set_cursor_position(None);
                    }
                    // Update the text selections calculated by the layout
                    CursorLayoutResponse::TextSelection { from, to, id } => {
                        editor.with_mut(|text_editor| {
                            text_editor.highlight_text(from, to, id);
                        });
                        cursor_reference.set_cursor_selections(None);
                    }
                }
            }
        }
    });

    use_editable
}
