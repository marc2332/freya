use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt::Display,
    ops::Range,
};

use freya_clipboard::clipboard::Clipboard;
use freya_core::prelude::PressEventType;
use keyboard_types::{
    Key,
    Modifiers,
    NamedKey,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    config::{
        EditAction,
        EditBindings,
    },
    editor_history::EditorHistory,
    event::TextDragging,
};

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

/// The characters [`ropey`] ends a line on, which [`TextEditor::line`] yields as part of
/// the line's own text.
const LINE_BREAKS: [char; 7] = [
    '\n', '\r', '\u{0B}', '\u{0C}', '\u{85}', '\u{2028}', '\u{2029}',
];

/// How far one caret motion travels. The modifiers held decide it (see
/// [`CaretGranularity::horizontal`] and [`CaretGranularity::vertical`]), and every
/// motion, selection and modified deletion resolves through the same
/// [`TextEditor::caret_target`], so a key added to one of them cannot disagree with
/// the others about where a word or a line ends.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CaretGranularity {
    /// One grapheme cluster.
    Grapheme,
    /// The next word boundary.
    Word,
    /// The start or end of the caret's line, with the line break left outside.
    LineBoundary,
    /// The start or end of the whole text.
    Document,
}

impl CaretGranularity {
    /// The distance a left/right motion or a deletion covers, from the modifiers held.
    ///
    /// macOS puts word jumps on Alt and line jumps on Cmd (arrows, Backspace and
    /// Delete alike); everywhere else the word jump is on Control and there is no
    /// line-jump chord, Home and End serving that role instead.
    pub fn horizontal(modifiers: &Modifiers) -> Self {
        if cfg!(target_os = "macos") {
            if modifiers.contains(Modifiers::META) {
                Self::LineBoundary
            } else if modifiers.contains(Modifiers::ALT) {
                Self::Word
            } else {
                Self::Grapheme
            }
        } else if modifiers.contains(Modifiers::CONTROL) {
            Self::Word
        } else {
            Self::Grapheme
        }
    }

