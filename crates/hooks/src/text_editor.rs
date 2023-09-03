use std::{borrow::Cow, fmt::Display, ops::Range};

use freya_elements::events::keyboard::{Code, Key, Modifiers};
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

/// Common trait for editable texts
pub trait TextEditor: Sized + Clone + Display {
    type LinesIterator<'a>: Iterator<Item = Line<'a>>
    where
        Self: 'a;

    fn set(&mut self, text: &str);

    /// Iterator over all the lines in the text.
    fn lines(&self) -> Self::LinesIterator<'_>;

    /// Insert a character in the text in the given position.
    fn insert_char(&mut self, char: char, char_idx: usize);

    /// Insert a string in the text in the given position.
    fn insert(&mut self, text: &str, char_idx: usize);

    /// Remove a part of the text.
    fn remove(&mut self, range: Range<usize>);

    /// Get line from the given char
    fn char_to_line(&self, char_idx: usize) -> usize;

    /// Get the first char from the given line
    fn line_to_char(&self, line_idx: usize) -> usize;

    /// Get a line from the text
    fn line(&self, line_idx: usize) -> Option<Line<'_>>;

    /// Total of lines
    fn len_lines(&self) -> usize;

    /// Get a readable cursor
    fn cursor(&self) -> &TextCursor;

    /// Get a mutable cursor
    fn cursor_mut(&mut self) -> &mut TextCursor;

    /// Get the cursor row
    fn cursor_row(&self) -> usize {
        self.cursor().row()
    }

    /// Get the cursor column
    fn cursor_col(&self) -> usize {
        self.cursor().col()
    }

    /// Move the cursor 1 line down
    fn cursor_down(&mut self) {
        let new_row = self.cursor_row() + 1;
        self.cursor_mut().set_row(new_row);
    }

    /// Move the cursor 1 line up
    fn cursor_up(&mut self) {
        let new_row = self.cursor_row() - 1;
        self.cursor_mut().set_row(new_row);
    }

    /// Move the cursor 1 char to the right
    fn cursor_right(&mut self) {
        let new_col = self.cursor_col() + 1;
        self.cursor_mut().set_col(new_col);
    }

    /// Move the cursor 1 char to the left
    fn cursor_left(&mut self) {
        let new_col = self.cursor_col() - 1;
        self.cursor_mut().set_col(new_col);
    }

    /// Get the cursor position
    fn cursor_pos(&self) -> usize {
        let line_begining = self.line_to_char(self.cursor_row());
        line_begining + self.cursor_col()
    }

    /// Set the cursor position
    fn set_cursor_pos(&mut self, pos: usize) {
        let row = self.char_to_line(pos);
        let row_idx = self.line_to_char(row);
        let col = pos - row_idx;
        self.cursor_mut().move_to(row, col)
    }

    // Return the highlighted text from a given editor Id
    fn highlights(&self, editor_id: usize) -> Option<(usize, usize)>;

    // Cancel highlight
    fn unhighlight(&mut self);

    // Highlight some text
    fn highlight_text(&mut self, from: usize, to: usize, editor_id: usize);

    fn move_highlight_to_cursor(&mut self);

