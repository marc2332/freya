use std::{
    cmp::Ordering,
    fmt::Display,
    ops::Range,
};

use ropey::{
    Rope,
    iter::Lines,
};

use crate::{
    EditorLine,
    TextSelection,
    editor_history::{
        EditorHistory,
        HistoryChange,
    },
    text_editor::{
        Line,
        TextEditor,
    },
};

/// TextEditor implementing a Rope
pub struct RopeEditor {
    pub(crate) rope: Rope,
    pub(crate) selection: TextSelection,
    pub(crate) indentation: u8,
    pub(crate) history: EditorHistory,
}

impl Display for RopeEditor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.rope.to_string())
    }
}

impl RopeEditor {
    // Create a new [`RopeEditor`]
    pub fn new(
        text: String,
        selection: TextSelection,
        indentation: u8,
        history: EditorHistory,
    ) -> Self {
        Self {
            rope: Rope::from_str(&text),
            selection,
            indentation,
            history,
        }
    }

    pub fn rope(&self) -> &Rope {
        &self.rope
    }
}

impl TextEditor for RopeEditor {
    type LinesIterator<'a> = LinesIterator<'a>;

    fn lines(&self) -> Self::LinesIterator<'_> {
        let lines = self.rope.lines();
        LinesIterator { lines }
    }

    fn insert_char(&mut self, ch: char, idx: usize) -> usize {
        let idx_utf8 = self.utf16_cu_to_char(idx);
        let selection = self.selection.clone();

        let len_before_insert = self.rope.len_utf16_cu();
        self.rope.insert_char(idx_utf8, ch);
        let len_after_insert = self.rope.len_utf16_cu();

        let inserted_text_len = len_after_insert - len_before_insert;

        self.history.push_change(HistoryChange::InsertChar {
            idx,
            ch,
            len: inserted_text_len,
            selection,
        });

        inserted_text_len
    }

    fn insert(&mut self, text: &str, idx: usize) -> usize {
        let idx_utf8 = self.utf16_cu_to_char(idx);
        let selection = self.selection.clone();

        let len_before_insert = self.rope.len_utf16_cu();
        self.rope.insert(idx_utf8, text);
        let len_after_insert = self.rope.len_utf16_cu();

        let inserted_text_len = len_after_insert - len_before_insert;

        self.history.push_change(HistoryChange::InsertText {
            idx,
            text: text.to_owned(),
            len: inserted_text_len,
            selection,
        });

        inserted_text_len
    }

    fn remove(&mut self, range_utf16: Range<usize>) -> usize {
        let range =
            self.utf16_cu_to_char(range_utf16.start)..self.utf16_cu_to_char(range_utf16.end);
        let text = self.rope.slice(range.clone()).to_string();
        let selection = self.selection.clone();

        let len_before_remove = self.rope.len_utf16_cu();
        self.rope.remove(range);
        let len_after_remove = self.rope.len_utf16_cu();

        let removed_text_len = len_before_remove - len_after_remove;

        self.history.push_change(HistoryChange::Remove {
            idx: range_utf16.end - removed_text_len,
            text,
            len: removed_text_len,
            selection,
        });

        removed_text_len
    }

    fn char_to_line(&self, char_idx: usize) -> usize {
        self.rope.char_to_line(char_idx)
    }

    fn line_to_char(&self, line_idx: usize) -> usize {
        self.rope.line_to_char(line_idx)
    }

    fn utf16_cu_to_char(&self, utf16_cu_idx: usize) -> usize {
        self.rope.utf16_cu_to_char(utf16_cu_idx)
    }

    fn char_to_utf16_cu(&self, idx: usize) -> usize {
        self.rope.char_to_utf16_cu(idx)
    }

    fn line(&self, line_idx: usize) -> Option<Line<'_>> {
        let line = self.rope.get_line(line_idx);

        line.map(|line| Line {
            text: line.into(),
            utf16_len: line.len_utf16_cu(),
        })
    }

    fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    fn len_utf16_cu(&self) -> usize {
        self.rope.len_utf16_cu()
    }

    fn selection(&self) -> &TextSelection {
        &self.selection
    }

    fn selection_mut(&mut self) -> &mut TextSelection {
        &mut self.selection
    }

    fn has_any_selection(&self) -> bool {
        self.selection.is_range()
    }

    fn get_selection(&self) -> Option<(usize, usize)> {
        match self.selection {
            TextSelection::Cursor(_) => None,
            TextSelection::Range { from, to } => Some((from, to)),
        }
    }

    fn get_visible_selection(&self, editor_line: EditorLine) -> Option<(usize, usize)> {
        let (selected_from, selected_to) = match self.selection {
            TextSelection::Cursor(_) => return None,
            TextSelection::Range { from, to } => (from, to),
        };

        match editor_line {
            EditorLine::Paragraph(line_index) => {
                let selected_from_row = self.char_to_line(self.utf16_cu_to_char(selected_from));
                let selected_to_row = self.char_to_line(self.utf16_cu_to_char(selected_to));

                let editor_row_idx = self.char_to_utf16_cu(self.line_to_char(line_index));
                let selected_from_row_idx =
                    self.char_to_utf16_cu(self.line_to_char(selected_from_row));
                let selected_to_row_idx = self.char_to_utf16_cu(self.line_to_char(selected_to_row));

                let selected_from_col_idx = selected_from - selected_from_row_idx;
                let selected_to_col_idx = selected_to - selected_to_row_idx;

                // Between starting line and endling line
                if (line_index > selected_from_row && line_index < selected_to_row)
                    || (line_index < selected_from_row && line_index > selected_to_row)
                {
                    let len = self.line(line_index).unwrap().utf16_len();
                    return Some((0, len));
                }

                match selected_from_row.cmp(&selected_to_row) {
                    // Selection direction is from bottom -> top
                    Ordering::Greater => {
                        if selected_from_row == line_index {
                            // Starting line
                            Some((0, selected_from_col_idx))
                        } else if selected_to_row == line_index {
                            // Ending line
                            let len = self.line(selected_to_row).unwrap().utf16_len();
                            Some((selected_to_col_idx, len))
                        } else {
                            None
                        }
                    }
                    // Selection direction is from top -> bottom
                    Ordering::Less => {
                        if selected_from_row == line_index {
                            // Starting line
                            let len = self.line(selected_from_row).unwrap().utf16_len();
                            Some((selected_from_col_idx, len))
                        } else if selected_to_row == line_index {
                            // Ending line
                            Some((0, selected_to_col_idx))
                        } else {
                            None
                        }
                    }
                    Ordering::Equal if selected_from_row == line_index => {
                        // Starting and endline line are the same
                        Some((selected_from - editor_row_idx, selected_to - editor_row_idx))
                    }
                    _ => None,
                }
            }
            EditorLine::SingleParagraph => Some((selected_from, selected_to)),
        }
    }

    fn set(&mut self, text: &str) {
        self.rope.remove(0..);
        self.rope.insert(0, text);
        if self.cursor_pos() > text.len() {
            self.move_cursor_to(text.len());
        }
    }

    fn clear_selection(&mut self) {
        let end = self.selection().end();
        self.selection_mut().set_as_cursor();
        self.selection_mut().move_to(end);
    }

    fn set_selection(&mut self, (from, to): (usize, usize)) {
        self.selection = TextSelection::Range { from, to };
    }

    fn get_selected_text(&self) -> Option<String> {
        let (start, end) = self.get_selection_range()?;

        let start = self.utf16_cu_to_char(start);
        let end = self.utf16_cu_to_char(end);

        Some(self.rope().get_slice(start..end)?.to_string())
    }

    fn get_selection_range(&self) -> Option<(usize, usize)> {
        let (start, end) = match self.selection {
            TextSelection::Cursor(_) => return None,
            TextSelection::Range { from, to } => (from, to),
        };

        // Use left-to-right selection
        let (start, end) = if start < end {
            (start, end)
        } else {
            (end, start)
        };

        Some((start, end))
    }

    fn undo(&mut self) -> Option<TextSelection> {
        self.history.undo(&mut self.rope)
    }

    fn redo(&mut self) -> Option<TextSelection> {
        self.history.redo(&mut self.rope)
    }

    fn editor_history(&mut self) -> &mut EditorHistory {
        &mut self.history
    }

    fn get_indentation(&self) -> u8 {
        self.indentation
    }
}

/// Iterator over text lines.
pub struct LinesIterator<'a> {
    pub lines: Lines<'a>,
}

impl<'a> Iterator for LinesIterator<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();

        line.map(|line| Line {
            text: line.into(),
            utf16_len: line.len_utf16_cu(),
        })
    }
}
