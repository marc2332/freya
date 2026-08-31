use std::{
    borrow::Cow,
    fmt::Display,
    ops::{
        Mul,
        Range,
    },
    time::Duration,
};

use freya_components::scrollviews::ScrollController;
use freya_core::{
    elements::paragraph::ParagraphHolderInner,
    prelude::*,
};
use freya_edit::*;
use ropey::Rope;
use torin::geometry::Size2D;
use tree_sitter::InputEdit;

use crate::{
    editor_theme::EditorSyntaxTheme,
    languages::EditorLanguage,
    metrics::EditorMetrics,
    syntax::InputEditExt,
};

pub struct CodeEditorData {
    pub(crate) history: EditorHistory,
    pub rope: Rope,
    pub(crate) selection: TextSelection,
    pub(crate) last_saved_history_change: usize,
    pub(crate) metrics: EditorMetrics,
    pub(crate) dragging: TextDragging,
    pub(crate) pending_edit: Option<InputEdit>,
    pub language: Option<EditorLanguage>,
    pub scroll_controller: ScrollController,
    viewport: Size2D,
    theme: EditorSyntaxTheme,
}

impl CodeEditorData {
    /// Creates the editor data for the given [`Rope`] and wit hthe given language.
    pub fn create(rope: Rope, language: impl Into<Option<EditorLanguage>>) -> Self {
        let mut data = Self {
            rope,
            selection: TextSelection::new_cursor(0),
            history: EditorHistory::new(Duration::from_secs(1)),
            last_saved_history_change: 0,
            metrics: EditorMetrics::new(),
            dragging: TextDragging::default(),
            pending_edit: None,
            language: language.into(),
            scroll_controller: ScrollController::new(0, 0, Vec::new()),
            viewport: Size2D::default(),
            theme: EditorSyntaxTheme::default(),
        };
        data.configure_highlighter();
        data
    }

    /// Size of the visible area, kept up to date by [`CodeEditor`](crate::editor_ui::CodeEditor).
    pub fn viewport(&self) -> Size2D {
        self.viewport
    }

    /// Mutable access to the size of the visible area.
    pub fn viewport_mut(&mut self) -> &mut Size2D {
        &mut self.viewport
    }

    /// Scrolls the viewport vertically just enough to make the cursor line visible.
    ///
    /// Returns whether the scroll position changed.
    pub fn scroll_to_cursor(&mut self, line_height: f32) -> bool {
        if self.viewport.height <= 0. || line_height <= 0. {
            return false;
        }

        let (_, scroll_y) = self.scroll_controller.into();
        let scrolled = -scroll_y as f32;
        let cursor_top = self.cursor_row() as f32 * line_height;
        let cursor_bottom = cursor_top + line_height;

        let scrolled = if cursor_top < scrolled {
            cursor_top
        } else if cursor_bottom > scrolled + self.viewport.height {
            cursor_bottom - self.viewport.height
        } else {
            return false;
        };

        self.scroll_controller.scroll_to_y(-scrolled as i32)
    }

    /// Reconfigures the highlighter with the current language and theme.
    fn configure_highlighter(&mut self) {
        self.metrics
            .highlighter
            .set_language(self.language.as_ref(), &self.theme);
    }

    /// Sets the language used for syntax highlighting, or disables it with `None`.
    pub fn set_language(&mut self, language: impl Into<Option<EditorLanguage>>) {
        self.language = language.into();
        self.configure_highlighter();
    }

    pub fn is_edited(&self) -> bool {
        self.history.current_change() != self.last_saved_history_change
    }

    pub fn mark_as_saved(&mut self) {
        self.last_saved_history_change = self.history.current_change();
    }

    pub fn parse(&mut self) {
        let edit = self.pending_edit.take();
        self.metrics.run_parser(&self.rope, edit, &self.theme);
    }

    pub fn measure(&mut self, font_size: f32, font_family: &str) {
        self.metrics
            .measure_longest_line(font_size, font_family, &self.rope);
    }

    pub fn set_theme(&mut self, theme: EditorSyntaxTheme) {
        self.theme = theme;
        self.configure_highlighter();
    }

