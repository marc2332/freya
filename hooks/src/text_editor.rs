use std::{borrow::Cow, fmt::Display, ops::Range};

use freya_elements::events::keyboard::{Code, Key, Modifiers};
use unicode_segmentation::UnicodeSegmentation;
pub use ropey::Rope;

/// Holds the position of a cursor in a text
#[derive(Clone, Default)]
pub struct TextCursor {
    col: usize,
    row: usize,
}

impl TextCursor {
    /// Construct a new [TextCursor] given a row and a column
    pub fn new(row: usize, col: usize) -> Self {
        Self { col, row }
    }

    /// Move the cursor to a new row and column
    pub fn move_to(&mut self, row: usize, col: usize) {
        self.col = col;
        self.row = row;
    }

    /// Get the current column
    pub fn col(&self) -> usize {
        self.col
    }

    /// Get the current row
    pub fn row(&self) -> usize {
        self.row
    }

    /// Set a new column
    pub fn set_col(&mut self, col: usize) {
        self.col = col;
    }

    /// Set a new row
    pub fn set_row(&mut self, row: usize) {
        self.row = row;
    }

    pub fn as_tuple(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

/// A text line from a [TextEditor]
#[derive(Clone)]
pub struct Line<'a> {
    pub text: Cow<'a, str>,
}

impl Line<'_> {
    /// Get the length of the line
    pub fn len_chars(&self) -> usize {
        self.text.chars().filter(|c| c != &'\r').count()
    }

    /// Get the text of the line
    fn as_str(&self) -> &str {
        &self.text
    }
}

impl Display for Line<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}

/// Events for [TextEditor]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TextEvent {
    /// Cursor position has been moved
    CursorChanged,
    /// Text has changed
    TextChanged,
    /// Nothing happened
    None,
}

/// Possible cursor movement actions
pub enum CursorMovement {
    /// Specific charcter index
    Position(usize),
    /// Text row
    Row(usize),
    /// Text column
    Col(usize),
    /// Previous character
    PrevCharacter,
    /// Next character
    NextCharacter,
    /// Previous unicode glyph cluster
    PrevWord,
    /// Next unicode glyph cluster
    NextWord,
    /// Previous Line. If already on the first line, moves to the first character
    PrevLine,
    /// Next Line. If already on last line, moves to the last character
    NextLine,

    LineStart,
    LineEnd,

    /// Start of character range
    Start,
    /// End of character range
    End,
}

/// Common trait for editable texts
pub trait TextEditor: Sized + Clone + Display {
    type LinesIterator<'a>: Iterator<Item = Line<'a>>
    where
        Self: 'a;

    /// Set text contents
    fn set_text(&mut self, text: &str);

    /// Iterator over all the lines in the text.
    fn lines(&self) -> Self::LinesIterator<'_>;

    /// Insert a str in the text in the given position.
    fn insert(&mut self, text: &str, char_idx: usize);
    
    /// Insert a character in the text in the given position.
    fn insert_char(&mut self, char: char, char_idx: usize);

    /// Remove a part of the text.
    fn remove(&mut self, range: Range<usize>);

    /// Get a line from the text
    fn line(&self, line_idx: usize) -> Option<Line<'_>>;

    /// Line count
    fn len_lines(&self) -> usize;

    /// Total character length
    fn len_chars(&self) -> usize;

    /// Get a readable cursor
    fn cursor(&self) -> &TextCursor;

    /// Get a mutable cursor
    fn cursor_mut(&mut self) -> &mut TextCursor;

    /// Get the current character index of the cursor
    fn cursor_pos(&self) -> usize {
        let line_begining = self.line_to_char(self.cursor().row());
        line_begining + self.cursor().col()
    }

    /// Move the cursor
    fn move_cursor(&mut self, movement: CursorMovement) {
        let (row, col) = (self.cursor().row(), self.cursor().col());
        
        match movement {
            CursorMovement::Position(char_idx) => {
                let row = self.char_to_line(char_idx);
                let col = char_idx - self.line_to_char(row);

                self.cursor_mut().move_to(row, col);
            },
            CursorMovement::Col(col) => {
                self.cursor_mut().set_col(col);
            },
            CursorMovement::Row(row) => {
                self.cursor_mut().set_row(row);
            },
            CursorMovement::PrevCharacter => {
                if col > 0 {
                    self.cursor_mut().set_col(col - 1);
                } else if row > 0 {
                    let prev_line_len = self.line(row - 1).unwrap().len_chars();
                    let cursor = self.cursor_mut();

                    cursor.set_row(row - 1);
                    cursor.set_col(prev_line_len - 1);
                }
            },
            CursorMovement::NextCharacter => {
                let current_line = self.line(row).unwrap();
                
                if col < current_line.len_chars() {
                    self.cursor_mut().set_col(col + 1);
                } else if row < self.len_lines() {
                    let cursor = self.cursor_mut();

                    cursor.set_row(row + 1);
                    cursor.set_col(0);
                }
            },
            CursorMovement::PrevWord => {
                todo!()
            },
            CursorMovement::NextWord => {
                todo!()
            },
            CursorMovement::PrevLine => {
                if row > 0 {
                    self.cursor_mut().set_row(row - 1);
                } else {
                    self.move_cursor(CursorMovement::Start);
                }
            },
            CursorMovement::NextLine => {
                if row < self.len_lines() - 1 {
                    self.cursor_mut().set_row(row + 1);
                } else {
                    self.move_cursor(CursorMovement::End);
                }
            },
            CursorMovement::LineStart => {
                self.cursor_mut().set_col(0);
            },
            CursorMovement::LineEnd => {
                let current_line_len = self.line(row).unwrap().len_chars();
                self.cursor_mut().set_col(current_line_len);
            },
            CursorMovement::Start => {
                self.cursor_mut().move_to(0, 0);
            },
            CursorMovement::End => {
                self.move_cursor(CursorMovement::Position(self.len_chars()));
            },
        }
    }

