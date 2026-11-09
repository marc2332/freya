use std::{
    fmt,
    rc::Rc,
};

use freya_core::prelude::*;
use futures_util::future::LocalBoxFuture;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ClipboardError {
    FailedToRead,
    FailedToSet,
    NotAvailable,
    UnsupportedFormat,
}

impl fmt::Display for ClipboardError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::FailedToRead => "Could not read the clipboard",
            Self::FailedToSet => "Could not write to the clipboard",
            Self::NotAvailable => "The clipboard is unavailable",
            Self::UnsupportedFormat => "The requested clipboard format is unavailable",
        })
    }
}

impl std::error::Error for ClipboardError {}

pub type ClipboardFuture<T> = LocalBoxFuture<'static, Result<T, ClipboardError>>;

/// Starts requests immediately and returns owned futures for their results.
pub trait ClipboardProvider {
    fn get(&self, format: DataFormat) -> ClipboardFuture<DataValue>;
    fn set(&self, data: TransferData) -> ClipboardFuture<()>;
}

/// The clipboard provider registered by the renderer.
#[derive(Clone, Copy, PartialEq)]
pub struct Clipboard(State<Option<Rc<dyn ClipboardProvider>>>);

impl Clipboard {
    pub fn create(provider: Option<Box<dyn ClipboardProvider>>) -> Self {
        Self(State::create_global(provider.map(Rc::from)))
    }

    /// Reads the requested format from the clipboard.
    pub fn get(format: DataFormat) -> ClipboardFuture<DataValue> {
        match Self::provider() {
            Ok(provider) => provider.get(format),
            Err(error) => Box::pin(async move { Err(error) }),
        }
    }

    /// Replaces the clipboard with data offered in one or more formats.
    pub fn set(data: TransferData) -> ClipboardFuture<()> {
        match Self::provider() {
            Ok(provider) => provider.set(data),
            Err(error) => Box::pin(async move { Err(error) }),
        }
    }

    /// Starts a text clipboard read and returns its asynchronous result.
    pub fn get_text() -> ClipboardFuture<String> {
        let read = Self::get(DataFormat::Text);
        Box::pin(async move {
            match read.await? {
                DataValue::Text(text) => Ok(text),
                DataValue::Bytes(bytes) => {
                    String::from_utf8(bytes).map_err(|_| ClipboardError::FailedToRead)
                }
                DataValue::Files(_) => Err(ClipboardError::FailedToRead),
            }
        })
    }

    /// Starts a text clipboard write and returns its asynchronous result.
    pub fn set_text(contents: String) -> ClipboardFuture<()> {
        Self::set(TransferData::text(contents))
    }

    /// Writes without awaiting the result and discards any errors.
    /// Requires an active Freya task context.
    pub fn set_deferred(data: TransferData) {
        let write = Self::set(data);
        spawn_global(async move {
            let _ = write.await;
        });
    }

    /// Writes text without awaiting the result and discards any errors.
    /// Requires an active Freya task context.
    pub fn set_text_deferred(contents: String) {
        Self::set_deferred(TransferData::text(contents));
    }

    fn provider() -> Result<Rc<dyn ClipboardProvider>, ClipboardError> {
        let contexts = GlobalContexts::try_get().ok_or(ClipboardError::NotAvailable)?;
        let clipboard = contexts
            .try_get_context::<Self>()
            .ok_or(ClipboardError::NotAvailable)?;
        let provider = clipboard.0.peek().clone();
        provider.ok_or(ClipboardError::NotAvailable)
    }
}
