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
    config::EditBindings,
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
    pub(crate) bindings: EditBindings,
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
            bindings: EditBindings::default(),
        }
    }

    pub fn rope(&self) -> &Rope {
        &self.rope
    }

    /// Replace the editing-action chords this editor responds to in
    /// [`TextEditor::process_key`].
    pub fn set_edit_bindings(&mut self, bindings: EditBindings) {
        self.bindings = bindings;
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

    fn edit_bindings(&self) -> &EditBindings {
        &self.bindings
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

    use freya_core::prelude::PressEventType;
    use keyboard_types::{
        Key,
        Modifiers,
        NamedKey,
    };

    use super::RopeEditor;
    use crate::{
        EditorHistory,
        TextDragging,
        TextSelection,
        text_editor::TextEditor,
    };

    /// The drag state a press of `press` over `anchor` leaves behind.
    fn after_press(press: PressEventType, anchor: (usize, usize)) -> TextDragging {
        let mut dragging = TextDragging {
            clicked: true,
            ..TextDragging::default()
        };
        dragging.pressed(press, &TextSelection::new_range(anchor));
        dragging
    }

    fn editor(text: &str) -> RopeEditor {
        RopeEditor::new(
            text.to_string(),
            TextSelection::new_cursor(0),
            4,
            EditorHistory::new(Duration::ZERO),
        )
    }

    fn press(ed: &mut RopeEditor, key: NamedKey) {
        press_with(ed, key, Modifiers::empty());
    }

    fn press_with(ed: &mut RopeEditor, key: NamedKey, modifiers: Modifiers) {
        ed.process_key(&Key::Named(key), &modifiers, true, true, false, false);
    }

    /// Put the caret at `pos` with nothing selected. `move_cursor_to` alone only moves
    /// a selection's active end, so a test that reuses an editor after a Shift press
    /// would otherwise carry the old anchor into the next assertion.
    fn place(ed: &mut RopeEditor, pos: usize) {
        ed.clear_selection();
        ed.move_cursor_to(pos);
    }

    /// The modifier a word jump sits on for this build's platform, so the assertions
    /// below read the same on either.
    #[cfg(target_os = "macos")]
    const WORD: Modifiers = Modifiers::ALT;
    #[cfg(not(target_os = "macos"))]
    const WORD: Modifiers = Modifiers::CONTROL;

    /// The primary modifier, which Home/End widen to the whole document under.
    #[cfg(target_os = "macos")]
    const PRIMARY: Modifiers = Modifiers::META;
    #[cfg(not(target_os = "macos"))]
    const PRIMARY: Modifiers = Modifiers::CONTROL;

    #[test]
    fn a_line_span_carries_its_terminator_and_the_line_bounds_do_not() {
        let ed = editor("hello world\nsecond line");

        // A triple press selects the line so that removing it removes the line.
        assert_eq!(ed.line_span(5), 0..12);
        // The caret and a delete-to-line-end both stop in front of the break.
        assert_eq!(ed.line_bounds(5), 0..11);

        // The last line has no terminator, so the two agree.
        assert_eq!(ed.line_span(15), 12..23);
        assert_eq!(ed.line_bounds(15), 12..23);
    }

    #[test]
    fn a_drag_extends_by_the_unit_its_press_used() {
        let ed = editor("hello world");
        let word = after_press(PressEventType::Double, (0, 5));

        // The pointer twitching inside the word the press selected keeps the word: the
        // regression a character-wise drag caused, leaving word-start to the pointer.
        for pointer in [0, 1, 4, 5] {
            assert_eq!(
                ed.drag_selection(pointer, &word, TextSelection::new_range((0, 5))),
                TextSelection::new_range((0, 5)),
                "pointer {pointer} broke the pressed word"
            );
        }

        // Dragging on past it extends by whole words, never mid-word.
        assert_eq!(
            ed.drag_selection(8, &word, TextSelection::new_range((0, 5))),
            TextSelection::new_range((0, 11))
        );

        // Dragging back before it pivots on the far edge of the pressed word.
        let word = after_press(PressEventType::Double, (6, 11));
        assert_eq!(
            ed.drag_selection(1, &word, TextSelection::new_range((6, 11))),
            TextSelection::new_range((11, 0))
        );

        // A single press still drags freely, character by character.
        let caret = after_press(PressEventType::Single, (2, 2));
        assert_eq!(
            ed.drag_selection(8, &caret, TextSelection::new_range((2, 2))),
            TextSelection::new_range((2, 8))
        );
    }

    #[test]
    fn a_drag_after_a_triple_press_extends_by_whole_lines() {
        let ed = editor("aaa\nbbb\nccc");
        let line = after_press(PressEventType::Triple, (0, 4));

        assert_eq!(
            ed.drag_selection(2, &line, TextSelection::new_range((0, 4))),
            TextSelection::new_range((0, 4))
        );
        assert_eq!(
            ed.drag_selection(5, &line, TextSelection::new_range((0, 4))),
            TextSelection::new_range((0, 8))
        );
    }

    #[test]
    fn home_and_end_move_by_line_and_primary_widens_them_to_the_document() {
        let mut ed = editor("hello world\nsecond line");

        place(&mut ed, 5);
        press(&mut ed, NamedKey::End);
        // In front of the line break, never past it.
        assert_eq!(ed.cursor_pos(), 11);
        press(&mut ed, NamedKey::Home);
        assert_eq!(ed.cursor_pos(), 0);

        place(&mut ed, 15);
        press(&mut ed, NamedKey::Home);
        assert_eq!(ed.cursor_pos(), 12);
        press(&mut ed, NamedKey::End);
        assert_eq!(ed.cursor_pos(), 23);

        place(&mut ed, 5);
        press_with(&mut ed, NamedKey::End, PRIMARY);
        assert_eq!(ed.cursor_pos(), 23);
        press_with(&mut ed, NamedKey::Home, PRIMARY);
        assert_eq!(ed.cursor_pos(), 0);
    }

    #[test]
    fn shift_extends_the_selection_over_every_granularity() {
        let mut ed = editor("hello world");

        place(&mut ed, 5);
        press_with(&mut ed, NamedKey::Home, Modifiers::SHIFT);
        assert_eq!(ed.get_selected_text().as_deref(), Some("hello"));

        place(&mut ed, 5);
        press_with(&mut ed, NamedKey::End, Modifiers::SHIFT);
        assert_eq!(ed.get_selected_text().as_deref(), Some(" world"));

        place(&mut ed, 0);
        press_with(&mut ed, NamedKey::ArrowRight, WORD | Modifiers::SHIFT);
        assert_eq!(ed.get_selected_text().as_deref(), Some("hello"));
        // A second press grows the same selection rather than starting a new one.
        press_with(&mut ed, NamedKey::ArrowRight, WORD | Modifiers::SHIFT);
        assert_eq!(ed.get_selected_text().as_deref(), Some("hello world"));
    }

    #[test]
    fn a_plain_arrow_collapses_a_selection_to_the_end_it_points_at() {
        let mut ed = editor("hello world");

        // Dragged left to right, and then the other way: the arrow answers the same,
        // because the caret follows the arrow rather than the drag.
        for (from, to) in [(2, 8), (8, 2)] {
            ed.set_selection((from, to));
            press(&mut ed, NamedKey::ArrowLeft);
            assert_eq!(ed.cursor_pos(), 2);
            assert!(ed.get_selection().is_none());

            ed.set_selection((from, to));
            press(&mut ed, NamedKey::ArrowRight);
            assert_eq!(ed.cursor_pos(), 8);
            assert!(ed.get_selection().is_none());
        }

        // A modified arrow collapses to that same end and then travels from it.
        ed.set_selection((2, 8));
        press_with(&mut ed, NamedKey::ArrowLeft, WORD);
        assert_eq!(ed.cursor_pos(), 0);
    }

    #[test]
    fn an_arrow_step_collapses_a_selection_and_still_changes_line() {
        let mut ed = editor("aaa\nbbb\nccc");

        ed.set_selection((4, 6));
        press(&mut ed, NamedKey::ArrowUp);
        assert_eq!(ed.cursor_pos(), 0);

        ed.set_selection((4, 6));
        press(&mut ed, NamedKey::ArrowDown);
        assert_eq!(ed.cursor_pos(), 10);
    }

    #[test]
    fn word_jumps_and_word_deletes_agree() {
        let mut ed = editor("hello world");

        place(&mut ed, 0);
        press_with(&mut ed, NamedKey::ArrowRight, WORD);
        assert_eq!(ed.cursor_pos(), 5);
        press_with(&mut ed, NamedKey::ArrowLeft, WORD);
        assert_eq!(ed.cursor_pos(), 0);

        // Backspace removes exactly what the leftward jump skipped over.
        place(&mut ed, 11);
        press_with(&mut ed, NamedKey::Backspace, WORD);
        assert_eq!(ed.rope().to_string(), "hello ");
        assert_eq!(ed.cursor_pos(), 6);

        let mut ed = editor("hello world");
        place(&mut ed, 0);
        press_with(&mut ed, NamedKey::Delete, WORD);
        assert_eq!(ed.rope().to_string(), " world");
        assert_eq!(ed.cursor_pos(), 0);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn cmd_moves_and_deletes_to_the_line_bounds_without_touching_the_break() {
        let mut ed = editor("hello world\nsecond line");

        place(&mut ed, 5);
        press_with(&mut ed, NamedKey::ArrowRight, Modifiers::META);
        assert_eq!(ed.cursor_pos(), 11);
        press_with(&mut ed, NamedKey::ArrowLeft, Modifiers::META);
        assert_eq!(ed.cursor_pos(), 0);

        place(&mut ed, 5);
        press_with(
            &mut ed,
            NamedKey::ArrowLeft,
            Modifiers::META | Modifiers::SHIFT,
        );
        assert_eq!(ed.get_selected_text().as_deref(), Some("hello"));

        // Cmd+Up/Down reach the document ends.
        place(&mut ed, 5);
        press_with(&mut ed, NamedKey::ArrowDown, Modifiers::META);
        assert_eq!(ed.cursor_pos(), 23);
        press_with(&mut ed, NamedKey::ArrowUp, Modifiers::META);
        assert_eq!(ed.cursor_pos(), 0);

        let mut ed = editor("hello world\nsecond line");
        place(&mut ed, 5);
        press_with(&mut ed, NamedKey::Backspace, Modifiers::META);
        assert_eq!(ed.rope().to_string(), " world\nsecond line");
        assert_eq!(ed.cursor_pos(), 0);

        let mut ed = editor("hello world\nsecond line");
        place(&mut ed, 5);
        press_with(&mut ed, NamedKey::Delete, Modifiers::META);
        // The line break survives: this deletes to the end of the line, not through it.
        assert_eq!(ed.rope().to_string(), "hello\nsecond line");
        assert_eq!(ed.cursor_pos(), 5);
    }

    #[test]
    fn cursor_moves_over_whole_graphemes() {
        let mut ed = editor("a🙂👨‍👩‍👧e\u{301}\nb");

        let mut positions = Vec::new();
        while ed.cursor_right() {
            positions.push(ed.cursor_pos());
        }
        assert_eq!(positions, [1, 3, 11, 13, 14, 15]);

        positions.clear();
        while ed.cursor_left() {
            positions.push(ed.cursor_pos());
        }
        assert_eq!(positions, [14, 13, 11, 3, 1, 0]);
    }

    #[test]
    fn backspace_and_delete_remove_whole_graphemes() {
        let mut ed = editor("🙂a🙂");

        press(&mut ed, NamedKey::Delete);
        assert_eq!(ed.rope().to_string(), "a🙂");
        assert_eq!(ed.cursor_pos(), 0);

        place(&mut ed, 3);
        press(&mut ed, NamedKey::Backspace);
        assert_eq!(ed.rope().to_string(), "a");
        assert_eq!(ed.cursor_pos(), 1);
    }

    #[test]
    fn preedit_lifecycle() {
        let mut ed = editor("Hello World");
        // Place cursor at position 5 ("Hello| World")
        place(&mut ed, 5);

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
        place(&mut ed, 5);
        assert!(!ed.history.can_undo());

        // Insert preedit, should NOT create undo history
        ed.set_preedit("XY");
        assert!(!ed.history.can_undo());
        assert_eq!(ed.rope().to_string(), "HelloXY");

        // Replace preedit, still no undo history
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
