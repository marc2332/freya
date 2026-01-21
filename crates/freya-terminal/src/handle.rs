use std::{
    io::Write,
    sync::{
        Arc,
        Mutex,
    },
};

use freya_core::{
    notify::ArcNotify,
    prelude::{
        TaskHandle,
        UseId,
    },
};
use futures_channel::mpsc::UnboundedSender;

use crate::{
    buffer::TerminalBuffer,
    pty::spawn_pty,
};

type ResizeSender = Arc<UnboundedSender<(u16, u16)>>;

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

/// Internal cleanup handler for terminal resources.
pub(crate) struct TerminalCleaner {
    /// Writer handle for the PTY.
    pub(crate) writer: Arc<Mutex<Option<Box<dyn Write + Send>>>>,
    /// Task handle for the terminal reader task.
    pub(crate) task: TaskHandle,
    /// Notifier that signals when the terminal should close.
    pub(crate) closer_notifier: ArcNotify,
}

impl Drop for TerminalCleaner {
    fn drop(&mut self) {
        *self.writer.lock().unwrap() = None;
        self.task.try_cancel();
        self.closer_notifier.notify();
    }
}

/// Handle to a running terminal instance.
///
/// The handle allows you to write input to the terminal and resize it.
/// Multiple Terminal components can share the same handle.
///
/// The PTY is automatically closed when the handle is dropped.
#[derive(Clone)]
#[allow(dead_code)]
pub struct TerminalHandle {
    /// Unique identifier for this terminal instance.
    pub(crate) id: TerminalId,
    /// Terminal buffer containing the current screen state.
    pub(crate) buffer: Arc<Mutex<TerminalBuffer>>,
    /// Writer for sending input to the PTY process.
    pub(crate) writer: Arc<Mutex<Option<Box<dyn Write + Send>>>>,
    /// Channel for sending resize events to the PTY.
    pub(crate) resize_sender: ResizeSender,
    /// Notifier that signals when the terminal/PTY closes.
    pub(crate) closer_notifier: ArcNotify,
    /// Handles cleanup when the terminal is dropped.
    pub(crate) cleaner: Arc<TerminalCleaner>,
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
        let _ = self.resize_sender.unbounded_send((rows, cols));
    }

    /// Read the current terminal buffer.
    pub fn read_buffer(&self) -> TerminalBuffer {
        self.buffer.lock().unwrap().clone()
    }

    /// Returns a future that completes when the terminal/PTY closes.
    ///
    /// This can be used to detect when the shell process exits and update the UI accordingly.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use_future(move || async move {
    ///     terminal_handle.closed().await;
    ///     // Terminal has exited, update UI state
    /// });
    /// ```
    pub fn closed(&self) -> impl std::future::Future<Output = ()> + '_ {
        self.closer_notifier.notified()
    }

    /// Returns the unique identifier for this terminal instance.
    pub fn id(&self) -> TerminalId {
        self.id
    }
}
