//! Terminal emulator integration for Freya.
//!
//! This crate provides a way to embed interactive terminal emulators in your Freya applications.
//! It uses PTY (pseudo-terminal) to spawn shell processes and renders VT100-compatible terminal output.
//!
//! # Example
//!
//! ```rust,no_run
//! use freya::prelude::*;
//! use freya_terminal::prelude::*;
//! use portable_pty::CommandBuilder;
//!
//! fn main() {
//!     launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
//! }
//!
//! fn app() -> impl IntoElement {
//!     let mut handle = use_state(|| {
//!         let mut cmd = CommandBuilder::new("bash");
//!         cmd.env("TERM", "xterm-256color");
//!         TerminalHandle::new(cmd).unwrap()
//!     });
//!
//!     rect()
//!         .expanded()
//!         .background((30, 30, 30))
//!         .child(Terminal::with_handle((*handle).clone()))
//! }
//! ```

pub mod buffer;
pub mod colors;
mod element;
mod handle;
mod parser;
mod pty;

pub use handle::TerminalHandle;

pub mod component {
    pub use crate::element::{
        Terminal,
        TerminalElement,
    };
}

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::{
        buffer::TerminalBuffer,
        component::{
            Terminal,
            TerminalElement,
        },
        handle::TerminalHandle,
    };
}