    pub fn process(
        &mut self,
        font_size: f32,
        line_height: f32,
        font_family: &str,
        edit_event: EditableEvent,
    ) -> bool {
        let mut processed = false;
        match edit_event {
            EditableEvent::Down {
                location,
                editor_line,
                holder,
            } => {
                let holder = holder.0.borrow();
                let ParagraphHolderInner {
                    paragraph,
                    scale_factor,
                } = holder.as_ref().unwrap();

                let current_selection = self.selection().clone();

                if self.dragging.shift || self.dragging.clicked {
                    self.selection_mut().set_as_range();
                } else {
                    self.clear_selection();
                }

                if &current_selection != self.selection() {
                    processed = true;
                }

                self.dragging.clicked = true;

                let char_position = paragraph.get_glyph_position_at_coordinate(
                    location.mul(*scale_factor).to_i32().to_tuple(),
                );
                let press_selection =
                    self.measure_selection(char_position.position as usize, editor_line);

                let new_selection = match EventsCombos::pressed(location) {
                    PressEventType::Quadruple => {
                        TextSelection::new_range((0, self.rope.len_utf16_cu()))
                    }
                    PressEventType::Triple => {
                        let line = self.char_to_line(press_selection.pos());
                        let line_char = self.line_to_char(line);
                        let line_len = self.line(line).unwrap().utf16_len();
                        TextSelection::new_range((line_char, line_char + line_len))
                    }
                    PressEventType::Double => {
                        let range = self.find_word_boundaries(press_selection.pos());
                        TextSelection::new_range(range)
                    }
                    PressEventType::Single => press_selection,
                };

                if *self.selection() != new_selection {
                    *self.selection_mut() = new_selection;
                    processed = true;
                }
            }
            EditableEvent::Move {
                location,
                editor_line,
                holder,
            } => {
                if self.dragging.clicked {
                    let paragraph = holder.0.borrow();
                    let ParagraphHolderInner {
                        paragraph,
                        scale_factor,
                    } = paragraph.as_ref().unwrap();

                    let dist_position = location.mul(*scale_factor);

                    // Calculate the end of the highlighting
                    let dist_char = paragraph
                        .get_glyph_position_at_coordinate(dist_position.to_i32().to_tuple());
                    let to = dist_char.position as usize;

                    if self.get_selection().is_none() {
                        self.selection_mut().set_as_range();
                        processed = true;
                    }

                    let current_selection = self.selection().clone();

                    let new_selection = self.measure_selection(to, editor_line);

                    // Update the cursor if it has changed
                    if current_selection != new_selection {
                        *self.selection_mut() = new_selection;
                        processed = true;
                    }
                }
            }
            EditableEvent::Release => {
                self.dragging.clicked = false;
            }
            EditableEvent::KeyDown {
                key,
                modifiers,
                editor_line,
                holder,
            } => {
                match key {
                    // Handle dragging
                    Key::Named(NamedKey::Shift) => {
                        self.dragging.shift = true;
                    }
                    // Handle editing
                    _ => {
                        let event = self.process_key(
                            key,
                            &modifiers,
                            editor_line,
                            holder,
                            true,
                            true,
                            true,
                            true,
                        );
                        if event.contains(TextEvent::TEXT_CHANGED) {
                            self.parse();
                            self.measure(font_size, font_family);
                            self.dragging = TextDragging::default();
                        }
                        if !event.is_empty() {
                            self.scroll_to_cursor(line_height);
                            processed = true;
                        }
                    }
                }
            }
            EditableEvent::KeyUp { key, .. } => {
                if *key == Key::Named(NamedKey::Shift) {
                    self.dragging.shift = false;
                }
            }
        };
        processed
    }
}

impl Display for CodeEditorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.rope.to_string())
    }
}

impl TextEditor for CodeEditorData {
    type LinesIterator<'a>
        = LinesIterator<'a>
    where
        Self: 'a;

