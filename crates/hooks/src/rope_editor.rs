use std::{cmp::Ordering, fmt::Display, ops::Range};

use ropey::iter::Lines;
pub use ropey::Rope;

use crate::{text_editor::*, EditableMode};

/// TextEditor implementing a Rope
#[derive(Clone)]
pub struct RopeEditor {
    rope: Rope,
    cursor: TextCursor,
    mode: EditableMode,

    /// Selected text range
    selected: Option<(usize, usize)>,
}

impl Display for RopeEditor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.rope.to_string())
    }
}

impl RopeEditor {
    // Create a new [`RopeEditor`]
    pub fn new(text: String, cursor: TextCursor, mode: EditableMode) -> Self {
        Self {
            rope: Rope::from_str(&text),
            cursor,
            selected: None,
            mode,
        }
    }
}

impl TextEditor for RopeEditor {
    type LinesIterator<'a> = LinesIterator<'a>;

    fn lines(&self) -> Self::LinesIterator<'_> {
        let lines = self.rope.lines();
        LinesIterator { lines }
    }

    fn insert_char(&mut self, char: char, char_idx: usize) {
        self.rope.insert_char(char_idx, char);
    }

    fn insert(&mut self, text: &str, char_idx: usize) {
        self.rope.insert(char_idx, text);
    }

    fn remove(&mut self, range: Range<usize>) {
        self.rope.remove(range)
    }

    fn char_to_line(&self, char_idx: usize) -> usize {
        self.rope.char_to_line(char_idx)
    }

    fn line_to_char(&self, line_idx: usize) -> usize {
        self.rope.line_to_char(line_idx)
    }

    fn line(&self, line_idx: usize) -> Option<Line<'_>> {
        let line = self.rope.get_line(line_idx);

        line.map(|line| Line { text: line.into() })
    }

    fn len_lines<'a>(&self) -> usize {
        self.rope.len_lines()
    }

    fn cursor(&self) -> &TextCursor {
        &self.cursor
    }

    fn cursor_mut(&mut self) -> &mut TextCursor {
        &mut self.cursor
    }

    fn move_highlight_to_cursor(&mut self) {
        let pos = self.cursor_pos();
        if let Some(selected) = self.selected.as_mut() {
            selected.1 = pos;
        } else {
            self.selected = Some((self.cursor_pos(), self.cursor_pos()))
        }
    }

    fn highlights(&self, editor_id: usize) -> Option<(usize, usize)> {
        let (selected_from, selected_to) = self.selected?;

        if self.mode == EditableMode::SingleLineMultipleEditors {
            let selected_to_row = self.char_to_line(selected_to);
            let selected_from_row = self.char_to_line(selected_from);

            let selected_to_line = self.char_to_line(selected_to);
            let selected_from_line = self.char_to_line(selected_from);

            let editor_row_idx = self.line_to_char(editor_id);
            let selected_to_row_idx = self.line_to_char(selected_to_line);
            let selected_from_row_idx = self.line_to_char(selected_from_line);

            let selected_to_col_idx = selected_to - selected_to_row_idx;
            let selected_from_col_idx = selected_from - selected_from_row_idx;

            // Between starting line and endling line
            if (editor_id > selected_from_row && editor_id < selected_to_row)
                || (editor_id < selected_from_row && editor_id > selected_to_row)
            {
                let len = self.line(editor_id).unwrap().len_chars();
                return Some((0, len));
            }

            match selected_from_row.cmp(&selected_to_row) {
                // Selection direction is from bottom -> top
                Ordering::Greater => {
                    if selected_from_row == editor_id {
                        // Starting line
                        return Some((0, selected_from_col_idx));
                    } else if selected_to_row == editor_id {
                        // Ending line
                        let len = self.line(selected_to_row).unwrap().len_chars();
                        return Some((selected_to_col_idx, len));
                    }
                }
                // Selection direction is from top -> bottom
                Ordering::Less => {
                    if selected_from_row == editor_id {
                        // Starting line
                        let len = self.line(selected_from_row).unwrap().len_chars();
                        return Some((selected_from_col_idx, len));
                    } else if selected_to_row == editor_id {
                        // Ending line
                        return Some((0, selected_to_col_idx));
                    }
                }
                Ordering::Equal => {
                    // Starting and endline line are the same
                    if selected_from_row == editor_id {
                        return Some((
                            selected_from - editor_row_idx,
                            selected_to - editor_row_idx,
                        ));
                    }
                }
            }

            None
        } else {
            Some((selected_from, selected_to))
        }
    }

    fn set(&mut self, text: &str) {
        if self.cursor_pos() > text.len() {
            self.set_cursor_pos(text.len());
        }
        self.rope.remove(0..);
        self.rope.insert(0, text);
    }

    fn unhighlight(&mut self) {
        self.selected = None;
    }

    fn highlight_text(&mut self, from: usize, to: usize, editor_id: usize) {
        if self.mode == EditableMode::SingleLineMultipleEditors {
            let row_idx = self.line_to_char(editor_id);
            if self.selected.is_none() {
                self.selected = Some((row_idx + from, row_idx + to));
            } else {
                self.selected.as_mut().unwrap().1 = row_idx + to;
            }
        } else if self.selected.is_none() {
            self.selected = Some((from, to));
        } else {
            self.selected.as_mut().unwrap().1 = to;
        }

        if self.mode == EditableMode::SingleLineMultipleEditors {
            self.cursor_mut().move_to(editor_id, to);
        } else {
            self.set_cursor_pos(to);
        }
    }
}

/// Iterator over text lines.
pub struct LinesIterator<'a> {
    lines: Lines<'a>,
}

impl<'a> Iterator for LinesIterator<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();

        line.map(|line| Line { text: line.into() })
    }
}
