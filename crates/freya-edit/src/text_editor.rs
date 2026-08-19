use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt::Display,
    ops::Range,
};

use freya_clipboard::clipboard::Clipboard;
use freya_core::{
    elements::paragraph::{
        ParagraphCursorExt,
        ParagraphHolder,
        ParagraphHolderInner,
    },
    events::modifiers::ModifiersExt,
};
use keyboard_types::{
    Key,
    Modifiers,
    NamedKey,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::editor_history::EditorHistory;

#[derive(PartialEq, Clone, Debug, Copy, Hash)]
pub enum EditorLine {
    /// Only one `paragraph` element exists in the whole editor.
    SingleParagraph,
    /// There are multiple `paragraph` elements in the editor, one per line.
    Paragraph(usize),
}

/// Holds the position of a cursor in a text
#[derive(Clone, PartialEq, Debug)]
pub enum TextSelection {
    Cursor(usize),
    Range { from: usize, to: usize },
}

impl TextSelection {
    /// Create a new [TextSelection::Cursor]
    pub fn new_cursor(pos: usize) -> Self {
        Self::Cursor(pos)
    }

    /// Create a new [TextSelection::Range]
    pub fn new_range((from, to): (usize, usize)) -> Self {
        Self::Range { from, to }
    }

    /// Get the position
    pub fn pos(&self) -> usize {
        self.end()
    }

    /// Set the selection as a cursor
    pub fn set_as_cursor(&mut self) {
        *self = Self::Cursor(self.end())
    }

    /// Set the selection as a range
    pub fn set_as_range(&mut self) {
        *self = Self::Range {
            from: self.start(),
            to: self.end(),
        }
    }

    /// Get the start of the cursor position.
    pub fn start(&self) -> usize {
        match self {
            Self::Cursor(pos) => *pos,
            Self::Range { from, .. } => *from,
        }
    }

    /// Get the end of the cursor position.
    pub fn end(&self) -> usize {
        match self {
            Self::Cursor(pos) => *pos,
            Self::Range { to, .. } => *to,
        }
    }

    /// Move the end position of the cursor.
    pub fn move_to(&mut self, position: usize) {
        match self {
            Self::Cursor(pos) => *pos = position,
            Self::Range { to, .. } => {
                *to = position;
            }
        }
    }

    pub fn is_range(&self) -> bool {
        matches!(self, Self::Range { .. })
    }
}

/// A text line from a [TextEditor]
#[derive(Clone)]
pub struct Line<'a> {
    pub text: Cow<'a, str>,
    pub utf16_len: usize,
}

impl Line<'_> {
    /// Get the length of the line
    pub fn utf16_len(&self) -> usize {
        self.utf16_len
    }
}

impl Display for Line<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}

bitflags::bitflags! {
    /// Events for [TextEditor]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
    pub struct TextEvent: u8 {
         /// Cursor position has been moved
        const CURSOR_CHANGED = 0x01;
        /// Text has changed
        const TEXT_CHANGED = 0x02;
        /// Selected text has changed
        const SELECTION_CHANGED = 0x04;
    }
}

/// Common trait for editable texts
pub trait TextEditor {
    type LinesIterator<'a>: Iterator<Item = Line<'a>>
    where
        Self: 'a;

    fn set(&mut self, text: &str);

    /// Iterator over all the lines in the text.
    fn lines(&self) -> Self::LinesIterator<'_>;

    /// Insert a character in the text in the given position.
    fn insert_char(&mut self, char: char, char_idx: usize) -> usize;

    /// Insert a string in the text in the given position.
    fn insert(&mut self, text: &str, char_idx: usize) -> usize;

    /// Remove a part of the text.
    fn remove(&mut self, range: Range<usize>) -> usize;

    /// Get line from the given char
    fn char_to_line(&self, char_idx: usize) -> usize;

    /// Get the first char from the given line
    fn line_to_char(&self, line_idx: usize) -> usize;

    fn utf16_cu_to_char(&self, utf16_cu_idx: usize) -> usize;

    fn char_to_utf16_cu(&self, idx: usize) -> usize;

    /// Get a line from the text
    fn line(&self, line_idx: usize) -> Option<Line<'_>>;

    fn text(&self) -> Cow<'_, str>;

    /// Total of lines
    fn len_lines(&self) -> usize;

