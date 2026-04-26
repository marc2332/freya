use std::{
    cell::RefCell,
    io::Write,
    path::PathBuf,
    rc::Rc,
    time::{
        Duration,
        Instant,
    },
};

use alacritty_terminal::{
    grid::{
        Dimensions,
        Scroll,
    },
    index::{
        Column,
        Line,
        Point,
        Side,
    },
    selection::{
        Selection,
        SelectionType,
    },
    term::{
        Term,
        TermMode,
    },
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
use keyboard_types::{
    Key,
    Modifiers,
    NamedKey,
};
use portable_pty::{
    MasterPty,
    PtySize,
};

use crate::{
    parser::{
        TerminalMouseButton,
        encode_mouse_move,
        encode_mouse_press,
        encode_mouse_release,
        encode_wheel_event,
    },
    pty::{
        EventProxy,
        TermSize,
        spawn_pty,
    },
};

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
    #[error("Write error: {0}")]
    WriteError(String),

    #[error("Terminal not initialized")]
    NotInitialized,
}

impl From<std::io::Error> for TerminalError {
    fn from(e: std::io::Error) -> Self {
        TerminalError::WriteError(e.to_string())
    }
}

/// Cleans up the PTY and the reader task when the last handle is dropped.
pub(crate) struct TerminalCleaner {
    /// Writer handle for the PTY.
    pub(crate) writer: Rc<RefCell<Option<Box<dyn Write + Send>>>>,
    /// PTY reader/parser task.
    pub(crate) pty_task: TaskHandle,
    /// Notifier that signals when the terminal should close.
    pub(crate) closer_notifier: ArcNotify,
}

/// Handle-local state grouped into a single `RefCell`.
pub(crate) struct TerminalInner {
    pub(crate) master: Box<dyn MasterPty + Send>,
    pub(crate) last_write_time: Instant,
    pub(crate) pressed_button: Option<TerminalMouseButton>,
    pub(crate) modifiers: Modifiers,
}

impl Drop for TerminalCleaner {
    fn drop(&mut self) {
        *self.writer.borrow_mut() = None;
        self.pty_task.try_cancel();
        self.closer_notifier.notify();
    }
}

/// Handle to a running terminal instance.
///
/// Multiple `Terminal` components can share the same handle. The PTY is
/// closed when the last handle is dropped.
#[derive(Clone)]
pub struct TerminalHandle {
    /// Unique identifier for this terminal instance, used for `PartialEq`.
    pub(crate) id: TerminalId,
    /// alacritty's terminal model: grid, modes, scrollback. The renderer
    /// borrows this directly during paint, so there is no parallel snapshot.
    pub(crate) term: Rc<RefCell<Term<EventProxy>>>,
    /// Writer for sending input to the PTY process.
    pub(crate) writer: Rc<RefCell<Option<Box<dyn Write + Send>>>>,
    /// Handle-local state (PTY master, input tracking).
    pub(crate) inner: Rc<RefCell<TerminalInner>>,
    /// Current working directory reported by the shell via OSC 7.
    pub(crate) cwd: Rc<RefCell<Option<PathBuf>>>,
    /// Window title reported by the shell via OSC 0 or OSC 2.
    pub(crate) title: Rc<RefCell<Option<String>>>,
    /// Notifier that signals when the terminal/PTY closes.
    pub(crate) closer_notifier: ArcNotify,
    /// Kept alive purely so its `Drop` runs when the last handle dies.
    #[allow(dead_code)]
    pub(crate) cleaner: Rc<TerminalCleaner>,
    /// Notifier that signals each time new output is received from the PTY.
    pub(crate) output_notifier: ArcNotify,
    /// Notifier that signals when the window title changes via OSC 0 or OSC 2.
    pub(crate) title_notifier: ArcNotify,
    /// Clipboard content set by the terminal app via OSC 52.
    pub(crate) clipboard_content: Rc<RefCell<Option<String>>>,
    /// Notifier that signals when clipboard content changes via OSC 52.
    pub(crate) clipboard_notifier: ArcNotify,
}

impl PartialEq for TerminalHandle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl TerminalHandle {
    /// Spawn a PTY for `command` and return a handle. Defaults to 1000 lines
    /// of scrollback when `scrollback_length` is `None`.
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

