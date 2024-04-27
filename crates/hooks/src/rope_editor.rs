use std::{cmp::Ordering, fmt::Display, ops::Range};

use dioxus_sdk::clipboard::UseClipboard;
use ropey::iter::Lines;
pub use ropey::Rope;

use crate::{text_editor::*, EditableMode, EditorHistory, HistoryChange};

/// TextEditor implementing a Rope
pub struct RopeEditor {
    pub(crate) rope: Rope,
    pub(crate) cursor: TextCursor,
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
        mode: EditableMode,
        clipboard: UseClipboard,
        history: EditorHistory,
    ) -> Self {
        Self {
            rope: Rope::from_str(&text),
            cursor,
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

    fn insert_char(&mut self, char: char, idx: usize) {
        self.history
            .push_change(HistoryChange::InsertChar { idx, char });
        self.rope.insert_char(idx, char);
    }

    fn insert(&mut self, text: &str, idx: usize) {
        self.history.push_change(HistoryChange::InsertText {
            idx,
            text: text.to_owned(),
        });
        self.rope.insert(idx, text);
    }

    fn remove(&mut self, range: Range<usize>) {
        let text = self.rope.slice(range.clone()).to_string();
        self.history.push_change(HistoryChange::Remove {
            idx: range.start,
            text,
        });
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

    fn get_clipboard(&mut self) -> &mut UseClipboard {
        &mut self.clipboard
    }

    fn has_any_highlight(&self) -> bool {
        self.selected.is_some()
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
        self.rope.remove(0..);
        self.rope.insert(0, text);
        if self.cursor_pos() > text.len() {
            self.set_cursor_pos(text.len());
        }
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

    fn get_selected_text(&self) -> Option<String> {
        let (start, end) = self.get_selection()?;

        Some(self.rope().get_slice(start..end)?.to_string())
    }

    fn get_selection(&self) -> Option<(usize, usize)> {
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
}

/// Iterator over text lines.
pub struct LinesIterator<'a> {
    pub lines: Lines<'a>,
}

impl<'a> Iterator for LinesIterator<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();

        line.map(|line| Line { text: line.into() })
    }
}