    /// Total of chars
    fn len_chars(&self) -> usize;

    /// Total of utf16 code units
    fn len_utf16_cu(&self) -> usize;

    /// Get a readable text selection
    fn selection(&self) -> &TextSelection;

    /// Get a mutable reference to text selection
    fn selection_mut(&mut self) -> &mut TextSelection;

    /// Get the UTF-16 range of the grapheme cluster containing the given position.
    fn grapheme_cluster_at(&self, pos_utf16: usize) -> Range<usize> {
        let line_idx = self.char_to_line(self.utf16_cu_to_char(pos_utf16));
        let line_start = self.char_to_utf16_cu(self.line_to_char(line_idx));
        let Some(line) = self.line(line_idx) else {
            return pos_utf16..pos_utf16;
        };

        let mut cluster = line_start..line_start;
        for grapheme in line.text.graphemes(true) {
            cluster = cluster.end..cluster.end + grapheme.encode_utf16().count();
            if cluster.end > pos_utf16 {
                return cluster;
            }
        }
        pos_utf16..pos_utf16
    }

    /// Get the cursor row
    fn cursor_row(&self) -> usize {
        let pos = self.cursor_pos();
        let pos_utf8 = self.utf16_cu_to_char(pos);
        self.char_to_line(pos_utf8)
    }

    /// Get the cursor column
    fn cursor_col(&self) -> usize {
        let pos = self.cursor_pos();
        let pos_utf8 = self.utf16_cu_to_char(pos);
        let line = self.char_to_line(pos_utf8);
        let line_char_utf8 = self.line_to_char(line);
        let line_char = self.char_to_utf16_cu(line_char_utf8);
        pos - line_char
    }

    /// Last valid cursor position in `row`, before the line ending if it has one.
    fn line_end_position(&self, row: usize) -> Option<usize> {
        let line = self.line(row)?;
        let row_start = self.char_to_utf16_cu(self.line_to_char(row));
        let row_end = row_start + line.utf16_len();
        if row + 1 == self.len_lines() {
            Some(row_end)
        } else {
            Some(self.grapheme_cluster_at(row_end - 1).start)
        }
    }

    /// Move the cursor to `row`, keeping `col` when possible and snapping to a
    /// grapheme cluster boundary.
    fn move_cursor_to_row(&mut self, row: usize, col: usize) {
        let Some(last_position) = self.line_end_position(row) else {
            return;
        };
        let row_start = self.char_to_utf16_cu(self.line_to_char(row));
        let col = col.min(last_position - row_start);
        let pos = self.grapheme_cluster_at(row_start + col).start;
        self.selection_mut().move_to(pos);
    }

    /// Move the cursor 1 line down
    fn cursor_down(
        &mut self,
        editor_line: Option<EditorLine>,
        holder: Option<&ParagraphHolder>,
    ) -> bool {
        if let Some(editor_line) = editor_line
            && let Some(holder) = holder
            && let Some(position) = self.visual_line_position(holder, editor_line, 1)
        {
            self.selection_mut().move_to(position);

            return true;
        }

        let old_row = self.cursor_row();
        let old_col = self.cursor_col();

        match old_row.cmp(&(self.len_lines() - 1)) {
            Ordering::Less => {
                self.move_cursor_to_row(old_row + 1, old_col);

                true
            }
            Ordering::Equal => {
                let end = self.len_utf16_cu();
                self.selection_mut().move_to(end);

                true
            }
            Ordering::Greater => false,
        }
    }

    /// Move the cursor 1 line up
    fn cursor_up(
        &mut self,
        editor_line: Option<EditorLine>,
        holder: Option<&ParagraphHolder>,
    ) -> bool {
        if let Some(editor_line) = editor_line
            && let Some(holder) = holder
            && let Some(position) = self.visual_line_position(holder, editor_line, -1)
        {
            self.selection_mut().move_to(position);

            return true;
        }

        let pos = self.cursor_pos();
        let old_row = self.cursor_row();
        let old_col = self.cursor_col();

        if pos > 0 {
            if old_row == 0 {
                self.selection_mut().move_to(0);
            } else {
                self.move_cursor_to_row(old_row - 1, old_col);
            }

            true
        } else {
            false
        }
    }

