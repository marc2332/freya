use std::{sync::{Arc, Mutex}, rc::Rc};

use dioxus_core::{ScopeState, Event};
use dioxus_hooks::{use_effect, use_state, UseState, UseRef, use_ref};
use freya_elements::events_data::{KeyCode, KeyboardData, MouseData};
use freya_node_state::CursorReference;
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
    &CursorRef,
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

    (
        content,
        cursor,
        keypress_channel_sender,
        click_channel_sender,
        cursor_ref,
    )
}
