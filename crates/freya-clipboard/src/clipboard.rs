//! Provides a clipboard abstraction to access the target system's clipboard.

use std::{
    cell::RefCell,
    rc::Rc,
};

use copypasta::ClipboardProvider;
use freya_core::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum ClipboardError {
    FailedToRead,
    FailedToSet,
    NotAvailable,
}

/// Clipboard shared by all windows through [GlobalContexts].
#[derive(Clone)]
pub struct GlobalClipboard(Rc<RefCell<Option<Box<dyn ClipboardProvider>>>>);

impl GlobalClipboard {
    pub fn new(provider: Option<Box<dyn ClipboardProvider>>) -> Self {
        Self(Rc::new(RefCell::new(provider)))
    }

    fn with_provider<T>(
        &self,
        run: impl FnOnce(&mut dyn ClipboardProvider) -> T,
    ) -> Result<T, ClipboardError> {
        let mut provider = self.0.borrow_mut();
        let provider = provider.as_mut().ok_or(ClipboardError::NotAvailable)?;
        Ok(run(provider.as_mut()))
    }
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
/// Clipboard::set("Hello, Freya!".to_string());
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct Clipboard;

impl Clipboard {
    // Read from the clipboard
    #[track_caller]
    pub fn get() -> Result<String, ClipboardError> {
        GlobalContexts::get()
            .get_context::<GlobalClipboard>()
            .with_provider(|provider| provider.get_contents())?
            .map_err(|_| ClipboardError::FailedToRead)
    }

    // Write to the clipboard
    #[track_caller]
    pub fn set(contents: String) -> Result<(), ClipboardError> {
        GlobalContexts::get()
            .get_context::<GlobalClipboard>()
            .with_provider(|provider| provider.set_contents(contents))?
            .map_err(|_| ClipboardError::FailedToSet)
    }
}
