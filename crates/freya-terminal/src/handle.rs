use std::{
    cell::{
        Ref,
        RefCell,
    },
    io::Write,
    rc::Rc,
    time::Instant,
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
    parser::{
        TerminalMouseButton,
        encode_mouse_move,
        encode_mouse_press,
        encode_mouse_release,
        encode_wheel_event,
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
    /// Notifier that signals when new output is received from the PTY.
    pub(crate) output_notifier: ArcNotify,
    /// Tracks when user last wrote input to the PTY.
    pub(crate) last_write_time: Rc<RefCell<Instant>>,
    /// Currently pressed mouse button (for drag/motion tracking).
    pub(crate) pressed_button: Rc<RefCell<Option<TerminalMouseButton>>>,
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
        self.write_raw(data)?;
        let mut buffer = self.buffer.borrow_mut();
        buffer.selection = None;
        buffer.scroll_offset = 0;
        *self.last_write_time.borrow_mut() = Instant::now();
        let _ = self.scroll_sender.unbounded_send(ScrollCommand::ToBottom);
        Ok(())
    }

    /// Write data to the PTY without resetting scroll or selection state.
    fn write_raw(&self, data: &[u8]) -> Result<(), TerminalError> {
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

    /// Send a wheel event to the PTY.
    ///
    /// This sends mouse wheel events as escape sequences to the running process.
    /// Uses the currently active mouse protocol encoding based on what
    /// the application has requested via DECSET sequences.
    pub fn send_wheel_to_pty(&self, row: usize, col: usize, delta_y: f64) {
        let encoding = self.parser.borrow().screen().mouse_protocol_encoding();
        let seq = encode_wheel_event(row, col, delta_y, encoding);
        let _ = self.write_raw(seq.as_bytes());
    }

    /// Send a mouse move/drag event to the PTY based on the active mouse mode.
    ///
    /// - `AnyMotion` (DECSET 1003): sends motion events regardless of button state.
    /// - `ButtonMotion` (DECSET 1002): sends motion events only while a button is held.
    ///
    /// When dragging, the held button is encoded in the motion event so TUI apps
    /// can implement their own text selection.
    pub fn mouse_move(&self, row: usize, col: usize) {
        let parser = self.parser.borrow();
        let mouse_mode = parser.screen().mouse_protocol_mode();
        let encoding = parser.screen().mouse_protocol_encoding();

        let held = *self.pressed_button.borrow();

        match mouse_mode {
            vt100::MouseProtocolMode::AnyMotion => {
                let seq = encode_mouse_move(row, col, held, encoding);
                let _ = self.write_raw(seq.as_bytes());
            }
            vt100::MouseProtocolMode::ButtonMotion => {
                if let Some(button) = held {
                    let seq = encode_mouse_move(row, col, Some(button), encoding);
                    let _ = self.write_raw(seq.as_bytes());
                }
            }
            _ => {}
        }
    }

    /// Returns whether the running application has enabled mouse tracking.
    fn is_mouse_tracking_enabled(&self) -> bool {
        let parser = self.parser.borrow();
        parser.screen().mouse_protocol_mode() != vt100::MouseProtocolMode::None
    }

    /// Handle a mouse button press event.
    ///
    /// When the running application has enabled mouse tracking (e.g. vim,
    /// helix, htop), this sends the press escape sequence to the PTY.
    /// Otherwise it starts a text selection.
    pub fn mouse_down(&self, row: usize, col: usize, button: TerminalMouseButton) {
        *self.pressed_button.borrow_mut() = Some(button);

        if self.is_mouse_tracking_enabled() {
            let encoding = self.parser.borrow().screen().mouse_protocol_encoding();
            let seq = encode_mouse_press(row, col, button, encoding);
            let _ = self.write_raw(seq.as_bytes());
        } else {
            self.start_selection(row, col);
        }
    }

    /// Handle a mouse button release event.
    ///
    /// When the running application has enabled mouse tracking, this sends the
    /// release escape sequence to the PTY. Only `PressRelease`, `ButtonMotion`,
    /// and `AnyMotion` modes receive release events — `Press` mode does not.
    /// Otherwise it ends the current text selection.
    pub fn mouse_up(&self, row: usize, col: usize, button: TerminalMouseButton) {
        *self.pressed_button.borrow_mut() = None;

        let parser = self.parser.borrow();
        let mouse_mode = parser.screen().mouse_protocol_mode();
        let encoding = parser.screen().mouse_protocol_encoding();

        match mouse_mode {
            vt100::MouseProtocolMode::PressRelease
            | vt100::MouseProtocolMode::ButtonMotion
            | vt100::MouseProtocolMode::AnyMotion => {
                let seq = encode_mouse_release(row, col, button, encoding);
                let _ = self.write_raw(seq.as_bytes());
            }
            vt100::MouseProtocolMode::Press => {
                // Press-only mode doesn't send release events
            }
            vt100::MouseProtocolMode::None => {
                self.end_selection();
            }
        }
    }

    /// Number of arrow key presses to send per wheel tick in alternate scroll mode.
    const ALTERNATE_SCROLL_LINES: usize = 3;

    /// Handle a wheel event intelligently.
    ///
    /// The behavior depends on the terminal state:
    /// - If viewing scrollback history: scrolls the scrollback buffer.
    /// - If mouse tracking is enabled (e.g., vim, helix): sends wheel escape
    ///   sequences to the PTY.
    /// - If on the alternate screen without mouse tracking (e.g., gitui, less):
    ///   sends arrow key sequences to the PTY (alternate scroll mode, like
    ///   wezterm/kitty/alacritty).
    /// - Otherwise (normal shell): scrolls the scrollback buffer.
    pub fn wheel(&self, delta_y: f64, row: usize, col: usize) {
        let scroll_delta = if delta_y > 0.0 { 3 } else { -3 };
        let scroll_offset = self.buffer.borrow().scroll_offset;
        let (mouse_mode, alt_screen, app_cursor) = {
            let parser = self.parser.borrow();
            let screen = parser.screen();
            (
                screen.mouse_protocol_mode(),
                screen.alternate_screen(),
                screen.application_cursor(),
            )
        };

        if scroll_offset > 0 {
            // User is viewing scrollback history
            let delta = scroll_delta;
            self.scroll(delta);
        } else if mouse_mode != vt100::MouseProtocolMode::None {
            // App has enabled mouse tracking (vim, helix, etc.)
            self.send_wheel_to_pty(row, col, delta_y);
        } else if alt_screen {
            // Alternate screen without mouse tracking (gitui, less, etc.)
            // Send arrow key presses, matching wezterm/kitty/alacritty behavior
            let key = match (delta_y > 0.0, app_cursor) {
                (true, true) => "\x1bOA",
                (true, false) => "\x1b[A",
                (false, true) => "\x1bOB",
                (false, false) => "\x1b[B",
            };
            for _ in 0..Self::ALTERNATE_SCROLL_LINES {
                let _ = self.write_raw(key.as_bytes());
            }
        } else {
            // Normal screen, no mouse tracking — scroll scrollback
            let delta = scroll_delta;
            self.scroll(delta);
        }
    }

    /// Read the current terminal buffer.
    pub fn read_buffer(&'_ self) -> Ref<'_, TerminalBuffer> {
        self.buffer.borrow()
    }

    /// Returns a future that completes when new output is received from the PTY.
    ///
    /// Can be called repeatedly in a loop to detect ongoing output activity.
    pub fn output_received(&self) -> impl std::future::Future<Output = ()> + '_ {
        self.output_notifier.notified()
    }

    pub fn last_write_elapsed(&self) -> std::time::Duration {
        self.last_write_time.borrow().elapsed()
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
