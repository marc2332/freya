use std::{
    cell::RefCell,
    io::Write,
    rc::Rc,
    sync::Arc,
};

use freya_core::{
    notify::ArcNotify,
    prelude::{
        Platform,
        TaskHandle,
        UseId,
        UserEvent,
    },
};
use futures_channel::mpsc::UnboundedSender;

use crate::{
    buffer::{
        TerminalBuffer,
        TerminalSelection,
    },
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
    pub(crate) writer: Rc<RefCell<Option<Box<dyn Write + Send>>>>,
    /// Async tasks
    pub(crate) reader_task: TaskHandle,
    pub(crate) pty_task: TaskHandle,
    /// Notifier that signals when the terminal should close.
    pub(crate) closer_notifier: ArcNotify,
}

impl Drop for TerminalCleaner {
    fn drop(&mut self) {
        *self.writer.borrow_mut() = None;
        self.reader_task.try_cancel();
        self.pty_task.try_cancel();
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
    pub(crate) buffer: Rc<RefCell<TerminalBuffer>>,
    /// Writer for sending input to the PTY process.
    pub(crate) writer: Rc<RefCell<Option<Box<dyn Write + Send>>>>,
    /// Channel for sending resize events to the PTY.
    pub(crate) resize_sender: ResizeSender,
    /// Notifier that signals when the terminal/PTY closes.
    pub(crate) closer_notifier: ArcNotify,
    /// Handles cleanup when the terminal is dropped.
    pub(crate) cleaner: Rc<TerminalCleaner>,
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
    /// let handle = TerminalHandle::new(TerminalId::new(), cmd).unwrap();
    /// ```
    pub fn new(
        id: TerminalId,
        command: portable_pty::CommandBuilder,
    ) -> Result<Self, TerminalError> {
        spawn_pty(id, command)
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
        match &mut *self.writer.borrow_mut() {
            Some(w) => {
                w.write_all(data)
                    .map_err(|e| TerminalError::WriteError(e.to_string()))?;
                w.flush()
                    .map_err(|e| TerminalError::WriteError(e.to_string()))?;
                Ok(())
            }
            None => Err(TerminalError::NotInitialized),
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
        self.buffer.borrow().clone()
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

    /// Get the current text selection.
    pub fn get_selection(&self) -> Option<TerminalSelection> {
        self.buffer.borrow().selection.clone()
    }

    /// Set the text selection.
    pub fn set_selection(&self, selection: Option<TerminalSelection>) {
        self.buffer.borrow_mut().selection = selection;
    }

    /// Start a new selection at the given position.
    pub fn start_selection(&self, row: usize, col: usize) {
        let mut selection = TerminalSelection::new(row, col, row, col);
        selection.dragging = true;
        self.buffer.borrow_mut().selection = Some(selection);
        Platform::get().send(UserEvent::RequestRedraw);
    }

    pub fn update_selection(&self, row: usize, col: usize) {
        if let Some(selection) = &mut self.buffer.borrow_mut().selection
            && selection.dragging
        {
            let mut new_selection =
                TerminalSelection::new(selection.start_row, selection.start_col, row, col);
            new_selection.dragging = true;
            *selection = new_selection;
            Platform::get().send(UserEvent::RequestRedraw);
        }
    }

    pub fn end_selection(&self) {
        if let Some(selection) = &mut self.buffer.borrow_mut().selection {
            selection.dragging = false;
            Platform::get().send(UserEvent::RequestRedraw);
        }
    }

    /// Clear the current selection.
    pub fn clear_selection(&self) {
        self.buffer.borrow_mut().selection = None;
        Platform::get().send(UserEvent::RequestRedraw);
    }

    /// Get selected text from the buffer.
    pub fn get_selected_text(&self) -> Option<String> {
        self.buffer.borrow_mut().get_selected_text()
    }
}