    // Process a Keyboard event
    fn process_key(&mut self, key: &Key, code: &Code, modifiers: &Modifiers) -> TextEvent {
        let mut event = TextEvent::None;
        let mut unhighlight = true;

        match key {
            Key::Shift => {
                unhighlight = false;
            }
            Key::Escape => {
                unhighlight = true;
            }
            Key::ArrowDown => {
                if modifiers.contains(Modifiers::SHIFT) {
                    unhighlight = false;
                    self.move_highlight_to_cursor();
                }

                let total_lines = self.len_lines() - 1;
                // Go one line down
                if self.cursor_row() < total_lines {
                    let next_line = self.line(self.cursor_row() + 1).unwrap();

                    // Try to use the current cursor column, otherwise use the new line length
                    let cursor_col = if self.cursor_col() <= next_line.len_chars() {
                        self.cursor_col()
                    } else {
                        next_line.len_chars().max(1) - 1
                    };

                    self.cursor_mut().set_col(cursor_col);
                    self.cursor_down();

                    event = TextEvent::CursorChanged
                }

                if modifiers.contains(Modifiers::SHIFT) {
                    self.move_highlight_to_cursor();
                }
            }
            Key::ArrowLeft => {
                if modifiers.contains(Modifiers::SHIFT) {
                    unhighlight = false;
                    self.move_highlight_to_cursor();
                }

                // Go one character to the left
                if self.cursor_col() > 0 {
                    self.cursor_left();

                    event = TextEvent::CursorChanged
                } else if self.cursor_row() > 0 {
                    // Go one line up if there is no more characters on the left
                    let prev_line = self.line(self.cursor_row() - 1);
                    if let Some(prev_line) = prev_line {
                        // Use the prev line length as new cursor column, otherwise just set it to 0
                        let cursor_col = if prev_line.len_chars() > 0 {
                            prev_line.len_chars() - 1
                        } else {
                            0
                        };

                        self.cursor_up();
                        self.cursor_mut().set_col(cursor_col);

                        event = TextEvent::CursorChanged
                    }
                }

                if modifiers.contains(Modifiers::SHIFT) {
                    self.move_highlight_to_cursor();
                }
            }
            Key::ArrowRight => {
                if modifiers.contains(Modifiers::SHIFT) {
                    unhighlight = false;
                    self.move_highlight_to_cursor();
                }

                let total_lines = self.len_lines() - 1;
                let current_line = self.line(self.cursor_row()).unwrap();

                // Go one line down if there isn't more characters on the right
                if self.cursor_row() < total_lines
                    && self.cursor_col() == current_line.len_chars().max(1) - 1
                {
                    self.cursor_down();
                    self.cursor_mut().set_col(0);

                    event = TextEvent::CursorChanged
                } else if self.cursor_col() < current_line.len_chars() {
                    // Go one character to the right if possible
                    self.cursor_right();

                    event = TextEvent::CursorChanged
                }

                if modifiers.contains(Modifiers::SHIFT) {
                    self.move_highlight_to_cursor();
                }
            }
            Key::ArrowUp => {
                if modifiers.contains(Modifiers::SHIFT) {
                    unhighlight = false;
                    self.move_highlight_to_cursor();
                }

                // Go one line up if there is any
                if self.cursor_row() > 0 {
                    let prev_line = self.line(self.cursor_row() - 1).unwrap();

                    // Try to use the current cursor column, otherwise use the prev line length
                    let cursor_col = if self.cursor_col() <= prev_line.len_chars() {
                        self.cursor_col()
                    } else {
                        prev_line.len_chars().max(1) - 1
                    };

                    self.cursor_up();
                    self.cursor_mut().set_col(cursor_col);

                    event = TextEvent::CursorChanged
                }

                if modifiers.contains(Modifiers::SHIFT) {
                    self.move_highlight_to_cursor();
                }
            }
            Key::Backspace => {
                if self.cursor_col() > 0 {
                    // Remove the character to the left if there is any
                    let char_idx = self.line_to_char(self.cursor_row()) + self.cursor_col();
                    self.remove(char_idx - 1..char_idx);

                    self.cursor_left();

                    event = TextEvent::TextChanged
                } else if self.cursor_row() > 0 {
                    // Moves the whole current line to the end of the line above.
                    let prev_line_len = self.line(self.cursor_row() - 1).unwrap().len_chars();
                    let current_line = self.line(self.cursor_row()).clone();

                    if let Some(current_line) = current_line {
                        let current_line_len = current_line.len_chars();
                        let prev_char_idx =
                            self.line_to_char(self.cursor_row() - 1) + prev_line_len - 1;
                        let char_idx = self.line_to_char(self.cursor_row()) + current_line_len - 1;

                        let line = current_line.as_str().to_string();
                        self.insert(&line, prev_char_idx);
                        self.remove(char_idx..(char_idx + current_line_len) + 1);
                    }

                    self.cursor_up();
                    self.cursor_mut().set_col(prev_line_len - 1);

                    event = TextEvent::TextChanged
                }
            }
            Key::Enter => {
                // Breaks the line
                let char_idx = self.line_to_char(self.cursor_row()) + self.cursor_col();
                self.insert_char('\n', char_idx);
                self.cursor_down();
                self.cursor_mut().set_col(0);

                event = TextEvent::TextChanged
            }
            Key::Character(character) => {
                match code {
                    Code::Delete => {}
                    Code::Space => {
                        // Simply adds an space
                        let char_idx = self.line_to_char(self.cursor_row()) + self.cursor_col();
                        self.insert_char(' ', char_idx);
                        self.cursor_right();

                        event = TextEvent::TextChanged
                    }
                    _ => {
                        if let Ok(ch) = character.parse::<char>() {
                            if !ch.is_ascii_control() {
                                // Adds a new character
                                let char_idx =
                                    self.line_to_char(self.cursor_row()) + self.cursor_col();
                                self.insert(character, char_idx);
                                self.cursor_right();

                                event = TextEvent::TextChanged
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        if unhighlight {
            self.unhighlight();
        }

        event
    }
}
