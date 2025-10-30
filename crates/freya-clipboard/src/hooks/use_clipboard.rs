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

/// Handle to access the ClipboardContext.
///
/// Use it through [use_clipboard].
#[derive(Clone, Copy, PartialEq)]
pub struct UseClipboard {
    clipboard: State<Option<ClipboardContext>>,
}

impl UseClipboard {
    pub fn create() -> Self {
        let clipboard = match try_consume_root_context() {
            Some(rt) => rt,
            None => {
                let clipboard_state =
                    State::create_in_scope(ClipboardContext::new().ok(), ScopeId::ROOT);
                provide_context_for_scope_id(clipboard_state, ScopeId::ROOT);
                try_consume_root_context().unwrap()
            }
        };
        UseClipboard { clipboard }
    }

    // Read from the clipboard
    pub fn get(&mut self) -> Result<String, ClipboardError> {
        self.clipboard
            .write()
            .as_mut()
            .ok_or(ClipboardError::NotAvailable)?
            .get_contents()
            .map_err(|_| ClipboardError::FailedToRead)
    }

    // Write to the clipboard
    pub fn set(&mut self, contents: String) -> Result<(), ClipboardError> {
        self.clipboard
            .write()
            .as_mut()
            .ok_or(ClipboardError::NotAvailable)?
            .set_contents(contents)
            .map_err(|_| ClipboardError::FailedToSet)
    }
}

/// Access the clipboard.
///
/// # Examples
///
/// ```rust,no_run
/// use freya_clipboard::prelude::use_clipboard;
///
/// // Get a handle to the clipboard
/// let mut clipboard = use_clipboard();
///
/// // Read the clipboard content
/// if let Ok(content) = clipboard.get() {
///     println!("{}", content);
/// }
///
/// // Write to the clipboard
/// clipboard.set("Hello, Dioxus!".to_string());
/// ```
pub fn use_clipboard() -> UseClipboard {
    UseClipboard::create()
}
