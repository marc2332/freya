use vt100::{
    MouseProtocolEncoding,
    Parser,
};

/// Check for terminal queries in PTY output and return appropriate responses.
///
/// This handles DSR, DA, and other queries that shells like nushell send.
pub(crate) fn check_for_terminal_queries(data: &[u8], parser: &Parser) -> Vec<Vec<u8>> {
    let mut responses = Vec::new();

    // DSR 6n - Cursor Position Report
    if data.windows(4).any(|w| w == b"\x1b[6n") {
        let (row, col) = parser.screen().cursor_position();
        let response = format!("\x1b[{};{}R", row + 1, col + 1);
        responses.push(response.into_bytes());
    }

    // DSR ?6n - Extended Cursor Position Report
    if data.windows(5).any(|w| w == b"\x1b[?6n") {
        let (row, col) = parser.screen().cursor_position();
        let response = format!("\x1b[?{};{}R", row + 1, col + 1);
        responses.push(response.into_bytes());
    }

    // DSR 5n - Device Status Report (terminal OK)
    if data.windows(4).any(|w| w == b"\x1b[5n") {
        responses.push(b"\x1b[0n".to_vec());
    }

    // DA1 - Primary Device Attributes
    if data.windows(3).any(|w| w == b"\x1b[c") || data.windows(4).any(|w| w == b"\x1b[0c") {
        responses.push(b"\x1b[?62;22c".to_vec());
    }

    // DA2 - Secondary Device Attributes
    if data.windows(4).any(|w| w == b"\x1b>c") || data.windows(5).any(|w| w == b"\x1b>0c") {
        responses.push(b"\x1b[>0;0;0c".to_vec());
    }

    responses
}

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

/// Encode a mouse button press event.
pub fn encode_mouse_press(
    row: usize,
    col: usize,
    button: TerminalMouseButton,
    encoding: MouseProtocolEncoding,
) -> String {
    match encoding {
        MouseProtocolEncoding::Sgr => {
            let sgr_row = row.saturating_add(1);
            let sgr_col = col.saturating_add(1);
            format!("\x1b[<{};{};{}M", button.code(), sgr_col, sgr_row)
        }
        _ => {
            let button_byte = button.code().saturating_add(32);
            let col_byte = col.saturating_add(1).min(255) as u8;
            let row_byte = row.saturating_add(1).min(255) as u8;
            format!(
                "\x1b[M{}{}{}",
                button_byte as char, col_byte as char, row_byte as char
            )
        }
    }
}

/// Encode a mouse button release event.
pub fn encode_mouse_release(
    row: usize,
    col: usize,
    button: TerminalMouseButton,
    encoding: MouseProtocolEncoding,
) -> String {
    match encoding {
        MouseProtocolEncoding::Sgr => {
            // SGR uses lowercase 'm' for release with the original button code
            let sgr_row = row.saturating_add(1);
            let sgr_col = col.saturating_add(1);
            format!("\x1b[<{};{};{}m", button.code(), sgr_col, sgr_row)
        }
        _ => {
            // X11: release is always button code 3 (no specific button) + 32
            let button_byte = 35u8; // 3 + 32
            let col_byte = col.saturating_add(1).min(255) as u8;
            let row_byte = row.saturating_add(1).min(255) as u8;
            format!(
                "\x1b[M{}{}{}",
                button_byte as char, col_byte as char, row_byte as char
            )
        }
    }
}

/// Encode a mouse motion event using the specified protocol encoding.
///
/// When `button` is `None`, this encodes hover motion (no button pressed):
/// button code = 3 + 32 (motion flag) = 35.
/// When `button` is `Some`, this encodes drag motion (button held):
/// button code = button.code() + 32 (motion flag).
pub fn encode_mouse_move(
    row: usize,
    col: usize,
    button: Option<TerminalMouseButton>,
    encoding: MouseProtocolEncoding,
) -> String {
    // Motion flag = 32. No-button code = 3.
    let code = match button {
        Some(b) => b.code() + 32,
        None => 3 + 32, // 35
    };

    match encoding {
        MouseProtocolEncoding::Sgr => {
            let sgr_row = row.saturating_add(1);
            let sgr_col = col.saturating_add(1);
            format!("\x1b[<{};{};{}M", code, sgr_col, sgr_row)
        }
        _ => {
            let button_byte = code.saturating_add(32);
            let col_byte = col.saturating_add(1).min(255) as u8;
            let row_byte = row.saturating_add(1).min(255) as u8;
            format!(
                "\x1b[M{}{}{}",
                button_byte as char, col_byte as char, row_byte as char
            )
        }
    }
}

/// Encode a mouse wheel event using the specified protocol encoding.
///
/// Positive `delta_y` = wheel up (away from user), negative = wheel down.
pub fn encode_wheel_event(
    row: usize,
    col: usize,
    delta_y: f64,
    encoding: MouseProtocolEncoding,
) -> String {
    // Terminal protocol: wheel up = button 64, wheel down = button 65.
    match encoding {
        MouseProtocolEncoding::Sgr => {
            let button = if delta_y > 0.0 { 64 } else { 65 };
            let sgr_row = row.saturating_add(1);
            let sgr_col = col.saturating_add(1);
            // Wheel events are press-only (M), no release needed
            format!("\x1b[<{};{};{}M", button, sgr_col, sgr_row)
        }
        // Default and Utf8 both use the X11-style encoding
        _ => {
            // \x1b[M followed by 3 bytes:
            //   Byte 1: button + 32 (wheel up = 64+32=96, wheel down = 65+32=97)
            //   Byte 2: column (1-indexed + 32)
            //   Byte 3: row (1-indexed + 32)
            let button_byte = if delta_y > 0.0 { 96u8 } else { 97u8 };
            let col_byte = col.saturating_add(1).min(255) as u8;
            let row_byte = row.saturating_add(1).min(255) as u8;
            format!(
                "\x1b[M{}{}{}",
                button_byte as char, col_byte as char, row_byte as char
            )
        }
    }
}
