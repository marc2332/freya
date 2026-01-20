use std::{
    io::Write,
    sync::{
        Arc,
        Mutex,
    },
};

use freya_core::prelude::UseId;
use futures_channel::mpsc::UnboundedSender;

use crate::{
    buffer::TerminalBuffer,
    pty::spawn_pty,
};

/// Type alias for the resize sender channel
type ResizeSender = Arc<Mutex<Option<UnboundedSender<(u16, u16)>>>>;

/// Unique identifier for a terminal instance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalId(pub usize);

impl TerminalId {
    pub fn new() -> Self {
        Self(UseId::<TerminalId>::get_in_hook())
    }
}

impl Default for TerminalId {
    fn default() -> Self {
        Self::new()
    }
}

/// Error type for terminal operations
#[derive(Debug, thiserror::Error)]
pub enum TerminalError {
    #[error("PTY error: {0}")]
    PtyError(String),

    #[error("Write error: {0}")]
    WriteError(String),

    #[error("Terminal not initialized")]
    NotInitialized,
}

/// Handle to a running terminal instance.
///
/// The handle allows you to write input to the terminal and resize it.
/// Multiple Terminal components can share the same handle.
///
/// The PTY is automatically closed when the handle is dropped.
#[derive(Clone)]
pub struct TerminalHandle {
    /// Unique identifier for this terminal
    pub id: TerminalId,
    /// Terminal buffer containing current state
    pub buffer: Arc<Mutex<TerminalBuffer>>,
    /// Writer for sending input to the PTY
    pub writer: Arc<Mutex<Option<Box<dyn Write + Send>>>>,
    /// Channel for notifying UI and PTY of resizing
    pub resize_holder: ResizeSender,
}

impl PartialEq for TerminalHandle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl TerminalHandle {
    /// Create a new terminal with the specified command.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use freya_terminal::prelude::*;
    /// use portable_pty::CommandBuilder;
    ///
    /// let mut cmd = CommandBuilder::new("bash");
    /// cmd.env("TERM", "xterm-256color");
    ///
    /// let handle = TerminalHandle::new(cmd).unwrap();
    /// ```
    pub fn new(command: portable_pty::CommandBuilder) -> Result<Self, TerminalError> {
        spawn_pty(command)
    }

    /// Write data to the terminal.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya_terminal::prelude::*;
    /// # let handle: TerminalHandle = unimplemented!();
    /// handle.write(b"ls -la\n").unwrap();
    /// ```
    pub fn write(&self, data: &[u8]) -> Result<(), TerminalError> {
        match self.writer.lock() {
            Ok(mut guard) => match guard.as_mut() {
                Some(w) => {
                    w.write_all(data)
                        .map_err(|e| TerminalError::WriteError(e.to_string()))?;
                    w.flush()
                        .map_err(|e| TerminalError::WriteError(e.to_string()))?;
                    Ok(())
                }
                None => Err(TerminalError::NotInitialized),
            },
            Err(_) => Err(TerminalError::WriteError("Lock poisoned".to_string())),
        }
    }

    /// Resize the terminal to the specified rows and columns.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya_terminal::prelude::*;
    /// # let handle: TerminalHandle = unimplemented!();
    /// handle.resize(24, 80);
    /// ```
    pub fn resize(&self, rows: u16, cols: u16) {
        // Notify UI thread to update parser
        if let Ok(holder) = self.resize_holder.lock()
            && let Some(tx) = holder.as_ref()
        {
            let _ = tx.unbounded_send((rows, cols));
        }
    }

    /// Read the current terminal buffer.
    pub fn read_buffer(&self) -> TerminalBuffer {
        self.buffer.lock().unwrap().clone()
    }
}

impl Drop for TerminalHandle {
    fn drop(&mut self) {
        // PTY is automatically cleaned up when the writer is dropped
        // The spawned threads also end when their channels close
        *self.writer.lock().unwrap() = None;
    }
}
