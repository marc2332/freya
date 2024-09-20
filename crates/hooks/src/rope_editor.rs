use std::{
    cmp::Ordering,
    fmt::Display,
    ops::Range,
};

use dioxus_sdk::clipboard::UseClipboard;
use ropey::iter::Lines;
pub use ropey::Rope;

use crate::{
    text_editor::*,
    EditableMode,
    EditorHistory,
    HistoryChange,
};

/// TextEditor implementing a Rope
pub struct RopeEditor {
    pub(crate) rope: Rope,
    pub(crate) cursor: TextCursor,
    pub(crate) identation: u8,
    pub(crate) mode: EditableMode,
    pub(crate) selected: Option<(usize, usize)>,
    pub(crate) clipboard: UseClipboard,
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
        cursor: TextCursor,
        identation: u8,
        mode: EditableMode,
        clipboard: UseClipboard,
        history: EditorHistory,
    ) -> Self {
        Self {
            rope: Rope::from_str(&text),
            cursor,
            identation,
            selected: None,
            mode,
            clipboard,
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

    fn insert_char(&mut self, ch: char, idx_utf16: usize) {
        let idx = self.utf16_cu_to_char(idx_utf16);

        let len_before_insert = self.rope.len_utf16_cu();
        self.rope.insert_char(idx, ch);
        let len_after_insert = self.rope.len_utf16_cu();

        self.history.push_change(HistoryChange::InsertChar {
            idx: idx_utf16,
            ch,
            len: len_after_insert - len_before_insert,
        });
    }

    fn insert(&mut self, text: &str, idx_utf16: usize) {
        let idx = self.utf16_cu_to_char(idx_utf16);

        let len_before_insert = self.rope.len_utf16_cu();
        self.rope.insert(idx, text);
        let len_after_insert = self.rope.len_utf16_cu();

        self.history.push_change(HistoryChange::InsertText {
            idx: idx_utf16,
            text: text.to_owned(),
            len: len_after_insert - len_before_insert,
        });
    }

    fn remove(&mut self, range_utf16: Range<usize>) {
        let range =
            self.utf16_cu_to_char(range_utf16.start)..self.utf16_cu_to_char(range_utf16.end);
        let text = self.rope.slice(range.clone()).to_string();

        let len_before_remove = self.rope.len_utf16_cu();
        self.rope.remove(range);
        let len_after_remove = self.rope.len_utf16_cu();

        self.history.push_change(HistoryChange::Remove {
            idx: range_utf16.start,
            text,
            len: len_before_remove - len_after_remove,
        });
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

    fn cursor(&self) -> &TextCursor {
        &self.cursor
    }

    fn cursor_mut(&mut self) -> &mut TextCursor {
        &mut self.cursor
    }

    fn expand_selection_to_cursor(&mut self) {
        let pos = self.cursor_pos();
        if let Some(selected) = self.selected.as_mut() {
            selected.1 = pos;
        } else {
            self.selected = Some((self.cursor_pos(), self.cursor_pos()))
        }
    }

    fn get_clipboard(&mut self) -> &mut UseClipboard {
        &mut self.clipboard
    }

    fn has_any_selection(&self) -> bool {
        self.selected.is_some()
    }

    fn get_selection(&self) -> Option<(usize, usize)> {
        self.selected
    }

    fn get_visible_selection(&self, editor_id: usize) -> Option<(usize, usize)> {
        let (selected_from, selected_to) = self.selected?;

        if self.mode == EditableMode::SingleLineMultipleEditors {
            let selected_from_row = self.char_to_line(self.utf16_cu_to_char(selected_from));
            let selected_to_row = self.char_to_line(self.utf16_cu_to_char(selected_to));

            let editor_row_idx = self.char_to_utf16_cu(self.line_to_char(editor_id));
            let selected_from_row_idx = self.char_to_utf16_cu(self.line_to_char(selected_from_row));
            let selected_to_row_idx = self.char_to_utf16_cu(self.line_to_char(selected_to_row));

            let selected_from_col_idx = selected_from - selected_from_row_idx;
            let selected_to_col_idx = selected_to - selected_to_row_idx;

            // Between starting line and endling line
            if (editor_id > selected_from_row && editor_id < selected_to_row)
                || (editor_id < selected_from_row && editor_id > selected_to_row)
            {
                let len = self.line(editor_id).unwrap().utf16_len();
                return Some((0, len));
            }

            let highlights = match selected_from_row.cmp(&selected_to_row) {
                // Selection direction is from bottom -> top
                Ordering::Greater => {
                    if selected_from_row == editor_id {
                        // Starting line
                        Some((0, selected_from_col_idx))
                    } else if selected_to_row == editor_id {
                        // Ending line
                        let len = self.line(selected_to_row).unwrap().utf16_len();
                        Some((selected_to_col_idx, len))
                    } else {
                        None
                    }
                }
                // Selection direction is from top -> bottom
                Ordering::Less => {
                    if selected_from_row == editor_id {
                        // Starting line
                        let len = self.line(selected_from_row).unwrap().utf16_len();
                        Some((selected_from_col_idx, len))
                    } else if selected_to_row == editor_id {
                        // Ending line
                        Some((0, selected_to_col_idx))
                    } else {
                        None
                    }
                }
                Ordering::Equal if selected_from_row == editor_id => {
                    // Starting and endline line are the same
                    Some((selected_from - editor_row_idx, selected_to - editor_row_idx))
                }
                _ => None,
            };

            highlights
        } else {
            Some((selected_from, selected_to))
        }
    }

    fn set(&mut self, text: &str) {
        self.rope.remove(0..);
        self.rope.insert(0, text);
        if self.cursor_pos() > text.len() {
            self.set_cursor_pos(text.len());
        }
    }

    fn clear_selection(&mut self) {
        self.selected = None;
    }

    fn measure_new_selection(&self, from: usize, to: usize, editor_id: usize) -> (usize, usize) {
        if self.mode == EditableMode::SingleLineMultipleEditors {
            let row_idx = self.line_to_char(editor_id);
            let row_idx = self.char_to_utf16_cu(row_idx);
            if let Some((start, _)) = self.selected {
                (start, row_idx + to)
            } else {
                (row_idx + from, row_idx + to)
            }
        } else if let Some((start, _)) = self.selected {
            (start, to)
        } else {
            (from, to)
        }
    }

    fn measure_new_cursor(&self, to: usize, editor_id: usize) -> TextCursor {
        if self.mode == EditableMode::SingleLineMultipleEditors {
            let row_char = self.line_to_char(editor_id);
            let pos = self.char_to_utf16_cu(row_char) + to;
            TextCursor::new(pos)
        } else {
            TextCursor::new(to)
        }
    }

    fn set_selection(&mut self, selected: (usize, usize)) {
        self.selected = Some(selected);
    }

    fn get_selected_text(&self) -> Option<String> {
        let (start, end) = self.get_selection_range()?;

        let start = self.utf16_cu_to_char(start);
        let end = self.utf16_cu_to_char(end);

        Some(self.rope().get_slice(start..end)?.to_string())
    }

    fn get_selection_range(&self) -> Option<(usize, usize)> {
        let (start, end) = self.selected?;

        // Use left-to-right selection
        let (start, end) = if start < end {
            (start, end)
        } else {
            (end, start)
        };

        Some((start, end))
    }

    fn undo(&mut self) -> Option<usize> {
        self.history.undo(&mut self.rope)
    }

    fn redo(&mut self) -> Option<usize> {
        self.history.redo(&mut self.rope)
    }

    fn get_identation(&self) -> u8 {
        self.identation
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