    /// The distance an up/down motion covers. macOS puts document jumps on Cmd; other
    /// platforms leave Control+Up/Down to the viewport and reach the document ends
    /// through Control+Home/End.
    pub fn vertical(modifiers: &Modifiers) -> Option<Self> {
        (cfg!(target_os = "macos") && modifiers.contains(Modifiers::META)).then_some(Self::Document)
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

    /// Move the cursor to `row`, keeping `col` when possible and snapping to a
    /// grapheme cluster boundary.
    fn move_cursor_to_row(&mut self, row: usize, col: usize) {
        let Some(line) = self.line(row) else { return };
        let row_start = self.char_to_utf16_cu(self.line_to_char(row));
        let col = col.min(line.utf16_len().saturating_sub(1));
        let pos = self.grapheme_cluster_at(row_start + col).start;
        self.selection_mut().move_to(pos);
    }

    /// Move the cursor 1 line down
    fn cursor_down(&mut self) -> bool {
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
    fn cursor_up(&mut self) -> bool {
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

    /// The end of the first word past `pos`, or the end of the text when only
    /// whitespace follows.
    fn word_end_after(&self, pos: usize) -> usize {
        let len = self.len_utf16_cu();
        if pos >= len {
            return len;
        }

        // Walk forward line by line starting at `pos`.
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

            // Stop at the end of the first non-whitespace segment past `pos`.
            let mut char_offset = 0;
            for word in line.text.split_word_bounds() {
                char_offset += word.chars().count();
                if char_offset > from && !word.chars().all(char::is_whitespace) {
                    return self.char_to_utf16_cu(line_char_offset + char_offset);
                }
            }
        }

        // Trailing whitespace only, snap to text end.
        len
    }

    /// The start of the last word beginning before `pos`, or the start of the text
    /// when only whitespace precedes it.
    fn word_start_before(&self, pos: usize) -> usize {
        if pos == 0 {
            return 0;
        }

        // Walk backward line by line starting at `pos`.
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

            // Track the latest non-whitespace segment that starts before `pos`.
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

            // Found one on this line, its start is the answer.
            if let Some(start) = last_word_start {
                return self.char_to_utf16_cu(line_char_offset + start);
            }
        }

        // Leading whitespace only, snap to text start.
        0
    }

    /// The UTF-16 bounds of the line holding `pos`, terminator **included**: selecting
    /// this and removing it removes the line, which is what a triple press means.
    fn line_span(&self, pos: usize) -> Range<usize> {
        let line_idx = self.char_to_line(self.utf16_cu_to_char(pos));
        let start = self.char_to_utf16_cu(self.line_to_char(line_idx));
        start..start + self.line(line_idx).map_or(0, |line| line.utf16_len())
    }

    /// [`Self::line_span`] with the line terminator left **outside**: where the caret
    /// stops, and how far a delete-to-line-end reaches. Neither may cross into the next
    /// line, so these are two answers rather than one.
    fn line_bounds(&self, pos: usize) -> Range<usize> {
        let span = self.line_span(pos);
        let Some(line) = self.line(self.char_to_line(self.utf16_cu_to_char(pos))) else {
            return span;
        };
        let text = line.text.as_ref();
        let body = text.trim_end_matches(LINE_BREAKS);
        span.start..span.end - text[body.len()..].encode_utf16().count()
    }

    /// The selection one press makes: the caret for a single press, then the word, the
    /// line, and the whole text. `at` is the position under the pointer and `current`
    /// the selection the press starts from, already widened to a range when Shift is
    /// held.
    fn press_selection(
        &self,
        at: usize,
        press: PressEventType,
        current: TextSelection,
    ) -> TextSelection {
        match press {
            PressEventType::Single => current,
            PressEventType::Double => TextSelection::new_range(self.find_word_boundaries(at)),
            PressEventType::Triple => {
                let span = self.line_span(at);
                TextSelection::new_range((span.start, span.end))
            }
            PressEventType::Quadruple => TextSelection::new_range((0, self.len_utf16_cu())),
        }
    }

    /// The selection a drag makes: it extends from the range the press established by
    /// the **unit that press used**, so a drag after a double press moves word by word.
    ///
    /// Extending by character instead is what undoes a double press: no real double
    /// click is perfectly still, and the first pointer sample inside the word the press
    /// selected would drag the active end back to it, leaving the word start to the
    /// pointer selected.
    fn drag_selection(
        &self,
        pointer: usize,
        dragging: &TextDragging,
        current: TextSelection,
    ) -> TextSelection {
        let (anchor_start, anchor_end) = dragging.anchor;
        match dragging.press {
            PressEventType::Single => {
                let mut selection = current;
                selection.move_to(pointer);
                return selection;
            }
            PressEventType::Quadruple => {
                return TextSelection::new_range((0, self.len_utf16_cu()));
            }
            _ => {}
        }

        // Still inside what the press selected, so that is the answer: a drag has to
        // leave the pressed unit before it extends anything. This is the whole fix for
        // a twitching double click, and it covers the pointer resting exactly on the
        // pressed word's edge, where the glyph under it is already the next one.
        if (anchor_start..=anchor_end).contains(&pointer) {
            return TextSelection::new_range((anchor_start, anchor_end));
        }

        let (edge_before, edge_after) = match dragging.press {
            PressEventType::Triple => {
                let span = self.line_span(pointer);
                (span.start, span.end)
            }
            _ => match self.find_word_boundaries(pointer) {
                // Whitespace belongs to no word, so there the pointer is its own edge.
                (from, to) if from == to => (pointer, pointer),
                bounds => bounds,
            },
        };

        if pointer < anchor_start {
            TextSelection::new_range((anchor_end, edge_before))
        } else {
            TextSelection::new_range((anchor_start, edge_after))
        }
    }

    /// The position `granularity` away from `pos` in the given direction.
    fn caret_target(&self, pos: usize, forward: bool, granularity: CaretGranularity) -> usize {
        match (granularity, forward) {
            (CaretGranularity::Grapheme, true) => self.grapheme_cluster_at(pos).end,
            (CaretGranularity::Grapheme, false) if pos > 0 => {
                self.grapheme_cluster_at(pos - 1).start
            }
            (CaretGranularity::Grapheme, false) => 0,
            (CaretGranularity::Word, true) => self.word_end_after(pos),
            (CaretGranularity::Word, false) => self.word_start_before(pos),
            (CaretGranularity::LineBoundary, true) => self.line_bounds(pos).end,
            (CaretGranularity::LineBoundary, false) => self.line_bounds(pos).start,
            (CaretGranularity::Document, true) => self.len_utf16_cu(),
            (CaretGranularity::Document, false) => 0,
        }
    }

    /// Apply a left/right caret motion, returning whether the caret ended up elsewhere.
    ///
    /// `extend` (Shift) grows the selection; without it the caret collapses to the end
    /// of the selection it is pointing at, which is the whole motion for a plain arrow
    /// and the starting point for a modified one.
    fn move_caret(&mut self, forward: bool, granularity: CaretGranularity, extend: bool) -> bool {
        let before = self.cursor_pos();
        let range = self.get_selection_range();

        if extend {
            self.selection_mut().set_as_range();
        } else {
            let collapsed = range.map(|(start, end)| if forward { end } else { start });
            self.selection_mut().set_as_cursor();
            if let Some(pos) = collapsed {
                // Collapse to the end the arrow points at, never to wherever the drag
                // happened to finish.
                self.selection_mut().move_to(pos);
                if granularity == CaretGranularity::Grapheme {
                    return self.cursor_pos() != before;
                }
            }
        }

        let to = self.caret_target(self.cursor_pos(), forward, granularity);
        self.selection_mut().move_to(to);
        self.cursor_pos() != before
    }

    /// Apply an up/down caret motion, returning whether the caret ended up elsewhere.
    ///
    /// `granularity` is [`None`] for a plain line step; [`Some`] carries the modified
    /// jump (macOS's Cmd+Up/Down to the document ends).
    fn move_caret_vertically(
        &mut self,
        down: bool,
        granularity: Option<CaretGranularity>,
        extend: bool,
    ) -> bool {
        let before = self.cursor_pos();
        let range = self.get_selection_range();

        if extend {
            self.selection_mut().set_as_range();
        } else {
            let collapsed = range.map(|(start, end)| if down { end } else { start });
            self.selection_mut().set_as_cursor();
            if let Some(pos) = collapsed {
                // Unlike a plain left/right, a line step still travels from the end it
                // collapsed to: the caret lands a line away, not at the selection edge.
                self.selection_mut().move_to(pos);
            }
        }

        match granularity {
            Some(granularity) => {
                let to = self.caret_target(self.cursor_pos(), down, granularity);
                self.selection_mut().move_to(to);
            }
            None if down => {
                self.cursor_down();
            }
            None => {
                self.cursor_up();
            }
        }
        self.cursor_pos() != before
    }

    /// Move the cursor to the end of the next word.
    fn cursor_word_right(&mut self) -> bool {
        let pos = self.cursor_pos();
        if pos >= self.len_utf16_cu() {
            return false;
        }
        let to = self.word_end_after(pos);
        self.selection_mut().move_to(to);
        true
    }

    /// Move the cursor to the start of the previous word.
    fn cursor_word_left(&mut self) -> bool {
        let pos = self.cursor_pos();
        if pos == 0 {
            return false;
        }
        let to = self.word_start_before(pos);
        self.selection_mut().move_to(to);
        true
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
    fn process_key(
        &mut self,
        key: &Key,
        modifiers: &Modifiers,
        allow_tabs: bool,
        allow_changes: bool,
        allow_read_clipboard: bool,
        allow_write_clipboard: bool,
    ) -> TextEvent {
        let mut event = TextEvent::empty();

        let selection = self.get_selection();

        // The rebindable editing chords (see [`EditBindings`]) run before the default
        // handling below since any editing action may sit on any chord. A matching
        // press is always consumed, even when its action is not allowed right now or
        // has nothing to do, so the chord never leaks into the text as typed input.
        if let Some(action) = self.edit_bindings().resolve(key, modifiers) {
            match action {
                EditAction::SelectAll => {
                    let len = self.len_utf16_cu();
                    self.set_selection((0, len));
                }
                EditAction::Copy if allow_write_clipboard => {
                    let selected = self.get_selected_text();
                    if let Some(selected) = selected {
                        Clipboard::set(selected).ok();
                    }
                }
                EditAction::Cut if allow_changes && allow_write_clipboard => {
                    let selection = self.get_selection_range();
                    if let Some((start, end)) = selection {
                        let text = self.get_selected_text().unwrap_or_default();
                        self.remove(start..end);
                        Clipboard::set(text).ok();
                        self.move_cursor_to(start);
                        event.insert(TextEvent::TEXT_CHANGED);
                    }
                }
                EditAction::Paste if allow_changes && allow_read_clipboard => {
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
                EditAction::Undo if allow_changes => {
                    if let Some(new_selection) = self.undo() {
                        *self.selection_mut() = new_selection;
                        event.insert(TextEvent::TEXT_CHANGED);
                        event.insert(TextEvent::SELECTION_CHANGED);
                    }
                }
                EditAction::Redo if allow_changes => {
                    if let Some(new_selection) = self.redo() {
                        *self.selection_mut() = new_selection;
                        event.insert(TextEvent::TEXT_CHANGED);
                        event.insert(TextEvent::SELECTION_CHANGED);
                    }
                }
                _ => {}
            }

            // Same tail as the fall-through path below.
            if event.contains(TextEvent::TEXT_CHANGED)
                && !event.contains(TextEvent::SELECTION_CHANGED)
            {
                self.clear_selection();
            }
            if self.get_selection() != selection {
                event.insert(TextEvent::SELECTION_CHANGED);
            }
            return event;
        }

        match key {
            Key::Named(NamedKey::Shift) => {}
            Key::Named(NamedKey::Control) => {}
            Key::Named(NamedKey::Alt) => {}
            Key::Named(NamedKey::Escape) => {
                self.clear_selection();
            }
            Key::Named(NamedKey::ArrowLeft) | Key::Named(NamedKey::ArrowRight) => {
                let forward = *key == Key::Named(NamedKey::ArrowRight);
                if self.move_caret(
                    forward,
                    CaretGranularity::horizontal(modifiers),
                    modifiers.contains(Modifiers::SHIFT),
                ) {
                    event.insert(TextEvent::CURSOR_CHANGED);
                }
            }
            Key::Named(NamedKey::ArrowUp) | Key::Named(NamedKey::ArrowDown) => {
                let down = *key == Key::Named(NamedKey::ArrowDown);
                if self.move_caret_vertically(
                    down,
                    CaretGranularity::vertical(modifiers),
                    modifiers.contains(Modifiers::SHIFT),
                ) {
                    event.insert(TextEvent::CURSOR_CHANGED);
                }
            }
            Key::Named(NamedKey::Home) | Key::Named(NamedKey::End) => {
                let forward = *key == Key::Named(NamedKey::End);
                // Primary+Home/End is the document, plain Home/End the line: the
                // convention wherever these keys exist at all, and on macOS what
                // Fn+Left/Right produces.
                let granularity = if modifiers.intersects(Modifiers::META | Modifiers::CONTROL) {
                    CaretGranularity::Document
                } else {
                    CaretGranularity::LineBoundary
                };
                if self.move_caret(forward, granularity, modifiers.contains(Modifiers::SHIFT)) {
                    event.insert(TextEvent::CURSOR_CHANGED);
                }
            }
            Key::Named(NamedKey::Backspace) | Key::Named(NamedKey::Delete) if allow_changes => {
                let forward = *key == Key::Named(NamedKey::Delete);

                if let Some((start, end)) = self.get_selection_range() {
                    self.remove(start..end);
                    self.move_cursor_to(start);
                    event.insert(TextEvent::TEXT_CHANGED);
                } else {
                    // The same granularity as the matching arrow, so Alt+Backspace
                    // removes exactly what Alt+Left would have skipped over.
                    let pos = self.cursor_pos();
                    let to =
                        self.caret_target(pos, forward, CaretGranularity::horizontal(modifiers));
                    if to != pos {
                        let range = if forward { pos..to } else { to..pos };
                        let start = range.start;
                        self.remove(range);
                        self.move_cursor_to(start);
                        event.insert(TextEvent::TEXT_CHANGED);
                    }
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

    /// The chords this editor's [`EditAction`]s (select all / copy / cut / paste /
    /// undo / redo) respond to in [`process_key`](Self::process_key). Override it to
    /// make the chords configurable per editor instance; the default is the platform
    /// convention (see [`EditBindings::default`]).
    fn edit_bindings(&self) -> &EditBindings {
        EditBindings::default_ref()
    }

    fn undo(&mut self) -> Option<TextSelection>;

    fn redo(&mut self) -> Option<TextSelection>;

    fn editor_history(&mut self) -> &mut EditorHistory;

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
