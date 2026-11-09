//! Asynchronous clipboard access through a renderer-provided backend.
//!
//! ```rust,no_run
//! use freya::clipboard::{
//!     Clipboard,
//!     ClipboardError,
//!     DataFormat,
//!     DataValue,
//!     TransferData,
//! };
//!
//! async fn copy() -> Result<(), ClipboardError> {
//!     Clipboard::set(
//!         TransferData::text("Hello")
//!             .with(DataFormat::Html, DataValue::Text("<b>Hello</b>".into())),
//!     )
//!     .await
//! }
//!
//! async fn read_html() -> Result<DataValue, ClipboardError> {
//!     Clipboard::get(DataFormat::Html).await
//! }
//! ```

#[cfg(target_os = "android")]
pub mod android;
pub mod clipboard;
pub mod memory;

pub mod prelude {
    pub use freya_core::data_transfer::{
        DataFormat,
        DataValue,
        TransferData,
    };

    pub use crate::{
        clipboard::*,
        memory::MemoryClipboard,
    };
}
