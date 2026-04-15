use alacritty_terminal::term::TermMode;

/// Mouse button for terminal encoding.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TerminalMouseButton {
    Left,
    Middle,
    Right,
}

impl TerminalMouseButton {
    /// X11/SGR button code (without modifier bits).
    fn code(self) -> u8 {
        match self {
            Self::Left => 0,
            Self::Middle => 1,
            Self::Right => 2,
        }
    }
}

/// Encode a mouse event to the byte sequence the running app expects.
///
/// `sgr_code` is the value emitted in SGR (1006) encoding; `x11_code` is the
/// value emitted in classic X11 encoding before the mandatory `+32` offset.
/// `release_in_sgr` only affects SGR encoding (lowercase `m` vs uppercase
/// `M`); X11 release uses a fixed button byte and is selected by passing
/// `x11_code = 3`.
fn encode(
    sgr_code: u8,
    x11_code: u8,
    row: usize,
    col: usize,
    mode: TermMode,
    release_in_sgr: bool,
) -> String {
    let row = row.saturating_add(1);
    let col = col.saturating_add(1);
    if mode.contains(TermMode::SGR_MOUSE) {
        let action = if release_in_sgr { 'm' } else { 'M' };
        format!("\x1b[<{sgr_code};{col};{row}{action}")
    } else {
        let button_byte = x11_code.saturating_add(32);
        let col_byte = col.min(255) as u8;
        let row_byte = row.min(255) as u8;
        format!(
            "\x1b[M{}{}{}",
            button_byte as char, col_byte as char, row_byte as char
        )
    }
}

pub fn encode_mouse_press(
    row: usize,
    col: usize,
    button: TerminalMouseButton,
    mode: TermMode,
) -> String {
    encode(button.code(), button.code(), row, col, mode, false)
}

pub fn encode_mouse_release(
    row: usize,
    col: usize,
    button: TerminalMouseButton,
    mode: TermMode,
) -> String {
    // X11 collapses release into a single "button 3" byte; SGR keeps the
    // original button code but switches `M` to `m`.
    encode(button.code(), 3, row, col, mode, true)
}

/// Encode a mouse motion event. `None` for `button` means hover (no button).
pub fn encode_mouse_move(
    row: usize,
    col: usize,
    button: Option<TerminalMouseButton>,
    mode: TermMode,
) -> String {
    let code = button.map_or(3, TerminalMouseButton::code) + 32;
    encode(code, code, row, col, mode, false)
}

/// Encode a mouse wheel event. Positive `delta_y` = wheel up.
pub fn encode_wheel_event(row: usize, col: usize, delta_y: f64, mode: TermMode) -> String {
    let code = if delta_y > 0.0 { 64 } else { 65 };
    encode(code, code, row, col, mode, false)
}
