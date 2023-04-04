use std::{
    collections::HashMap,
    fmt::Display,
    ops::Range,
    rc::Rc,
    sync::{Arc, Mutex},
};

use dioxus_core::{AttributeValue, Event, Scope, ScopeState};
use dioxus_hooks::{use_effect, use_state, UseState};
use freya_common::{CursorLayoutResponse, EventMessage};
use freya_elements::events::{KeyboardData, MouseData};
use freya_node_state::{CursorReference, CustomAttributeValues};
use ropey::iter::Lines;
pub use ropey::Rope;
use tokio::sync::{mpsc::unbounded_channel, mpsc::UnboundedSender};
use winit::event_loop::EventLoopProxy;

use crate::text_editor::*;

pub type KeypressNotifier = UnboundedSender<Rc<KeyboardData>>;
pub type ClickNotifier = UnboundedSender<(Rc<MouseData>, usize, EditableEvent)>;
pub type EditorState = UseState<RopeEditor>;
pub type KeyboardEvent = Event<KeyboardData>;

/// Events emitted to the [`UseEditable`].
#[derive(PartialEq, Eq)]
pub enum EditableEvent {
    Click,
    MouseOver,
    MouseDown,
}

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

/// Manage an editable content.
#[derive(Clone)]
pub struct UseEditable {
    pub editor: EditorState,
    pub keypress_notifier: KeypressNotifier,
    pub click_notifier: ClickNotifier,
    pub cursor_reference: CursorReference,
}

impl UseEditable {
    /// Reference to the editor.
    pub fn editor(&self) -> &EditorState {
        &self.editor
    }

    /// Reference to the Keypress notifier.
    pub fn keypress_notifier(&self) -> &KeypressNotifier {
        &self.keypress_notifier
    }

    /// Reference to the click notifier.
    pub fn click_notifier(&self) -> &ClickNotifier {
        &self.click_notifier
    }

    /// Create a cursor attribute.
    pub fn cursor_attr<'a>(&self, cx: Scope<'a>) -> AttributeValue<'a> {
        cx.any_value(CustomAttributeValues::CursorReference(
            self.cursor_reference.clone(),
        ))
    }

    /// Create a highlights attribute.
    pub fn highlights_attr<'a>(&self, cx: Scope<'a>, editor_num: usize) -> AttributeValue<'a> {
        cx.any_value(CustomAttributeValues::TextHighlights(
            self.editor.get().highlights(editor_num).unwrap_or_default(),
        ))
    }
}

/// Create a virtual text editor with it's own cursor and rope.
pub fn use_editable(
    cx: &ScopeState,
    initializer: impl Fn() -> String,
    mode: EditableMode,
) -> UseEditable {
    // Hold the text editor manager
    let text_editor = use_state(cx, || RopeEditor::from(initializer()));

    let cursor_channels = cx.use_hook(|| {
        let (tx, rx) = unbounded_channel::<CursorLayoutResponse>();
        (tx, Some(rx))
    });

    // Cursor reference passed to the layout engine
    let cursor_reference = cx.use_hook(|| CursorReference {
        agent: cursor_channels.0.clone(),
        positions: Arc::new(Mutex::new(None)),
        id: Arc::new(Mutex::new(None)),
        highlights: Arc::new(Mutex::new(None)),
    });

    // Move cursor with clicks
    let click_channel = cx.use_hook(|| {
        let (tx, rx) = unbounded_channel::<(Rc<MouseData>, usize, EditableEvent)>();
        (tx, Some(rx))
    });

    // Write into the text
    let keypress_channel = cx.use_hook(|| {
        let (tx, rx) = unbounded_channel::<Rc<KeyboardData>>();
        (tx, Some(rx))
    });

    let use_editable = UseEditable {
        editor: text_editor.clone(),
        keypress_notifier: keypress_channel.0.clone(),
        click_notifier: click_channel.0.clone(),
        cursor_reference: cursor_reference.clone(),
    };

    // Listen for click events and pass them to the layout engine
    {
        let cursor_reference = cursor_reference.clone();
        use_effect(cx, (), move |_| {
            let editor = text_editor.clone();
            let rx = click_channel.1.take();
            let event_loop_proxy = cx.consume_context::<EventLoopProxy<EventMessage>>();
            async move {
                let mut rx = rx.unwrap();
                let mut current_dragging = None;

                while let Some((e, id, edit_event)) = rx.recv().await {
                    match edit_event {
                        EditableEvent::MouseDown => {
                            let coords = e.get_element_coordinates();
                            current_dragging = Some(coords);

                            let cursor_reference = cursor_reference.clone();
                            cursor_reference.id.lock().unwrap().replace(id);
                            cursor_reference
                                .positions
                                .lock()
                                .unwrap()
                                .replace((coords.x as f32, coords.y as f32));

                            editor.with_mut(|text_editor| {
                                text_editor.clear_highlights();
                            });
                        }
                        EditableEvent::MouseOver => {
                            if let Some(initial_dragging) = current_dragging {
                                let coords = e.get_element_coordinates();
                                let cursor_reference = cursor_reference.clone();
                                cursor_reference.id.lock().unwrap().replace(id);
                                cursor_reference.highlights.lock().unwrap().replace((
                                    initial_dragging.to_usize().to_tuple(),
                                    coords.to_usize().to_tuple(),
                                ));
                            }
                        }
                        EditableEvent::Click => {
                            current_dragging = None;
                        }
                    }

                    if edit_event != EditableEvent::Click {
                        if let Some(event_loop_proxy) = &event_loop_proxy {
                            event_loop_proxy
                                .send_event(EventMessage::RequestRelayout)
                                .unwrap();
                        }
                    }
                }
            }
        });
    }

    // Listen for new calculations from the layout engine
    use_effect(cx, (), move |_| {
        let cursor_reference = cursor_reference.clone();
        let cursor_receiver = cursor_channels.1.take();
        let editor = text_editor.clone();

        async move {
            let mut cursor_receiver = cursor_receiver.unwrap();
            let cursor_reference = cursor_reference.clone();

            while let Some(message) = cursor_receiver.recv().await {
                match message {
                    CursorLayoutResponse::CursorPosition((new_index, editor_num)) => {
                        let text_editor = editor.current();

                        let new_cursor_row = match mode {
                            EditableMode::MultipleLinesSingleEditor => {
                                text_editor.char_to_line(new_index)
                            }
                            EditableMode::SingleLineMultipleEditors => editor_num,
                        };

                        let new_cursor_col = match mode {
                            EditableMode::MultipleLinesSingleEditor => {
                                new_index - text_editor.line_to_char(new_cursor_row)
                            }
                            EditableMode::SingleLineMultipleEditors => new_index,
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
                                text_editor.clear_highlights();
                            })
                        }

                        // Remove the current calcutions so the layout engine doesn't try to calculate again
                        cursor_reference.positions.lock().unwrap().take();
                    }
                    CursorLayoutResponse::Highlight((highlights, editor_num)) => {
                        editor.with_mut(|text_editor| {
                            text_editor.set_highlights(vec![highlights], editor_num);
                        });

                        cursor_reference.highlights.lock().unwrap().take();
                    }
                }
            }
        }
    });

    // Listen for keypresses
    use_effect(cx, (), move |_| {
        let rx = keypress_channel.1.take();
        let text_editor = text_editor.clone();
        async move {
            let mut rx = rx.unwrap();

            while let Some(pressed_key) = rx.recv().await {
                text_editor.with_mut(|text_editor| {
                    text_editor.clear_highlights();
                    text_editor.process_key(
                        &pressed_key.key,
                        &pressed_key.code,
                        &pressed_key.modifiers,
                    );
                });
            }
        }
    });

    use_editable
}

