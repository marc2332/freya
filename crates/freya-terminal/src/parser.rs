use std::sync::Arc;

use vt100::Parser;

/// Check for terminal queries in PTY output and return appropriate responses.
///
/// This handles DSR, DA, and other queries that shells like nushell send.
pub(crate) fn check_for_terminal_queries(
    data: &[u8],
    parser: &Arc<std::sync::RwLock<Parser>>,
) -> Vec<Vec<u8>> {
    let mut responses = Vec::new();

    // DSR 6n - Cursor Position Report
    if data.windows(4).any(|w| w == b"\x1b[6n")
        && let Ok(p) = parser.read()
    {
        let (row, col) = p.screen().cursor_position();
        let response = format!("\x1b[{};{}R", row + 1, col + 1);
        responses.push(response.into_bytes());
    }

    // DSR ?6n - Extended Cursor Position Report
    if data.windows(5).any(|w| w == b"\x1b[?6n")
        && let Ok(p) = parser.read()
    {
        let (row, col) = p.screen().cursor_position();
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
