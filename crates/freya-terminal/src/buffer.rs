use vt100::Cell;

/// Selection range in the terminal grid.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct TerminalSelection {
    pub dragging: bool,
    pub start_row: usize,
    pub start_col: usize,
    pub start_scroll: usize,
    pub end_row: usize,
    pub end_col: usize,
    pub end_scroll: usize,
}

impl TerminalSelection {
    /// Get normalized display positions for rendering at the given scroll offset.
    pub fn display_positions(&self, current_scroll: usize) -> (i64, usize, i64, usize) {
        let current_scroll = current_scroll as i64;
        let start_display = self.start_row as i64 - self.start_scroll as i64 + current_scroll;
        let end_display = self.end_row as i64 - self.end_scroll as i64 + current_scroll;
        if start_display < end_display
            || (start_display == end_display && self.start_col <= self.end_col)
        {
            (start_display, self.start_col, end_display, self.end_col)
        } else {
            (end_display, self.end_col, start_display, self.start_col)
        }
    }

    pub fn is_empty(&self) -> bool {
        let start_content = self.start_row as i64 - self.start_scroll as i64;
        let end_content = self.end_row as i64 - self.end_scroll as i64;
        start_content == end_content && self.start_col == self.end_col
    }
}

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
    /// Current text selection
    pub selection: Option<TerminalSelection>,
    /// Current scroll offset from the bottom (0 = no scroll, at latest output)
    pub scroll_offset: usize,
    /// Total number of scrollback lines available
    pub total_scrollback: usize,
}
