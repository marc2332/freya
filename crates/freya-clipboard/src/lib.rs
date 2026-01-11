//! Clipboard utilities to read and write the system clipboard.
//!
//! This crate wraps [copypasta] and exposes a small, ergonomic API to access
//! the clipboard from Freya applications and tests. See [Clipboard](clipboard::Clipboard) in
//! `clipboard.rs` for usage examples.
//!
//! This crate is reexported in `freya::clipboard`.
//!
//! # Examples
//!
//! ```rust, no_run
//! use freya::clipboard::Clipboard;
//!
//! // Read the clipboard content
//! if let Ok(text) = Clipboard::get() {
//!     println!("clipboard: {}", text);
//! }
//!
//! // Write to the clipboard
//! let _ = Clipboard::set("Hello, Freya!".to_string());
//! ```

pub mod clipboard;
pub use copypasta;

pub mod prelude {
    pub use crate::clipboard::*;
}