    /// Cursor position one visual line up or down.
    fn visual_line_position(
        &self,
        holder: &ParagraphHolder,
        editor_line: EditorLine,
        line_offset: isize,
    ) -> Option<usize> {
        let holder = holder.0.borrow();
        let ParagraphHolderInner { paragraph, .. } = holder.as_ref()?;

        if !matches!(editor_line, EditorLine::SingleParagraph) {
            return None;
        }

        let cursor_rect = paragraph.measured_cursor_rect(&self.text(), self.cursor_pos())?;

        let lines = paragraph.get_line_metrics();
        let current = lines
            .iter()
            .position(|line| (cursor_rect.top as f64) < line.baseline + line.descent)?;
        let line = lines.get(current.checked_add_signed(line_offset)?)?;

        // Clamp to the end of soft wrapped lines
        let mut horizontal_position = cursor_rect.left as i32;
        if !line.hard_break {
            horizontal_position = horizontal_position.min((line.left + line.width) as i32 - 1);
        }

        let position = paragraph
            .get_glyph_position_at_coordinate((horizontal_position, line.baseline as i32))
            .position
            .max(0) as usize;

        Some(position)
    }

    /// Move the cursor 1 grapheme cluster to the right
    fn cursor_right(&mut self) -> bool {
        if self.cursor_pos() < self.len_utf16_cu() {
            let to = self.grapheme_cluster_at(self.selection().end()).end;
            self.selection_mut().move_to(to);

            true
        } else {
            false
        }
    }

    /// Move the cursor 1 grapheme cluster to the left
    fn cursor_left(&mut self) -> bool {
        if self.cursor_pos() > 0 {
            let to = self.grapheme_cluster_at(self.selection().end() - 1).start;
            self.selection_mut().move_to(to);

            true
        } else {
            false
        }
    }

    /// Find the end of the next word from the given position, if any.
    fn next_word_pos(&self, pos: usize) -> Option<usize> {
        let len = self.len_utf16_cu();
        if pos >= len {
            return None;
        }

        // Walk forward line by line starting at the given position.
        let start_char = self.utf16_cu_to_char(pos);
        let initial_line = self.char_to_line(start_char);
        let initial_offset = start_char - self.line_to_char(initial_line);

        for line_idx in initial_line..self.len_lines() {
            let Some(line) = self.line(line_idx) else {
                continue;
            };
            let line_char_offset = self.line_to_char(line_idx);
            let from = if line_idx == initial_line {
                initial_offset
            } else {
                0
            };

            // Stop at the end of the first non-whitespace segment past the position.
            let mut char_offset = 0;
            for word in line.text.split_word_bounds() {
                char_offset += word.chars().count();
                if char_offset > from && !word.chars().all(char::is_whitespace) {
                    return Some(self.char_to_utf16_cu(line_char_offset + char_offset));
                }
            }
        }

        // Trailing whitespace only, snap to text end.
        Some(len)
    }

    /// Find the start of the previous word from the given position, if any.
    fn prev_word_pos(&self, pos: usize) -> Option<usize> {
        if pos == 0 {
            return None;
        }

        // Walk backward line by line starting at the given position.
        let start_char = self.utf16_cu_to_char(pos);
        let initial_line = self.char_to_line(start_char);
        let initial_offset = start_char - self.line_to_char(initial_line);

        for line_idx in (0..=initial_line).rev() {
            let Some(line) = self.line(line_idx) else {
                continue;
            };
            let line_char_offset = self.line_to_char(line_idx);
            let to = if line_idx == initial_line {
                initial_offset
            } else {
                line.text.chars().count()
            };

            // Track the latest non-whitespace segment that starts before the position.
            let mut char_offset = 0;
            let mut last_word_start = None;
            for word in line.text.split_word_bounds() {
                if char_offset >= to {
                    break;
                }
                if !word.chars().all(char::is_whitespace) {
                    last_word_start = Some(char_offset);
                }
                char_offset += word.chars().count();
            }

            if let Some(start) = last_word_start {
                return Some(self.char_to_utf16_cu(line_char_offset + start));
            }
        }

        // Leading whitespace only, snap to text start.
        Some(0)
    }

    /// Move the cursor to the end of the next word.
    fn cursor_word_right(&mut self) -> bool {
        if let Some(new_pos) = self.next_word_pos(self.cursor_pos()) {
            self.selection_mut().move_to(new_pos);
            true
        } else {
            false
        }
    }