    /// Return the highlight range from a given editor Id
    fn highlights(&self, editor_id: usize) -> Option<(usize, usize)>;
   
    /// Highlight some text in a range of character indexes
    fn highlight_text(&mut self, from: usize, to: usize, editor_id: usize);

    /// Cancel the current highlight
    fn unhighlight(&mut self);

    /// Move the start or end of the highlight range to the cursor
    fn highlight_to_cursor(&mut self);

    /// Get the line that a character position is on.
    fn char_to_line(&self, char_idx: usize) -> usize;

    /// Get the first character position of a line.
    fn line_to_char(&self, line_idx: usize) -> usize;

    // Process a Keyboard event
    fn process_key(&mut self, key: &Key, code: &Code, modifiers: &Modifiers) -> TextEvent {
        let mut event = TextEvent::None;
        let ctrl_or_meta = modifiers.contains(Modifiers::CONTROL) || modifiers.contains(Modifiers::META);

        match key {
            Key::ArrowLeft | Key::ArrowRight | Key::ArrowUp | Key::ArrowDown => {
                // If control is held, move based on words rather than characters
                if ctrl_or_meta {
                    match key {
                        Key::ArrowLeft => self.move_cursor(CursorMovement::PrevWord),
                        Key::ArrowRight => self.move_cursor(CursorMovement::NextWord),
                        _ => {},
                    }
                } else {
                    match key {
                        Key::ArrowLeft => self.move_cursor(CursorMovement::PrevCharacter),
                        Key::ArrowRight => self.move_cursor(CursorMovement::NextCharacter),
                        Key::ArrowUp => self.move_cursor(CursorMovement::PrevLine),
                        Key::ArrowDown => self.move_cursor(CursorMovement::NextLine),
                        _ => {},
                    }
                }

                // If holding shift, move highlight to cursor as well
                if modifiers.contains(Modifiers::SHIFT) {
                    self.highlight_to_cursor();
                }

                event = TextEvent::CursorChanged;
            },
            Key::Backspace => {
                let cursor_position = self.cursor_pos();
                if cursor_position > 0 {
                    // Delete the character directly before the cursor
                    if ctrl_or_meta {
                        todo!();
                    }
                    self.move_cursor(CursorMovement::PrevCharacter);
                    self.remove(cursor_position - 1..cursor_position);
                    event = TextEvent::TextChanged;
                }
            },
            Key::Delete => {
                let cursor_position = self.cursor_pos();
                if cursor_position < self.len_chars() {
                    // Delete the character directly after the cursor
                    if ctrl_or_meta {
                        todo!();
                    }
                    self.remove(cursor_position..cursor_position + 1);
                    event = TextEvent::TextChanged;
                }
            },
            Key::Enter => {
                // Insert line break at current character.
                self.insert_char('\n', self.cursor_pos());

                // Move cursor to next line, placing it at the start character.
                self.move_cursor(CursorMovement::NextLine);
                self.move_cursor(CursorMovement::Col(0));

                event = TextEvent::TextChanged;
            },
            Key::PageUp => {
                todo!();
            },
            Key::PageDown => {
                todo!();
            },
            Key::Home => {
                if ctrl_or_meta {
                    self.move_cursor(CursorMovement::Start);
                } else {
                    self.move_cursor(CursorMovement::LineStart);
                }
            },
            Key::End => {
                if ctrl_or_meta {
                    self.move_cursor(CursorMovement::End);
                } else {
                    self.move_cursor(CursorMovement::LineEnd);
                }
            },
            Key::Character(character) => {
                if modifiers.contains(Modifiers::CONTROL) || modifiers.contains(Modifiers::META) {
                    match code {
                        Code::KeyA => {
                            self.move_cursor(CursorMovement::End);
                            self.highlight_to_cursor();
                        },
                        _ => {},
                    }
                } else {
                    // Adds the new character and moves cursor to new character position
                    self.insert(character, self.cursor_pos());
                    self.move_cursor(CursorMovement::NextCharacter);

                    event = TextEvent::TextChanged;
                }

            },
            _ => {},
        }

        if !(key == &Key::Shift && modifiers.contains(Modifiers::SHIFT)) {
            self.unhighlight();
        }

        event
    }
}
