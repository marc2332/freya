use std::{
    fmt,
    future::Future,
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

/// Methods for reading from and writing to the clipboard.
pub trait ClipboardProvider {
    fn get(&self, format: DataFormat) -> ClipboardFuture<DataValue>;
    fn set(&self, data: TransferData) -> ClipboardFuture<()>;
}

/// Clipboard integration for Freya apps.
///
/// # Examples
///
/// ```no_run
/// use freya_clipboard::prelude::{
///     Clipboard,
///     ClipboardError,
/// };
///
/// async fn read_and_write_clipboard() -> Result<(), ClipboardError> {
///     let content = Clipboard::get_text().await?;
///     println!("{content}");
///     Clipboard::set_text("Hello, Freya!".to_string()).await
/// }
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct Clipboard(State<Rc<dyn ClipboardProvider>>);

impl Clipboard {
    #[doc(hidden)]
    pub fn create(provider: Box<dyn ClipboardProvider>) -> Self {
        Self(State::create_global(Rc::from(provider)))
    }

    /// Reads the requested format from the clipboard.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use freya_clipboard::prelude::{
    ///     Clipboard,
    ///     ClipboardError,
    ///     DataFormat,
    ///     DataValue,
    /// };
    ///
    /// async fn read_html() -> Result<DataValue, ClipboardError> {
    ///     Clipboard::get(DataFormat::Html).await
    /// }
    /// ```
    #[track_caller]
    pub fn get(format: DataFormat) -> ClipboardFuture<DataValue> {
        Self::provider().get(format)
    }

    /// Replaces the clipboard with data offered in one or more formats.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use freya_clipboard::prelude::{
    ///     Clipboard,
    ///     ClipboardError,
    ///     DataFormat,
    ///     DataValue,
    ///     TransferData,
    /// };
    ///
    /// async fn copy_greeting() -> Result<(), ClipboardError> {
    ///     let data = TransferData::text("Hello, Freya!").with(
    ///         DataFormat::Html,
    ///         DataValue::Text("<strong>Hello, Freya!</strong>".into()),
    ///     );
    ///     Clipboard::set(data).await
    /// }
    /// ```
    #[track_caller]
    pub fn set(data: TransferData) -> ClipboardFuture<()> {
        Self::provider().set(data)
    }

    /// Starts a text clipboard read and returns its asynchronous result.
    pub fn get_text() -> impl Future<Output = Result<String, ClipboardError>> {
        let read = Self::get(DataFormat::Text);
        async move {
            match read.await? {
                DataValue::Text(text) => Ok(text),
                DataValue::Bytes(bytes) => {
                    String::from_utf8(bytes).map_err(|_| ClipboardError::FailedToRead)
                }
                DataValue::Files(_) => Err(ClipboardError::FailedToRead),
            }
        }
    }

    /// Starts a text clipboard write and returns its asynchronous result.
    pub fn set_text(contents: String) -> ClipboardFuture<()> {
        Self::set(TransferData::text(contents))
    }

    /// Writes without awaiting the result and discards any errors.
    pub fn set_deferred(data: TransferData) {
        let write = Self::set(data);
        spawn_global(async move {
            let _ = write.await;
        });
    }

    /// Writes text without awaiting the result and discards any errors.
    pub fn set_text_deferred(contents: String) {
        Self::set_deferred(TransferData::text(contents));
    }

    #[track_caller]
    fn provider() -> Rc<dyn ClipboardProvider> {
        let clipboard = GlobalContexts::get()
            .try_get_context::<Self>()
            .expect("Clipboard access requires a registered clipboard provider.");
        clipboard.0.peek().clone()
    }
}
