use vt100::Cell;

/// Selection range in the terminal grid.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct TerminalSelection {
    pub dragging: bool,
    /// Start row of selection
    pub start_row: usize,
    /// Start column of selection
    pub start_col: usize,
    /// End row of selection
    pub end_row: usize,
    /// End column of selection
    pub end_col: usize,
}

impl TerminalSelection {
    /// Create a new selection from start to end positions
    pub fn new(start_row: usize, start_col: usize, end_row: usize, end_col: usize) -> Self {
        Self {
            dragging: false,
            start_row,
            start_col,
            end_row,
            end_col,
        }
    }

    /// Normalize selection so start is always before end (top-left to bottom-right)
    pub fn normalized(&self) -> (usize, usize, usize, usize) {
        let (start_row, start_col, end_row, end_col) = if self.start_row < self.end_row
            || (self.start_row == self.end_row && self.start_col <= self.end_col)
        {
            (self.start_row, self.start_col, self.end_row, self.end_col)
        } else {
            (self.end_row, self.end_col, self.start_row, self.start_col)
        };
        (start_row, start_col, end_row, end_col)
    }

    /// Check if a cell is within the selection
    pub fn contains(&self, row: usize, col: usize) -> bool {
        let (start_row, start_col, end_row, end_col) = self.normalized();
        row >= start_row && row <= end_row && col >= start_col && col <= end_col
    }

    /// Check if selection is empty (zero length)
    pub fn is_empty(&self) -> bool {
        self.start_row == self.end_row && self.start_col == self.end_col
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
}

impl TerminalBuffer {
    /// Get the selected text from the buffer
    pub fn get_selected_text(&self) -> Option<String> {
        let selection = self.selection.as_ref()?;

        if selection.is_empty() {
            return None;
        }

        let (start_row, start_col, end_row, end_col) = selection.normalized();

        let mut lines = Vec::new();

        for row_idx in start_row..=end_row {
            let Some(row) = self.rows.get(row_idx) else {
                continue;
            };

            let line = match row_idx {
                _ if start_row == end_row => {
                    let start = start_col.min(row.len());
                    let end = end_col.min(row.len());
                    cells_to_string(&row[start..end])
                }
                _ if row_idx == start_row => {
                    let start = start_col.min(row.len());
                    cells_to_string(&row[start..])
                }
                _ if row_idx == end_row => cells_to_string(&row[..end_col.min(row.len())]),
                _ => cells_to_string(row),
            };

            lines.push(line);
        }

        Some(lines.join("\n"))
    }
}

/// Convert a slice of cells to a string, treating empty cells as spaces
fn cells_to_string(cells: &[Cell]) -> String {
    cells
        .iter()
        .map(|cell| {
            if cell.has_contents() {
                cell.contents()
            } else {
                " "
            }
        })
        .collect()
}