    /// Write data to the PTY. Drops any selection and snaps the viewport to the bottom.
    pub fn write(&self, data: &[u8]) -> Result<(), TerminalError> {
        self.write_raw(data)?;
        let mut term = self.term.borrow_mut();
        term.selection = None;
        term.scroll_display(Scroll::Bottom);
        self.inner.borrow_mut().last_write_time = Instant::now();
        Ok(())
    }

    /// Time since the user last wrote input to the PTY.
    pub fn last_write_elapsed(&self) -> Duration {
        self.inner.borrow().last_write_time.elapsed()
    }

    /// Write a key event to the PTY as the matching escape sequence. Returns whether it was recognised.
    pub fn write_key(&self, key: &Key, modifiers: Modifiers) -> Result<bool, TerminalError> {
        let shift = modifiers.contains(Modifiers::SHIFT);
        let ctrl = modifiers.contains(Modifiers::CONTROL);
        let alt = modifiers.contains(Modifiers::ALT);

        // CSI u / xterm modifier byte: `1 + shift + alt*2 + ctrl*4`.
        let modifier = || 1 + shift as u8 + (alt as u8) * 2 + (ctrl as u8) * 4;

        let seq: Vec<u8> = match key {
            Key::Character(ch) if ctrl && ch.len() == 1 => vec![ch.as_bytes()[0] & 0x1f],
            Key::Named(NamedKey::Enter) if shift || ctrl => {
                format!("\x1b[13;{}u", modifier()).into_bytes()
            }
            Key::Named(NamedKey::Enter) => b"\r".to_vec(),
            Key::Named(NamedKey::Backspace) if ctrl => vec![0x08],
            Key::Named(NamedKey::Backspace) if alt => vec![0x1b, 0x7f],
            Key::Named(NamedKey::Backspace) => vec![0x7f],
            Key::Named(NamedKey::Delete) if alt || ctrl || shift => {
                format!("\x1b[3;{}~", modifier()).into_bytes()
            }
            Key::Named(NamedKey::Delete) => b"\x1b[3~".to_vec(),
            Key::Named(NamedKey::Tab) if shift => b"\x1b[Z".to_vec(),
            Key::Named(NamedKey::Tab) => b"\t".to_vec(),
            Key::Named(NamedKey::Escape) => vec![0x1b],
            Key::Named(
                dir @ (NamedKey::ArrowUp
                | NamedKey::ArrowDown
                | NamedKey::ArrowLeft
                | NamedKey::ArrowRight),
            ) => {
                let ch = match dir {
                    NamedKey::ArrowUp => 'A',
                    NamedKey::ArrowDown => 'B',
                    NamedKey::ArrowRight => 'C',
                    NamedKey::ArrowLeft => 'D',
                    _ => unreachable!(),
                };
                if shift || ctrl {
                    format!("\x1b[1;{}{ch}", modifier()).into_bytes()
                } else {
                    vec![0x1b, b'[', ch as u8]
                }
            }
            Key::Character(ch) => ch.as_bytes().to_vec(),
            Key::Named(NamedKey::Shift) => {
                self.shift_pressed(true);
                return Ok(true);
            }
            _ => return Ok(false),
        };

        self.write(&seq)?;
        Ok(true)
    }

    /// Paste text into the PTY, wrapping in bracketed-paste markers if the app enabled them.
    pub fn paste(&self, text: &str) -> Result<(), TerminalError> {
        let bracketed = self
            .term
            .borrow()
            .mode()
            .contains(TermMode::BRACKETED_PASTE);
        if bracketed {
            let filtered = text.replace(['\x1b', '\x03'], "");
            self.write_raw(b"\x1b[200~")?;
            self.write_raw(filtered.as_bytes())?;
            self.write_raw(b"\x1b[201~")?;
        } else {
            let normalized = text.replace("\r\n", "\r").replace('\n', "\r");
            self.write_raw(normalized.as_bytes())?;
        }
        Ok(())
    }

    /// Write data to the PTY without resetting scroll or selection state.
    fn write_raw(&self, data: &[u8]) -> Result<(), TerminalError> {
        let mut writer = self.writer.borrow_mut();
        let writer = writer.as_mut().ok_or(TerminalError::NotInitialized)?;
        writer.write_all(data)?;
        writer.flush()?;
        Ok(())
    }