    /// Move the cursor to the start of the previous word.
    fn cursor_word_left(&mut self) -> bool {
        if let Some(new_pos) = self.prev_word_pos(self.cursor_pos()) {
            self.selection_mut().move_to(new_pos);
            true
        } else {
            false
        }
    }

    /// Get the cursor position
    fn cursor_pos(&self) -> usize {
        self.selection().pos()
    }

    /// Move the cursor position
    fn move_cursor_to(&mut self, pos: usize) {
        self.selection_mut().move_to(pos);
    }

    // Check if has any selection at all
    fn has_any_selection(&self) -> bool;

    // Return the selected text
    fn get_selection(&self) -> Option<(usize, usize)>;

    // Return the visible selected text for the given editor line
    fn get_visible_selection(&self, editor_line: EditorLine) -> Option<(usize, usize)> {
        let (selected_from, selected_to) = match self.selection() {
            TextSelection::Cursor(_) => return None,
            TextSelection::Range { from, to } => (*from, *to),
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

    // Remove the selection
    fn clear_selection(&mut self);

    // Select some text
    fn set_selection(&mut self, selected: (usize, usize));

    // Measure a new text selection

    fn measure_selection(&self, to: usize, line_index: EditorLine) -> TextSelection {
        let mut selection = self.selection().clone();

        match line_index {
            EditorLine::Paragraph(line_index) => {
                let row_char = self.line_to_char(line_index);
                let pos = self.char_to_utf16_cu(row_char) + to;
                selection.move_to(pos);
            }
            EditorLine::SingleParagraph => {
                selection.move_to(to);
            }
        }

        selection
    }

    // Process a Keyboard event
    #[allow(clippy::too_many_arguments)]
    fn process_key(
        &mut self,
        key: &Key,
        modifiers: &Modifiers,
        editor_line: Option<EditorLine>,
        holder: Option<&ParagraphHolder>,
        allow_tabs: bool,
        allow_changes: bool,
        allow_read_clipboard: bool,
        allow_write_clipboard: bool,
    ) -> TextEvent {
        let mut event = TextEvent::empty();

        let selection = self.get_selection();
        let skip_arrows_movement = !modifiers.contains(Modifiers::SHIFT) && selection.is_some();
        let word_jump = modifiers.contains(Modifiers::ctrl_or_alt());

        match key {
            Key::Named(NamedKey::Shift) => {}
            Key::Named(NamedKey::Control) => {}
            Key::Named(NamedKey::Alt) => {}
            Key::Named(NamedKey::Escape) => {
                self.clear_selection();
            }
            Key::Named(NamedKey::ArrowDown) => {
                if modifiers.contains(Modifiers::SHIFT) {
                    self.selection_mut().set_as_range();
                } else {
                    self.selection_mut().set_as_cursor();
                }

                if !skip_arrows_movement && self.cursor_down(editor_line, holder) {
                    event.insert(TextEvent::CURSOR_CHANGED);
                }
            }
            Key::Named(NamedKey::ArrowLeft) => {
                if modifiers.contains(Modifiers::SHIFT) {
                    self.selection_mut().set_as_range();
                } else {
                    self.selection_mut().set_as_cursor();
                }

                let moved = !skip_arrows_movement
                    && if word_jump {
                        self.cursor_word_left()
                    } else {
                        self.cursor_left()
                    };

                if moved {
                    event.insert(TextEvent::CURSOR_CHANGED);
                }
            }
            Key::Named(NamedKey::ArrowRight) => {
                if modifiers.contains(Modifiers::SHIFT) {
                    self.selection_mut().set_as_range();
                } else {
                    self.selection_mut().set_as_cursor();
                }

                let moved = !skip_arrows_movement
                    && if word_jump {
                        self.cursor_word_right()
                    } else {
                        self.cursor_right()
                    };

                if moved {
                    event.insert(TextEvent::CURSOR_CHANGED);
                }
            }
            Key::Named(NamedKey::ArrowUp) => {
                if modifiers.contains(Modifiers::SHIFT) {
                    self.selection_mut().set_as_range();
                } else {
                    self.selection_mut().set_as_cursor();
                }

                if !skip_arrows_movement && self.cursor_up(editor_line, holder) {
                    event.insert(TextEvent::CURSOR_CHANGED);
                }
            }
            Key::Named(named_key @ (NamedKey::Home | NamedKey::End)) => {
                if modifiers.contains(Modifiers::SHIFT) {
                    self.selection_mut().set_as_range();
                } else {
                    self.selection_mut().set_as_cursor();
                }

                let whole_text = modifiers.contains(Modifiers::ctrl_or_meta());
                let pos = match (named_key, whole_text) {
                    (NamedKey::Home, true) => 0,
                    (NamedKey::Home, false) => {
                        self.char_to_utf16_cu(self.line_to_char(self.cursor_row()))
                    }
                    (_, true) => self.len_utf16_cu(),
                    (_, false) => self
                        .line_end_position(self.cursor_row())
                        .unwrap_or_else(|| self.len_utf16_cu()),
                };

                if pos != self.cursor_pos() {
                    self.selection_mut().move_to(pos);
                    event.insert(TextEvent::CURSOR_CHANGED);
                }
            }
            Key::Named(NamedKey::Backspace) if allow_changes => {
                let cursor_pos = self.cursor_pos();

                let removal = if let Some((start, end)) = self.get_selection_range() {
                    Some(start..end)
                } else if word_jump {
                    self.prev_word_pos(cursor_pos)
                        .map(|start| start..cursor_pos)
                } else if cursor_pos > 0 {
                    Some(self.grapheme_cluster_at(cursor_pos - 1).start..cursor_pos)
                } else {
                    None
                };

                if let Some(removal) = removal {
                    let end = removal.end;
                    let removed_text_len = self.remove(removal);
                    self.move_cursor_to(end - removed_text_len);
                    event.insert(TextEvent::TEXT_CHANGED);
                }
            }
            Key::Named(NamedKey::Delete) if allow_changes => {
                let cursor_pos = self.cursor_pos();

                let removal = if let Some((start, end)) = self.get_selection_range() {
                    Some(start..end)
                } else if word_jump {
                    self.next_word_pos(cursor_pos).map(|end| cursor_pos..end)
                } else if cursor_pos < self.len_utf16_cu() {
                    Some(cursor_pos..self.grapheme_cluster_at(cursor_pos).end)
                } else {
                    None
                };

                if let Some(removal) = removal {
                    let start = removal.start;
                    self.remove(removal);
                    self.move_cursor_to(start);
                    event.insert(TextEvent::TEXT_CHANGED);
                }
            }
            Key::Named(NamedKey::Enter) if allow_changes => {
                // Breaks the line
                let cursor_pos = self.cursor_pos();
                self.insert_char('\n', cursor_pos);
                self.cursor_right();

                event.insert(TextEvent::TEXT_CHANGED);
            }
            Key::Named(NamedKey::Tab) if allow_tabs && allow_changes => {
                // Inserts a tab
                let text = " ".repeat(self.get_indentation().into());
                let cursor_pos = self.cursor_pos();
                self.insert(&text, cursor_pos);
                self.move_cursor_to(cursor_pos + text.chars().count());

                event.insert(TextEvent::TEXT_CHANGED);
            }
            Key::Character(character) => {
                let meta_or_ctrl = modifiers.contains(Modifiers::ctrl_or_meta());

                match character.as_str() {
                    " " if allow_changes => {
                        let selection = self.get_selection_range();
                        if let Some((start, end)) = selection {
                            self.remove(start..end);
                            self.move_cursor_to(start);
                            event.insert(TextEvent::TEXT_CHANGED);
                        }

                        // Simply adds an space
                        let cursor_pos = self.cursor_pos();
                        self.insert_char(' ', cursor_pos);
                        self.cursor_right();

                        event.insert(TextEvent::TEXT_CHANGED);
                    }

                    // Select all text
                    "a" if meta_or_ctrl => {
                        let len = self.len_utf16_cu();
                        self.set_selection((0, len));
                    }

                    // Copy selected text
                    "c" if meta_or_ctrl && allow_write_clipboard => {
                        let selected = self.get_selected_text();
                        if let Some(selected) = selected {
                            Clipboard::set(selected).ok();
                        }
                    }

                    // Cut selected text
                    "x" if meta_or_ctrl && allow_changes && allow_write_clipboard => {
                        let selection = self.get_selection_range();
                        if let Some((start, end)) = selection {
                            let text = self.get_selected_text().unwrap();
                            self.remove(start..end);
                            Clipboard::set(text).ok();
                            self.move_cursor_to(start);
                            event.insert(TextEvent::TEXT_CHANGED);
                        }
                    }

                    // Paste copied text
                    "v" if meta_or_ctrl && allow_changes && allow_read_clipboard => {
                        if let Ok(copied_text) = Clipboard::get() {
                            let selection = self.get_selection_range();
                            if let Some((start, end)) = selection {
                                self.remove(start..end);
                                self.move_cursor_to(start);
                            }
                            let cursor_pos = self.cursor_pos();
                            self.insert(&copied_text, cursor_pos);
                            let last_idx = copied_text.encode_utf16().count() + cursor_pos;
                            self.move_cursor_to(last_idx);
                            event.insert(TextEvent::TEXT_CHANGED);
                        }
                    }

                    // Undo last change
                    "z" if meta_or_ctrl && allow_changes => {
                        let undo_result = self.undo();

                        if let Some(selection) = undo_result {
                            *self.selection_mut() = selection;
                            event.insert(TextEvent::TEXT_CHANGED);
                            event.insert(TextEvent::SELECTION_CHANGED);
                        }
                    }

                    // Redo last change
                    "y" if meta_or_ctrl && allow_changes => {
                        let redo_result = self.redo();

                        if let Some(selection) = redo_result {
                            *self.selection_mut() = selection;
                            event.insert(TextEvent::TEXT_CHANGED);
                            event.insert(TextEvent::SELECTION_CHANGED);
                        }
                    }

                    _ if allow_changes => {
                        // Remove selected text
                        let selection = self.get_selection_range();
                        if let Some((start, end)) = selection {
                            self.remove(start..end);
                            self.move_cursor_to(start);
                            event.insert(TextEvent::TEXT_CHANGED);
                        }

                        if let Ok(ch) = character.parse::<char>() {
                            // Inserts a character
                            let cursor_pos = self.cursor_pos();
                            let inserted_text_len = self.insert_char(ch, cursor_pos);
                            self.move_cursor_to(cursor_pos + inserted_text_len);
                            event.insert(TextEvent::TEXT_CHANGED);
                        } else {
                            // Inserts a text
                            let cursor_pos = self.cursor_pos();
                            let inserted_text_len = self.insert(character, cursor_pos);
                            self.move_cursor_to(cursor_pos + inserted_text_len);
                            event.insert(TextEvent::TEXT_CHANGED);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        if event.contains(TextEvent::TEXT_CHANGED) && !event.contains(TextEvent::SELECTION_CHANGED)
        {
            self.clear_selection();
        }

        if self.get_selection() != selection {
            event.insert(TextEvent::SELECTION_CHANGED);
        }

        event
    }

    fn get_selected_text(&self) -> Option<String>;

    fn undo(&mut self) -> Option<TextSelection>;

    fn redo(&mut self) -> Option<TextSelection>;

    fn editor_history(&self) -> &EditorHistory;

    fn editor_history_mut(&mut self) -> &mut EditorHistory;

    fn get_selection_range(&self) -> Option<(usize, usize)>;

    fn get_indentation(&self) -> u8;

    fn find_word_boundaries(&self, pos: usize) -> (usize, usize) {
        let pos_char = self.utf16_cu_to_char(pos);
        let len_chars = self.len_chars();

        if len_chars == 0 {
            return (pos, pos);
        }

        // Get the line containing the cursor
        let line_idx = self.char_to_line(pos_char);
        let line_char = self.line_to_char(line_idx);
        let line = self.line(line_idx).unwrap();

        let line_str: std::borrow::Cow<str> = line.text;
        let pos_in_line = pos_char - line_char;

        // Find word boundaries within the line
        let mut char_offset = 0;
        for word in line_str.split_word_bounds() {
            let word_char_len = word.chars().count();
            let word_start = char_offset;
            let word_end = char_offset + word_char_len;

            if pos_in_line >= word_start && pos_in_line < word_end {
                let start_char = line_char + word_start;
                let end_char = line_char + word_end;
                return (
                    self.char_to_utf16_cu(start_char),
                    self.char_to_utf16_cu(end_char),
                );
            }

            char_offset = word_end;
        }

        (pos, pos)
    }
}
