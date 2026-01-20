use vt100::Cell;

/// Terminal buffer containing the current state of the terminal.
#[derive(Clone, PartialEq, Default)]
pub struct TerminalBuffer {
    /// Terminal grid rows
    pub rows: Vec<Vec<Cell>>,
    /// Cursor row position
    pub cursor_row: usize,
    /// Cursor column position
    pub cursor_col: usize,
    /// Number of columns in the terminal
    pub cols: usize,
    /// Number of rows in the terminal
    pub rows_count: usize,
}
