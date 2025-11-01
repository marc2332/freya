use std::{borrow::Cow, fmt::Display, ops::Range};

use freya_clipboard::hooks::UseClipboard;

use crate::editor_history::EditorHistory;

/// Holds the position of a cursor in a text
#[derive(Clone, Default, PartialEq, Debug)]
pub struct TextCursor {
    pos: usize,
    x_pos: f32,
}

impl TextCursor {
    /// Construct a new [TextCursor]
    pub fn new(pos: usize) -> Self {
        Self { pos, x_pos: 0. }
    }

    /// Get the position
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Set the position
    pub fn set(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Write the position
    pub fn write(&mut self) -> &mut usize {
        &mut self.pos
    }

    pub fn x_pos(&self) -> f32 {
        self.x_pos
    }

    pub fn set_x_pos(&mut self, x_pos: f32) {
        self.x_pos = x_pos;
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

    /// Get a readable cursor
    fn cursor(&self) -> &TextCursor;

    /// Get a mutable cursor
    fn cursor_mut(&mut self) -> &mut TextCursor;

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

    /// Get the cursor position
    fn cursor_pos(&self) -> usize {
        self.cursor().pos()
    }

    /// Set the cursor position
    fn set_cursor_pos(&mut self, pos: usize) {
        self.cursor_mut().set(pos);
        self.cursor_mut().set_x_pos(0.);
    }

    // Check if has any selection at all
    fn has_any_selection(&self) -> bool;

    // Return the selected text
    fn get_selection(&self) -> Option<(usize, usize)>;

    // Return the visible selected text from a given editor Id
    fn get_visible_selection(&self, editor_id: usize) -> Option<(usize, usize)>;

    // Remove the selection
    fn clear_selection(&mut self);

    // Select some text
    fn set_selection(&mut self, selected: (usize, usize));

    // Measure a new text selection
    fn measure_new_selection(&self, from: usize, to: usize, editor_id: usize) -> (usize, usize);

    // Measure a new cursor
    fn measure_new_cursor(&self, to: usize, editor_id: usize) -> TextCursor;

    // Update the selection with a new cursor
    fn expand_selection_to_cursor(&mut self);

    fn get_clipboard(&mut self) -> &mut UseClipboard;

    fn get_selected_text(&self) -> Option<String>;

    fn undo(&mut self) -> Option<usize>;

    fn redo(&mut self) -> Option<usize>;

    fn editor_history(&mut self) -> &mut EditorHistory;

    fn get_selection_range(&self) -> Option<(usize, usize)>;

    fn get_identation(&self) -> u8;
}
