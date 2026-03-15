//! # Freya Terminal 🖥️
//!
//! Terminal emulator integration for Freya applications.
//!
//! This crate provides a way to embed interactive terminal emulators in your Freya applications.
//! It uses PTY (pseudo-terminal) to spawn shell processes and renders VT100-compatible terminal output
//! with full 256-color support.
//!
//! ## Features
//!
//! - **PTY Integration**: Spawn and interact with shell processes
//! - **VT100 Rendering**: Full terminal emulation with cursor, colors, and text attributes
//! - **256-Color Support**: ANSI 16 colors, 6x6x6 RGB cube, and 24-level grayscale
//! - **Keyboard Input**: Handle all standard terminal key sequences
//! - **Auto-resize**: Terminal automatically resizes based on available space
//!
//! ## Basic Usage
//!
//! ```rust,no_run
//! use freya::prelude::*;
//! use freya_terminal::prelude::*;
//!
//! fn main() {
//!     launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
//! }
//!
//! fn app() -> impl IntoElement {
//!     let mut handle = use_state(|| {
//!         let mut cmd = CommandBuilder::new("bash");
//!         cmd.env("TERM", "xterm-256color");
//!         TerminalHandle::new(TerminalId::new(), cmd, None).ok()
//!     });
//!
//!     let focus = use_focus();
//!
//!     rect().expanded().background((30, 30, 30)).child(
//!         if let Some(handle) = handle.read().clone() {
//!             rect()
//!                 .child(
//!                     Terminal::new(handle.clone())
//!                         .a11y_id(focus.a11y_id())
//!                         .on_mouse_down(move |_| focus.request_focus())
//!                         .on_key_down(move |e: Event<KeyboardEventData>| {
//!                             let _ = handle.write_key(&e.key, e.modifiers);
//!                         }),
//!                 )
//!                 .expanded()
//!                 .into_element()
//!         } else {
//!             "Failed to start Terminal.".into_element()
//!         },
//!     )
//! }
//! ```
//!
//! ## Handling Terminal Exit
//!
//! You can detect when the terminal/PTY closes using `TerminalHandle::closed`:
//!
//! ```rust,ignore
//! use_future(move || async move {
//!     terminal_handle.closed().await;
//!     // Terminal has exited, update UI state
//! });
//! ```
//!
//! ## Advance usage
//!
//! Check the `feature_terminal.rs` example in the repository.
pub mod buffer;
pub mod colors;
pub mod element;
pub mod handle;
pub mod parser;
pub mod pty;

/// Prelude module for convenient imports.
pub mod prelude {
    pub use portable_pty::CommandBuilder;

    pub use crate::{
        buffer::{
            TerminalBuffer,
            TerminalSelection,
        },
        element::Terminal,
        handle::{
            TerminalError,
            TerminalHandle,
            TerminalId,
        },
        parser::TerminalMouseButton,
    };
}