    /// Resize the terminal. Lossless: the grid reflows on width, preserves scrollback on height.
    pub fn resize(&self, rows: u16, cols: u16) {
        // PTY first so SIGWINCH reaches the program before we update locally.
        let _ = self.inner.borrow().master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        });

        self.term.borrow_mut().resize(TermSize {
            screen_lines: rows as usize,
            columns: cols as usize,
        });
    }

    /// Scroll by delta. Positive moves up into scrollback (vt100 convention).
    pub fn scroll(&self, delta: i32) {
        self.scroll_to(Scroll::Delta(delta));
    }

    /// Scroll to the bottom of the buffer.
    pub fn scroll_to_bottom(&self) {
        self.scroll_to(Scroll::Bottom);
    }

    fn scroll_to(&self, target: Scroll) {
        let mut term = self.term.borrow_mut();
        if term.mode().contains(TermMode::ALT_SCREEN) {
            return;
        }
        term.scroll_display(target);
        Platform::get().send(UserEvent::RequestRedraw);
    }

    /// Current working directory reported via OSC 7.
    pub fn cwd(&self) -> Option<PathBuf> {
        self.cwd.borrow().clone()
    }

    /// Window title reported via OSC 0 / 2.
    pub fn title(&self) -> Option<String> {
        self.title.borrow().clone()
    }

    /// Latest clipboard content set via OSC 52.
    pub fn clipboard_content(&self) -> Option<String> {
        self.clipboard_content.borrow().clone()
    }

    /// Snapshot of the active terminal mode bits.
    fn mode(&self) -> TermMode {
        *self.term.borrow().mode()
    }

    fn pressed_button(&self) -> Option<TerminalMouseButton> {
        self.inner.borrow().pressed_button
    }

    fn set_pressed_button(&self, button: Option<TerminalMouseButton>) {
        self.inner.borrow_mut().pressed_button = button;
    }

    fn is_shift_held(&self) -> bool {
        self.inner.borrow().modifiers.contains(Modifiers::SHIFT)
    }

    /// Handle a mouse move/drag. `row` and `col` are fractional cell units;
    /// the fraction of `col` picks which cell half anchors the selection.
    pub fn mouse_move(&self, row: f32, col: f32) {
        let held = self.pressed_button();

        if self.is_shift_held() && held.is_some() {
            self.update_selection(row, col);
            return;
        }

        let mode = self.mode();
        if mode.contains(TermMode::MOUSE_MOTION) {
            // Any-motion mode: report regardless of button state.
            let _ = self
                .write_raw(encode_mouse_move(row as usize, col as usize, held, mode).as_bytes());
        } else if mode.contains(TermMode::MOUSE_DRAG)
            && let Some(button) = held
        {
            // Button-motion mode: only while a button is held.
            let _ = self.write_raw(
                encode_mouse_move(row as usize, col as usize, Some(button), mode).as_bytes(),
            );
        } else if !mode.intersects(TermMode::MOUSE_MODE) && held.is_some() {
            self.update_selection(row, col);
        }
    }

    /// Handle a mouse button press. `selection_type` picks the selection kind when not in mouse mode:
    /// [`SelectionType::Semantic`] for double-click (word), [`SelectionType::Lines`] for triple-click.
    /// See [`Self::mouse_move`] for the fractional coordinates.
    pub fn mouse_down(
        &self,
        row: f32,
        col: f32,
        button: TerminalMouseButton,
        selection_type: SelectionType,
    ) {
        self.set_pressed_button(Some(button));

        let mode = self.mode();
        if !self.is_shift_held() && mode.intersects(TermMode::MOUSE_MODE) {
            let _ = self
                .write_raw(encode_mouse_press(row as usize, col as usize, button, mode).as_bytes());
        } else {
            self.start_selection(row, col, selection_type);
        }
    }

    /// Handle a mouse button release.
    pub fn mouse_up(&self, row: f32, col: f32, button: TerminalMouseButton) {
        self.set_pressed_button(None);

        let mode = self.mode();
        if !self.is_shift_held() && mode.intersects(TermMode::MOUSE_MODE) {
            let _ = self.write_raw(
                encode_mouse_release(row as usize, col as usize, button, mode).as_bytes(),
            );
        }
    }

    /// Handle a mouse button release from outside the terminal viewport.
    pub fn release(&self) {
        self.set_pressed_button(None);
    }

    /// Route a wheel event to scrollback, PTY mouse, or arrow-key sequences
    /// depending on the active mouse mode and alt-screen state (matches wezterm/kitty).
    pub fn wheel(&self, delta_y: f64, row: f32, col: f32) {
        // Lines per event from the OS delta, capped to keep flings sane.
        let lines = (delta_y.abs().ceil() as i32).clamp(1, 10);
        let scroll_delta = if delta_y > 0.0 { lines } else { -lines };

        let mode = self.mode();
        let scroll_offset = self.term.borrow().grid().display_offset();

        if scroll_offset > 0 {
            self.scroll(scroll_delta);
        } else if mode.intersects(TermMode::MOUSE_MODE) {
            let _ = self.write_raw(
                encode_wheel_event(row as usize, col as usize, delta_y, mode).as_bytes(),
            );
        } else if mode.contains(TermMode::ALT_SCREEN) {
            let app_cursor = mode.contains(TermMode::APP_CURSOR);
            let key = match (delta_y > 0.0, app_cursor) {
                (true, true) => "\x1bOA",
                (true, false) => "\x1b[A",
                (false, true) => "\x1bOB",
                (false, false) => "\x1b[B",
            };
            for _ in 0..lines {
                let _ = self.write_raw(key.as_bytes());
            }
        } else {
            self.scroll(scroll_delta);
        }
    }

    /// Borrow the underlying alacritty `Term` for direct read access.
    pub fn term(&self) -> std::cell::Ref<'_, Term<EventProxy>> {
        self.term.borrow()
    }

    /// Future that completes each time new output is received from the PTY.
    pub fn output_received(&self) -> impl std::future::Future<Output = ()> + '_ {
        self.output_notifier.notified()
    }

    /// Future that completes when the window title changes (OSC 0 / OSC 2).
    pub fn title_changed(&self) -> impl std::future::Future<Output = ()> + '_ {
        self.title_notifier.notified()
    }

    /// Future that completes when clipboard content changes (OSC 52).
    pub fn clipboard_changed(&self) -> impl std::future::Future<Output = ()> + '_ {
        self.clipboard_notifier.notified()
    }

    /// Future that completes when the PTY closes.
    pub fn closed(&self) -> impl std::future::Future<Output = ()> + '_ {
        self.closer_notifier.notified()
    }

    /// Unique identifier for this terminal instance.
    pub fn id(&self) -> TerminalId {
        self.id
    }

    /// Track whether shift is currently pressed.
    pub fn shift_pressed(&self, pressed: bool) {
        let mods = &mut self.inner.borrow_mut().modifiers;
        if pressed {
            mods.insert(Modifiers::SHIFT);
        } else {
            mods.remove(Modifiers::SHIFT);
        }
    }

    /// Start a new selection of `selection_type`. See [`Self::mouse_move`] for the fractional coordinates.
    pub fn start_selection(&self, row: f32, col: f32, selection_type: SelectionType) {
        let (point, side) = self.point_and_side_at(row, col);
        self.term.borrow_mut().selection = Some(Selection::new(selection_type, point, side));
        Platform::get().send(UserEvent::RequestRedraw);
    }

    /// Extend the in-progress selection, if any.
    pub fn update_selection(&self, row: f32, col: f32) {
        let (point, side) = self.point_and_side_at(row, col);
        if let Some(selection) = self.term.borrow_mut().selection.as_mut() {
            selection.update(point, side);
            Platform::get().send(UserEvent::RequestRedraw);
        }
    }

    /// Currently selected text, if any.
    pub fn get_selected_text(&self) -> Option<String> {
        self.term.borrow().selection_to_string()
    }

    /// Grid point and cell half (left vs right) for a pointer at fractional cell coordinates.
    fn point_and_side_at(&self, row: f32, col: f32) -> (Point, Side) {
        let term = self.term.borrow();
        let col = col.max(0.0);
        let side = if col.fract() < 0.5 {
            Side::Left
        } else {
            Side::Right
        };
        let point = Point::new(
            Line(row.max(0.0) as i32 - term.grid().display_offset() as i32),
            Column((col as usize).min(term.columns().saturating_sub(1))),
        );
        (point, side)
    }
}
