use std::{
    fmt::Display,
    ops::Range,
};

use ropey::{
    Rope,
    iter::Lines,
};

use crate::{
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

/// Tracks the position and length of IME preedit text within the rope.
#[derive(Clone, Debug)]
pub struct PreeditState {
    /// Start position in UTF-16 code units.
    pub start: usize,
    /// Length in UTF-16 code units.
    pub len: usize,
}

/// TextEditor implementing a Rope
pub struct RopeEditor {
    pub(crate) rope: Rope,
    pub(crate) selection: TextSelection,
    pub(crate) indentation: u8,
    pub(crate) history: EditorHistory,
    pub(crate) preedit: Option<PreeditState>,
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
            preedit: None,
        }
    }

    pub fn rope(&self) -> &Rope {
        &self.rope
    }

    /// Insert or replace IME preedit text at the current cursor position.
    ///
    /// The preedit text is inserted directly into the rope without recording
    /// undo history. If there is already active preedit text, it is replaced.
    /// An empty `text` clears the preedit.
    pub fn set_preedit(&mut self, text: &str) {
        // Remove existing preedit text from the rope if any
        let preedit_start = if let Some(preedit) = self.preedit.take() {
            let start_char = self.rope.utf16_cu_to_char(preedit.start);
            let end_char = self.rope.utf16_cu_to_char(preedit.start + preedit.len);
            self.rope.remove(start_char..end_char);
            preedit.start
        } else {
            self.cursor_pos()
        };

        // Insert new preedit text at the start position
        let start_char = self.rope.utf16_cu_to_char(preedit_start);
        let len_before = self.rope.len_utf16_cu();
        self.rope.insert(start_char, text);
        let len_after = self.rope.len_utf16_cu();
        let preedit_len = len_after - len_before;

        self.preedit = Some(PreeditState {
            start: preedit_start,
            len: preedit_len,
        });
        self.selection = TextSelection::Cursor(preedit_start + preedit_len);
    }

    /// Remove active preedit text from the rope and restore the cursor.
    pub fn clear_preedit(&mut self) {
        if let Some(preedit) = self.preedit.take() {
            let start_char = self.rope.utf16_cu_to_char(preedit.start);
            let end_char = self.rope.utf16_cu_to_char(preedit.start + preedit.len);
            self.rope.remove(start_char..end_char);
            self.selection = TextSelection::Cursor(preedit.start);
        }
    }

    /// Whether there is active IME preedit text in the rope.
    pub fn has_preedit(&self) -> bool {
        self.preedit.is_some()
    }

    /// Returns the rope content with preedit text excluded.
    ///
    /// This represents the "committed" text that should be synced
    /// to external state.
    pub fn committed_text(&self) -> String {
        if let Some(preedit) = &self.preedit {
            let start_char = self.rope.utf16_cu_to_char(preedit.start);
            let end_char = self.rope.utf16_cu_to_char(preedit.start + preedit.len);
            let before = self.rope.slice(..start_char);
            let after = self.rope.slice(end_char..);
            format!("{before}{after}")
        } else {
            self.rope.to_string()
        }
    }

    /// Returns the rope text split into (before_preedit, preedit, after_preedit).
    ///
    /// If there is no active preedit, returns the full rope text as `before`
    /// with empty preedit and after segments.
    pub fn preedit_text_segments(&self) -> (String, String, String) {
        if let Some(preedit) = &self.preedit {
            let start_char = self.rope.utf16_cu_to_char(preedit.start);
            let end_char = self.rope.utf16_cu_to_char(preedit.start + preedit.len);
            let before = self.rope.slice(..start_char).to_string();
            let preedit_text = self.rope.slice(start_char..end_char).to_string();
            let after = self.rope.slice(end_char..).to_string();
            (before, preedit_text, after)
        } else {
            (self.rope.to_string(), String::new(), String::new())
        }
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

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::RopeEditor;
    use crate::{
        EditorHistory,
        TextSelection,
        text_editor::TextEditor,
    };

    fn editor(text: &str) -> RopeEditor {
        RopeEditor::new(
            text.to_string(),
            TextSelection::new_cursor(0),
            4,
            EditorHistory::new(Duration::ZERO),
        )
    }

    #[test]
    fn preedit_lifecycle() {
        let mut ed = editor("Hello World");
        // Place cursor at position 5 ("Hello| World")
        ed.move_cursor_to(5);

        // Initially no preedit
        assert!(!ed.has_preedit());
        assert_eq!(ed.committed_text(), "Hello World");

        // Set preedit: text is inserted into the rope, cursor moves after it
        ed.set_preedit("你好");
        assert!(ed.has_preedit());
        assert_eq!(ed.rope().to_string(), "Hello你好 World");
        assert_eq!(ed.committed_text(), "Hello World");
        assert_eq!(ed.cursor_pos(), 5 + "你好".encode_utf16().count());

        // Replace preedit with different text
        ed.set_preedit("世界abc");
        assert!(ed.has_preedit());
        assert_eq!(ed.rope().to_string(), "Hello世界abc World");
        assert_eq!(ed.committed_text(), "Hello World");
        assert_eq!(ed.cursor_pos(), 5 + "世界abc".encode_utf16().count());

        // Clear preedit (simulates Ime::Preedit("", None))
        ed.clear_preedit();
        assert!(!ed.has_preedit());
        assert_eq!(ed.rope().to_string(), "Hello World");
        assert_eq!(ed.committed_text(), "Hello World");
        assert_eq!(ed.cursor_pos(), 5);
    }

    #[test]
    fn preedit_skips_undo_history_and_clear_restores() {
        let mut ed = editor("Hello");
        ed.move_cursor_to(5);
        assert!(!ed.history.can_undo());

        // Insert preedit — should NOT create undo history
        ed.set_preedit("XY");
        assert!(!ed.history.can_undo());
        assert_eq!(ed.rope().to_string(), "HelloXY");

        // Replace preedit — still no undo history
        ed.set_preedit("Z");
        assert!(!ed.history.can_undo());
        assert_eq!(ed.rope().to_string(), "HelloZ");

        // clear_preedit restores rope and cursor
        ed.clear_preedit();
        assert!(!ed.has_preedit());
        assert!(!ed.history.can_undo());
        assert_eq!(ed.rope().to_string(), "Hello");
        assert_eq!(ed.cursor_pos(), 5);

        // Clearing again is a no-op
        ed.clear_preedit();
        assert_eq!(ed.rope().to_string(), "Hello");
        assert_eq!(ed.cursor_pos(), 5);
    }
}
