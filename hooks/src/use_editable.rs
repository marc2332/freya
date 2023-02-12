use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use dioxus_core::{AttributeValue, Event, ScopeState};
use dioxus_hooks::{use_effect, use_ref, use_state, UseRef, UseState};
use freya_elements::events_data::{KeyboardData, MouseData};
use freya_node_state::{CursorReference, CustomAttributeValues};
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
    // Hold the actual editable content
    let content = use_state(cx, || UseEditableText::from(initializer()));

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
        let cursor_receiver = cursor_channels.1.take();
        let content = content.clone();
        let rope = content.clone();

        async move {
            let mut cursor_receiver = cursor_receiver.unwrap();
            let cursor_ref = cursor_ref.clone();

            while let Some((new_index, editor_num)) = cursor_receiver.recv().await {
                let content = content.current();

                let new_cursor_row = match mode {
                    EditableMode::MultipleLinesSingleEditor => content.char_to_line(new_index),
                    EditableMode::SingleLineMultipleEditors => editor_num,
                };

                let new_cursor_col = match mode {
                    EditableMode::MultipleLinesSingleEditor => {
                        new_index - content.line_to_char(new_cursor_row)
                    }
                    EditableMode::SingleLineMultipleEditors => new_index,
                };

                let new_current_line = content.line(new_cursor_row).unwrap();

                // Use the line lenght as new column if the clicked column surpases the length
                let new_cursor = if new_cursor_col >= new_current_line.len_chars() {
                    (new_current_line.len_chars(), new_cursor_row)
                } else {
                    (new_cursor_col, new_cursor_row)
                };

                // Only update if it's actually different
                if rope.cursor().col() != new_cursor.0 || rope.cursor().row() != new_cursor.1 {
                    rope.with_mut(|rope| {
                        rope.cursor_mut().set_col(new_cursor.0);
                        rope.cursor_mut().set_row(new_cursor.1);
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
        let rope = content.clone();
        async move {
            let mut rx = rx.unwrap();

            while let Some(pressed_key) = rx.recv().await {
                rope.with_mut(|rope| {
                    rope.process_key(&pressed_key.key, &pressed_key.code, &pressed_key.modifiers);
                });
            }
        }
    });

    (
        content,
        keypress_channel_sender,
        click_channel_sender,
        cursor_ref_attr,
    )
}
