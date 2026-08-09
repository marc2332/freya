//! Clipboard utilities to read and write the system clipboard.
//!
//! This crate exposes a small, ergonomic API to access the clipboard from Freya applications and
//! tests. See [Clipboard](clipboard::Clipboard) in `clipboard.rs` for usage examples.
//!
//! The platform integration provides a
//! [ClipboardProvider](clipboard::ClipboardProvider) into the root context and
//! [Clipboard](clipboard::Clipboard) reads it from there, so a host that speaks to something
//! other than the desktop clipboard has a seam to speak through.
//! [ClipboardContext](clipboard::ClipboardContext) is the desktop one, over [arboard].
//!
//! Text and images share that one provider, because they are one clipboard: a second backend
//! beside it is a second connection claiming the same selection.
//!
//! This crate is reexported in `freya::clipboard`.
//!
//! # Examples
//!
//! ```rust, no_run
//! use freya::clipboard::{
//!     Clipboard,
//!     ClipboardImage,
//! };
//!
//! // Read the clipboard content
//! if let Ok(text) = Clipboard::get() {
//!     println!("clipboard: {}", text);
//! }
//!
//! // Write to the clipboard
//! let _ = Clipboard::set("Hello, Freya!".to_string());
//!
//! // Write an image to the clipboard: one red pixel
//! let _ = Clipboard::set_image(ClipboardImage {
//!     width: 1,
//!     height: 1,
//!     rgba: vec![255, 0, 0, 255],
//! });
//! ```

pub mod clipboard;

pub mod prelude {
    pub use crate::clipboard::*;
}
