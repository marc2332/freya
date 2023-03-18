use std::{
    fmt::Display,
    ops::Range,
    rc::Rc,
    sync::{Arc, Mutex},
};

use dioxus_core::{AttributeValue, Event, ScopeState};
use dioxus_hooks::{use_effect, use_ref, use_state, UseRef, UseState};
use freya_common::EventMessage;
use freya_elements::events_data::{KeyboardData, MouseData};
use freya_node_state::{CursorReference, CustomAttributeValues};
use glutin::event_loop::EventLoopProxy;
use ropey::iter::Lines;
pub use ropey::Rope;
use tokio::sync::{mpsc::unbounded_channel, mpsc::UnboundedSender};

use crate::text_editor::*;

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
pub type EditableText = UseState<UseEditableText>;
pub type KeyboardEvent = Event<KeyboardData>;
pub type CursorRef = UseRef<CursorReference>;

/// Create a virtual text editor with it's own cursor and rope.
pub fn use_editable<'a>(
    cx: &ScopeState,
    initializer: impl Fn() -> &'a str,
    mode: EditableMode,
) -> (
    &EditableText,
    KeypressNotifier,
    ClickNotifier,
    AttributeValue,
) {
    // Hold the text editor manager
    let text_editor = use_state(cx, || UseEditableText::from(initializer()));

    let cursor_channels = cx.use_hook(|| {
        let (tx, rx) = unbounded_channel::<(usize, usize)>();
        (tx, Some(rx))
    });

    // Cursor reference passed to the layout engine
    let cursor_ref = use_ref(cx, || CursorReference {
        agent: cursor_channels.0.clone(),
        positions: Arc::new(Mutex::new(None)),
        cursor_id: Arc::new(Mutex::new(None)),
    });

    // This will allow to pass the cursor reference as an attribute value
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

    // Listen for click events and pass them to the layout engine
    {
        let cursor_ref = cursor_ref.clone();
        use_effect(cx, (), move |_| {
            let rx = click_channel.1.take();
            let event_loop_proxy = cx.consume_context::<EventLoopProxy<EventMessage>>();
            async move {
                let mut rx = rx.unwrap();

                while let Some((e, id)) = rx.recv().await {
                    let points = e.get_element_coordinates();
                    let cursor_ref = cursor_ref.clone();
                    cursor_ref.write().cursor_id.lock().unwrap().replace(id);
                    cursor_ref
                        .write()
                        .positions
                        .lock()
                        .unwrap()
                        .replace((points.x as f32, points.y as f32));

                    // Request the renderer to relayout
                    if let Some(event_loop_proxy) = &event_loop_proxy {
                        event_loop_proxy
                            .send_event(EventMessage::RequestRelayout)
                            .unwrap();
                    }
                }
            }
        });
    }

    // Listen for new calculations from the layout engine
    use_effect(cx, (), move |_| {
        let cursor_ref = cursor_ref.clone();
        let cursor_receiver = cursor_channels.1.take();
        let editor = text_editor.clone();

        async move {
            let mut cursor_receiver = cursor_receiver.unwrap();
            let cursor_ref = cursor_ref.clone();

            while let Some((new_index, editor_num)) = cursor_receiver.recv().await {
                let text_editor = editor.current();

                let new_cursor_row = match mode {
                    EditableMode::MultipleLinesSingleEditor => text_editor.char_to_line(new_index),
                    EditableMode::SingleLineMultipleEditors => editor_num,
                };

                let new_cursor_col = match mode {
                    EditableMode::MultipleLinesSingleEditor => {
                        new_index - text_editor.line_to_char(new_cursor_row)
                    }
                    EditableMode::SingleLineMultipleEditors => new_index,
                };

                let new_current_line = text_editor.line(new_cursor_row).unwrap();

                // Use the line lenght as new column if the clicked column surpases the length
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
                    })
                }

                // Remove the current calcutions so the layout engine doesn't try to calculate again
                cursor_ref.write().positions.lock().unwrap().take();
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
                    text_editor.process_key(
                        &pressed_key.key,
                        &pressed_key.code,
                        &pressed_key.modifiers,
                    );
                });
            }
        }
    });

    (
        text_editor,
        keypress_channel_sender,
        click_channel_sender,
        cursor_ref_attr,
    )
}

#[derive(Clone)]
pub struct UseEditableText {
    rope: Rope,
    cursor: TextCursor,
}

impl Display for UseEditableText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.rope.to_string())
    }
}

impl TextEditor for UseEditableText {
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

impl From<&str> for UseEditableText {
    fn from(value: &str) -> Self {
        Self {
            rope: Rope::from_str(value),
            cursor: TextCursor::new(0, 0),
        }
    }
}
