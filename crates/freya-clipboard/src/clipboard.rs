//! Provides a clipboard abstraction to access the target system's clipboard.

use copypasta::{
    ClipboardContext,
    ClipboardProvider,
};
use freya_core::{
    prelude::*,
    scope_id::ScopeId,
};

#[derive(Debug, PartialEq, Clone)]
pub enum ClipboardError {
    FailedToRead,
    FailedToSet,
    NotAvailable,
}

/// Access the clipboard.
///
/// # Examples
///
/// ```rust,no_run
/// use freya_clipboard::prelude::Clipboard;
///
/// // Read the clipboard content
/// if let Ok(content) = Clipboard::get() {
///     println!("{}", content);
/// }
///
/// // Write to the clipboard
/// Clipboard::set("Hello, Dioxus!".to_string());
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct Clipboard;

impl Clipboard {
    pub(crate) fn create_or_create() -> State<Option<ClipboardContext>> {
        let clipboard = match try_consume_root_context() {
            Some(rt) => rt,
            None => {
                let clipboard_state =
                    State::create_in_scope(ClipboardContext::new().ok(), ScopeId::ROOT);
                provide_context_for_scope_id(clipboard_state, ScopeId::ROOT);
                clipboard_state
            }
        };
        clipboard
    }

    // Read from the clipboard
    pub fn get() -> Result<String, ClipboardError> {
        Self::create_or_create()
            .write()
            .as_mut()
            .ok_or(ClipboardError::NotAvailable)?
            .get_contents()
            .map_err(|_| ClipboardError::FailedToRead)
    }

    // Write to the clipboard
    pub fn set(contents: String) -> Result<(), ClipboardError> {
        Self::create_or_create()
            .write()
            .as_mut()
            .ok_or(ClipboardError::NotAvailable)?
            .set_contents(contents)
            .map_err(|_| ClipboardError::FailedToSet)
    }
}
