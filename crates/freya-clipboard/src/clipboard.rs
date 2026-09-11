//! Provides a clipboard abstraction to access the target system's clipboard.

use copypasta::ClipboardProvider;
use freya_core::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum ClipboardError {
    FailedToRead,
    FailedToSet,
    NotAvailable,
}

/// App clipboard.
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
pub struct Clipboard(State<Option<Box<dyn ClipboardProvider>>>);

impl Clipboard {
    pub fn create(provider: Option<Box<dyn ClipboardProvider>>) -> Self {
        Self(State::create_global(provider))
    }

    /// Read from the clipboard.
    #[track_caller]
    pub fn get() -> Result<String, ClipboardError> {
        Self::with_provider(|provider| provider.get_contents())?
            .map_err(|_| ClipboardError::FailedToRead)
    }

    /// Write to the clipboard.
    #[track_caller]
    pub fn set(contents: String) -> Result<(), ClipboardError> {
        Self::with_provider(|provider| provider.set_contents(contents))?
            .map_err(|_| ClipboardError::FailedToSet)
    }

    fn with_provider<T>(
        run: impl FnOnce(&mut dyn ClipboardProvider) -> T,
    ) -> Result<T, ClipboardError> {
        let mut clipboard = GlobalContexts::get().get_context::<Clipboard>();
        let mut provider = clipboard.0.write();
        let provider = provider.as_mut().ok_or(ClipboardError::NotAvailable)?;
        Ok(run(provider.as_mut()))
    }
}