    fn lines(&self) -> Self::LinesIterator<'_> {
        unimplemented!("Unused.")
    }

    fn text(&self) -> Cow<'_, str> {
        self.rope.slice(..).into()
    }

    fn insert_char(&mut self, ch: char, idx: usize) -> usize {
        let idx_utf8 = self.utf16_cu_to_char(idx);
        let selection = self.selection.clone();

        // Capture byte offset and position before mutation for InputEdit.
        let start_byte = self.rope.char_to_byte(idx_utf8);
        let start_line = self.rope.char_to_line(idx_utf8);
        let start_line_byte = self.rope.line_to_byte(start_line);
        let start_col = start_byte - start_line_byte;

        let len_before_insert = self.rope.len_utf16_cu();
        self.rope.insert_char(idx_utf8, ch);
        let len_after_insert = self.rope.len_utf16_cu();

        let inserted_text_len = len_after_insert - len_before_insert;

        // Compute new end position after insertion.
        let new_end_char = idx_utf8 + 1; // one char inserted
        let new_end_byte = self.rope.char_to_byte(new_end_char);
        let new_end_line = self.rope.char_to_line(new_end_char);
        let new_end_line_byte = self.rope.line_to_byte(new_end_line);
        let new_end_col = new_end_byte - new_end_line_byte;

        self.pending_edit = Some(InputEdit::new_edit(
            start_byte,
            start_byte,
            new_end_byte,
            (start_line, start_col),
            (start_line, start_col),
            (new_end_line, new_end_col),
        ));

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

        // Capture byte offset and position before mutation for InputEdit.
        let start_byte = self.rope.char_to_byte(idx_utf8);
        let start_line = self.rope.char_to_line(idx_utf8);
        let start_line_byte = self.rope.line_to_byte(start_line);
        let start_col = start_byte - start_line_byte;

        let len_before_insert = self.rope.len_utf16_cu();
        self.rope.insert(idx_utf8, text);
        let len_after_insert = self.rope.len_utf16_cu();

        let inserted_text_len = len_after_insert - len_before_insert;

        // Compute new end position after insertion.
        let inserted_chars = text.chars().count();
        let new_end_char = idx_utf8 + inserted_chars;
        let new_end_byte = self.rope.char_to_byte(new_end_char);
        let new_end_line = self.rope.char_to_line(new_end_char);
        let new_end_line_byte = self.rope.line_to_byte(new_end_line);
        let new_end_col = new_end_byte - new_end_line_byte;

        self.pending_edit = Some(InputEdit::new_edit(
            start_byte,
            start_byte,
            new_end_byte,
            (start_line, start_col),
            (start_line, start_col),
            (new_end_line, new_end_col),
        ));

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

        // Capture byte offsets and positions before mutation for InputEdit.
        let start_byte = self.rope.char_to_byte(range.start);
        let old_end_byte = self.rope.char_to_byte(range.end);
        let start_line = self.rope.char_to_line(range.start);
        let start_line_byte = self.rope.line_to_byte(start_line);
        let start_col = start_byte - start_line_byte;
        let old_end_line = self.rope.char_to_line(range.end);
        let old_end_line_byte = self.rope.line_to_byte(old_end_line);
        let old_end_col = old_end_byte - old_end_line_byte;

        let len_before_remove = self.rope.len_utf16_cu();
        self.rope.remove(range);
        let len_after_remove = self.rope.len_utf16_cu();

        let removed_text_len = len_before_remove - len_after_remove;

        // After removal, new_end == start (the removed range collapses to a point).
        self.pending_edit = Some(InputEdit::new_edit(
            start_byte,
            old_end_byte,
            start_byte,
            (start_line, start_col),
            (old_end_line, old_end_col),
            (start_line, start_col),
        ));

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
            text: Cow::Owned(line.to_string()),
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

        Some(self.rope.get_slice(start..end)?.to_string())
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
        // Undo can make arbitrary changes, therefore invalidate the tree for a full re-parse.
        self.pending_edit = None;
        self.metrics.highlighter.invalidate_tree();
        self.history.undo(&mut self.rope)
    }

    fn redo(&mut self) -> Option<TextSelection> {
        // Redo can make arbitrary changes, therefore invalidate the tree for a full re-parse.
        self.pending_edit = None;
        self.metrics.highlighter.invalidate_tree();
        self.history.redo(&mut self.rope)
    }

    fn editor_history(&self) -> &EditorHistory {
        &self.history
    }

    fn editor_history_mut(&mut self) -> &mut EditorHistory {
        &mut self.history
    }

    fn selection(&self) -> &TextSelection {
        &self.selection
    }

    fn selection_mut(&mut self) -> &mut TextSelection {
        &mut self.selection
    }

    fn get_indentation(&self) -> u8 {
        4
    }
}
