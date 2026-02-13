use std::{
    cell::{
        Ref,
        RefCell,
    },
    io::Write,
    rc::Rc,
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
use vt100::Parser;

use crate::{
    buffer::{
        TerminalBuffer,
        TerminalSelection,
    },
    pty::{
        ScrollCommand,
        spawn_pty,
    },
};

type ResizeSender = Rc<UnboundedSender<(u16, u16)>>;
type ScrollSender = Rc<UnboundedSender<ScrollCommand>>;

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
    /// VT100 parser for accessing full scrollback content.
    pub(crate) parser: Rc<RefCell<Parser>>,
    /// Writer for sending input to the PTY process.
    pub(crate) writer: Rc<RefCell<Option<Box<dyn Write + Send>>>>,
    /// Channel for sending resize events to the PTY.
    pub(crate) resize_sender: ResizeSender,
    /// Channel for sending scroll commands to the PTY.
    pub(crate) scroll_sender: ScrollSender,
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
    /// Create a new terminal with the specified command and default scrollback size (1000 lines).
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
    /// let handle = TerminalHandle::new(TerminalId::new(), cmd, None).unwrap();
    /// ```
    pub fn new(
        id: TerminalId,
        command: portable_pty::CommandBuilder,
        scrollback_length: Option<usize>,
    ) -> Result<Self, TerminalError> {
        spawn_pty(id, command, scrollback_length.unwrap_or(1000))
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
                {
                    let mut buf = self.buffer.borrow_mut();
                    buf.selection = None;
                    buf.scroll_offset = 0;
                }
                let _ = self.scroll_sender.unbounded_send(ScrollCommand::ToBottom);
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

    /// Scroll the terminal by the specified delta.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya_terminal::prelude::*;
    /// # let handle: TerminalHandle = unimplemented!();
    /// handle.scroll(-3); // Scroll up 3 lines
    /// handle.scroll(3); // Scroll down 3 lines
    /// ```
    pub fn scroll(&self, delta: i32) {
        let mut buffer = self.buffer.borrow_mut();
        let new_offset = (buffer.scroll_offset as i64 + delta as i64).max(0) as usize;
        buffer.scroll_offset = new_offset.min(buffer.total_scrollback);
        let _ = self
            .scroll_sender
            .unbounded_send(ScrollCommand::Delta(delta));
    }

    /// Scroll the terminal to the bottom.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya_terminal::prelude::*;
    /// # let handle: TerminalHandle = unimplemented!();
    /// handle.scroll_to_bottom();
    /// ```
    pub fn scroll_to_bottom(&self) {
        self.buffer.borrow_mut().scroll_offset = 0;
        let _ = self.scroll_sender.unbounded_send(ScrollCommand::ToBottom);
    }

    /// Get the current scrollback position (scroll offset from buffer).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya_terminal::prelude::*;
    /// # let handle: TerminalHandle = unimplemented!();
    /// let position = handle.scrollback_position();
    /// ```
    pub fn scrollback_position(&self) -> usize {
        self.buffer.borrow().scroll_offset
    }

    /// Read the current terminal buffer.
    pub fn read_buffer(&'_ self) -> Ref<'_, TerminalBuffer> {
        self.buffer.borrow()
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

    pub fn start_selection(&self, row: usize, col: usize) {
        let mut buffer = self.buffer.borrow_mut();
        let scroll = buffer.scroll_offset;
        buffer.selection = Some(TerminalSelection {
            dragging: true,
            start_row: row,
            start_col: col,
            start_scroll: scroll,
            end_row: row,
            end_col: col,
            end_scroll: scroll,
        });
        Platform::get().send(UserEvent::RequestRedraw);
    }

    pub fn update_selection(&self, row: usize, col: usize) {
        let mut buffer = self.buffer.borrow_mut();
        let scroll = buffer.scroll_offset;
        if let Some(selection) = &mut buffer.selection
            && selection.dragging
        {
            selection.end_row = row;
            selection.end_col = col;
            selection.end_scroll = scroll;
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

    pub fn get_selected_text(&self) -> Option<String> {
        let buffer = self.buffer.borrow();
        let selection = buffer.selection.clone()?;
        if selection.is_empty() {
            return None;
        }

        let scroll = buffer.scroll_offset;
        let (display_start, start_col, display_end, end_col) = selection.display_positions(scroll);

        let mut parser = self.parser.borrow_mut();
        let saved_scrollback = parser.screen().scrollback();
        let (_rows, cols) = parser.screen().size();

        let mut lines = Vec::new();

        for d in display_start..=display_end {
            let cp = d - scroll as i64;
            let needed_scrollback = (-cp).max(0) as usize;
            let viewport_row = cp.max(0) as u16;

            parser.screen_mut().set_scrollback(needed_scrollback);

            let row_cells: Vec<_> = (0..cols)
                .filter_map(|c| parser.screen().cell(viewport_row, c).cloned())
                .collect();

            let is_single = display_start == display_end;
            let is_first = d == display_start;
            let is_last = d == display_end;

            let cells = if is_single {
                let s = start_col.min(row_cells.len());
                let e = end_col.min(row_cells.len());
                &row_cells[s..e]
            } else if is_first {
                let s = start_col.min(row_cells.len());
                &row_cells[s..]
            } else if is_last {
                &row_cells[..end_col.min(row_cells.len())]
            } else {
                &row_cells
            };

            let line: String = cells
                .iter()
                .map(|cell| {
                    if cell.has_contents() {
                        cell.contents()
                    } else {
                        " "
                    }
                })
                .collect::<String>();

            lines.push(line);
        }

        parser.screen_mut().set_scrollback(saved_scrollback);

        Some(lines.join("\n"))
    }
}