/// TextEditor implementing a Rope
#[derive(Clone)]
pub struct RopeEditor {
    rope: Rope,
    cursor: TextCursor,
    highlights: HashMap<usize, Vec<(usize, usize)>>,
}

impl Display for RopeEditor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.rope.to_string())
    }
}

impl RopeEditor {
    pub fn clear_highlights(&mut self) {
        if !self.highlights.is_empty() {
            self.highlights = HashMap::default()
        }
    }
}

impl TextEditor for RopeEditor {
    type LinesIterator<'a> = LinesIterator<'a>;

    fn lines(&self) -> Self::LinesIterator<'_> {
        let lines = self.rope.lines();
        LinesIterator { lines }
    }

    fn insert_char(&mut self, char: char, char_idx: usize) {
        self.rope.insert_char(char_idx, char);
    }

    fn insert(&mut self, text: &str, char_idx: usize) {
        self.rope.insert(char_idx, text);
    }

    fn remove(&mut self, range: Range<usize>) {
        self.rope.remove(range)
    }

    fn char_to_line(&self, char_idx: usize) -> usize {
        self.rope.char_to_line(char_idx)
    }

    fn line_to_char(&self, line_idx: usize) -> usize {
        self.rope.line_to_char(line_idx)
    }

    fn line(&self, line_idx: usize) -> Option<Line<'_>> {
        let line = self.rope.get_line(line_idx);

        line.map(|line| Line {
            text: line.as_str().unwrap_or(""),
        })
    }

    fn len_lines<'a>(&self) -> usize {
        self.rope.len_lines()
    }

    fn cursor(&self) -> &TextCursor {
        &self.cursor
    }

    fn cursor_mut(&mut self) -> &mut TextCursor {
        &mut self.cursor
    }

    fn set_highlights(&mut self, highlights: Vec<(usize, usize)>, editor_id: usize) {
        let entry = self.highlights.entry(editor_id).or_default();
        entry.clear();
        entry.extend(highlights);
    }

    fn highlights(&self, editor_id: usize) -> Option<Vec<(usize, usize)>> {
        self.highlights.get(&editor_id).cloned()
    }
}

/// Iterator over text lines.
pub struct LinesIterator<'a> {
    lines: Lines<'a>,
}

impl<'a> Iterator for LinesIterator<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();

        line.map(|line| Line {
            text: line.as_str().unwrap_or(""),
        })
    }
}

impl From<String> for RopeEditor {
    fn from(value: String) -> Self {
        Self {
            rope: Rope::from_str(&value),
            cursor: TextCursor::new(0, 0),
            highlights: HashMap::new(),
        }
    }
}
