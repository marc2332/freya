use std::{borrow::Cow, cmp::Ordering, fmt::Display, ops::Range};

use dioxus_sdk::clipboard::UseClipboard;
use freya_elements::events::keyboard::{Code, Key, Modifiers};

/// Holds the position of a cursor in a text
#[derive(Clone, Default, PartialEq, Debug)]
pub struct TextCursor(usize);

impl TextCursor {
    /// Construct a new [TextCursor]
    pub fn new(pos: usize) -> Self {
        Self(pos)
    }

    /// Get the position
    pub fn pos(&self) -> usize {
        self.0
    }

    /// Set the position
    pub fn set(&mut self, pos: usize) {
        self.0 = pos;
    }

    /// Write the position
    pub fn write(&mut self) -> &mut usize {
        &mut self.0
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
    fn insert_char(&mut self, char: char, char_idx: usize);

    /// Insert a string in the text in the given position.
    fn insert(&mut self, text: &str, char_idx: usize);

    /// Remove a part of the text.
    fn remove(&mut self, range: Range<usize>);

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

    /// Get a readable cursor
    fn cursor(&self) -> &TextCursor;

    /// Get a mutable cursor
    fn cursor_mut(&mut self) -> &mut TextCursor;

    /// Get the cursor row
    fn cursor_row(&self) -> usize {
        let pos = self.cursor_pos();
        self.char_to_line(pos)
    }

    /// Get the cursor column
    fn cursor_col(&self) -> usize {
        let pos = self.cursor_pos();
        let line = self.char_to_line(pos);
        let line_char = self.line_to_char(line);
        pos - line_char
    }

    /// Get the cursor row and col
    fn cursor_row_and_col(&self) -> (usize, usize) {
        (self.cursor_row(), self.cursor_col())
    }

    /// Get the visible cursor position
    fn visible_cursor_col(&self) -> usize {
        self.char_to_utf16_cu(self.cursor_col())
    }

    /// Move the cursor 1 line down
    fn cursor_down(&mut self) -> bool {
        let old_row = self.cursor_row();
        let old_col = self.cursor_col();

        match old_row.cmp(&(self.len_lines() - 1)) {
            Ordering::Less => {
                // One line below
                let new_row = old_row + 1;
                let new_row_char = self.line_to_char(new_row);
                let new_row_len = self.line(new_row).unwrap().len_chars();
                let new_col = old_col.min(new_row_len - 1);
                self.cursor_mut().set(new_row_char + new_col);

                true
            }
            Ordering::Equal => {
                let end = self.len_chars();
                // Reached max
                self.cursor_mut().set(end);

                true
            }
            Ordering::Greater => {
                // Can't go further

                false
            }
        }
    }

    /// Move the cursor 1 line up
    fn cursor_up(&mut self) -> bool {
        let pos = self.cursor_pos();
        let old_row = self.cursor_row();
        let old_col = self.cursor_col();

        if pos > 0 {
            // Reached max
            if old_row == 0 {
                self.cursor_mut().set(0);
            } else {
                let new_row = old_row - 1;
                let new_row_char = self.line_to_char(new_row);
                let new_row_len = self.line(new_row).unwrap().len_chars();
                let new_col = old_col.min(new_row_len - 1);
                self.cursor_mut().set(new_row_char + new_col);
            }

            true
        } else {
            false
        }
    }

    /// Move the cursor 1 char to the right
    fn cursor_right(&mut self) -> bool {
        if self.cursor_pos() < self.len_chars() {
            *self.cursor_mut().write() += 1;

            true
        } else {
            false
        }
    }

    /// Move the cursor 1 char to the left
    fn cursor_left(&mut self) -> bool {
        if self.cursor_pos() > 0 {
            *self.cursor_mut().write() -= 1;

            true
        } else {
            false
        }
    }

    /// Get the cursor position
    fn cursor_pos(&self) -> usize {
        self.cursor().pos()
    }

    /// Get the cursor position
    fn visible_cursor_pos(&self) -> usize {
        self.char_to_utf16_cu(self.cursor_pos())
    }

    /// Set the cursor position
    fn set_cursor_pos(&mut self, pos: usize) {
        self.cursor_mut().set(pos);
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

    // Process a Keyboard event
    fn process_key(&mut self, key: &Key, code: &Code, modifiers: &Modifiers) -> TextEvent {
        let mut event = if self.has_any_selection() {
            TextEvent::SELECTION_CHANGED
        } else {
            TextEvent::empty()
        };

        match key {
            Key::Shift => {
                event.remove(TextEvent::SELECTION_CHANGED);
            }
            Key::Control => {
                event.remove(TextEvent::SELECTION_CHANGED);
            }
            Key::Alt => {
                event.remove(TextEvent::SELECTION_CHANGED);
            }
            Key::Escape => {
                event.insert(TextEvent::SELECTION_CHANGED);
            }
            Key::ArrowDown => {
                if modifiers.contains(Modifiers::SHIFT) {
                    event.remove(TextEvent::SELECTION_CHANGED);
                    self.expand_selection_to_cursor();
                }

                if self.cursor_down() {
                    event.insert(TextEvent::CURSOR_CHANGED);
                }

                if modifiers.contains(Modifiers::SHIFT) {
                    self.expand_selection_to_cursor();
                }
            }
            Key::ArrowLeft => {
                if modifiers.contains(Modifiers::SHIFT) {
                    event.remove(TextEvent::SELECTION_CHANGED);
                    self.expand_selection_to_cursor();
                }

                if self.cursor_left() {
                    event.insert(TextEvent::CURSOR_CHANGED);
                }

                if modifiers.contains(Modifiers::SHIFT) {
                    self.expand_selection_to_cursor();
                }
            }
            Key::ArrowRight => {
                if modifiers.contains(Modifiers::SHIFT) {
                    event.remove(TextEvent::SELECTION_CHANGED);
                    self.expand_selection_to_cursor();
                }

                if self.cursor_right() {
                    event.insert(TextEvent::CURSOR_CHANGED);
                }

                if modifiers.contains(Modifiers::SHIFT) {
                    self.expand_selection_to_cursor();
                }
            }
            Key::ArrowUp => {
                if modifiers.contains(Modifiers::SHIFT) {
                    event.remove(TextEvent::SELECTION_CHANGED);
                    self.expand_selection_to_cursor();
                }

                if self.cursor_up() {
                    event.insert(TextEvent::CURSOR_CHANGED);
                }

                if modifiers.contains(Modifiers::SHIFT) {
                    self.expand_selection_to_cursor();
                }
            }
            Key::Backspace => {
                let char_idx = self.line_to_char(self.cursor_row()) + self.cursor_col();
                let selection = self.get_selection_range();

                if let Some((start, end)) = selection {
                    self.remove(start..end);
                    self.set_cursor_pos(start);
                    event.insert(TextEvent::TEXT_CHANGED);
                } else if char_idx > 0 {
                    // Remove the character to the left if there is any
                    self.remove(char_idx - 1..char_idx);
                    self.set_cursor_pos(char_idx - 1);
                    event.insert(TextEvent::TEXT_CHANGED);
                }
            }
            Key::Delete => {
                let char_idx = self.line_to_char(self.cursor_row()) + self.cursor_col();
                let selection = self.get_selection_range();

                if let Some((start, end)) = selection {
                    self.remove(start..end);
                    self.set_cursor_pos(start);
                    event.insert(TextEvent::TEXT_CHANGED);
                } else if char_idx < self.len_chars() {
                    // Remove the character to the right if there is any
                    self.remove(char_idx..char_idx + 1);
                    event.insert(TextEvent::TEXT_CHANGED);
                }
            }
            Key::Enter => {
                // Breaks the line
                let cursor_pos = self.cursor_pos();
                self.insert_char('\n', cursor_pos);
                self.cursor_right();

                event.insert(TextEvent::TEXT_CHANGED);
            }
            Key::Tab => {
                // Inserts a tab
                let text = " ".repeat(self.get_identation());
                let cursor_pos = self.cursor_pos();
                self.insert(&text, cursor_pos);
                self.set_cursor_pos(cursor_pos + text.chars().count());

                event.insert(TextEvent::TEXT_CHANGED);
            }
            Key::Character(character) => {
                let meta_or_ctrl = if cfg!(target_os = "macos") {
                    modifiers.meta()
                } else {
                    modifiers.ctrl()
                };

                match code {
                    Code::Delete => {}
                    Code::Space => {
                        // Simply adds an space
                        let cursor_pos = self.cursor_pos();
                        self.insert_char(' ', cursor_pos);
                        self.cursor_right();

                        event.insert(TextEvent::TEXT_CHANGED);
                    }

                    // Select all text
                    Code::KeyA if meta_or_ctrl => {
                        let len = self.len_chars();
                        self.set_selection((0, len));
                        event.remove(TextEvent::SELECTION_CHANGED);
                    }

                    // Copy selected text
                    Code::KeyC if meta_or_ctrl => {
                        let selected = self.get_selected_text();
                        if let Some(selected) = selected {
                            self.get_clipboard().set(selected).ok();
                        }
                        event.remove(TextEvent::SELECTION_CHANGED);
                    }

                    // Cut selected text
                    Code::KeyX if meta_or_ctrl => {
                        let selection = self.get_selection_range();
                        if let Some((start, end)) = selection {
                            let text = self.get_selected_text().unwrap();
                            self.remove(start..end);
                            self.get_clipboard().set(text).ok();
                            self.set_cursor_pos(start);
                            event.insert(TextEvent::TEXT_CHANGED);
                        }
                    }

                    // Paste copied text
                    Code::KeyV if meta_or_ctrl => {
                        let copied_text = self.get_clipboard().get();
                        if let Ok(copied_text) = copied_text {
                            let char_idx = self.line_to_char(self.cursor_row()) + self.cursor_col();
                            self.insert(&copied_text, char_idx);
                            let last_idx = copied_text.len() + char_idx;
                            self.set_cursor_pos(last_idx);
                            event.insert(TextEvent::TEXT_CHANGED);
                        }
                    }

                    // Undo last change
                    Code::KeyZ if meta_or_ctrl => {
                        let undo_result = self.undo();

                        if let Some(idx) = undo_result {
                            self.set_cursor_pos(idx);
                            event.insert(TextEvent::TEXT_CHANGED);
                        }
                    }

                    // Redo last change
                    Code::KeyY if meta_or_ctrl => {
                        let undo_result = self.redo();

                        if let Some(idx) = undo_result {
                            self.set_cursor_pos(idx);
                            event.insert(TextEvent::TEXT_CHANGED);
                        }
                    }

                    _ => {
                        if let Ok(ch) = character.parse::<char>() {
                            // Inserts a character
                            let cursor_pos = self.cursor_pos();
                            self.insert_char(ch, cursor_pos);
                            self.cursor_right();

                            event.insert(TextEvent::TEXT_CHANGED);
                        } else {
                            // Inserts a text
                            let cursor_pos = self.cursor_pos();
                            self.insert(character, cursor_pos);
                            self.set_cursor_pos(cursor_pos + character.chars().count());

                            event.insert(TextEvent::TEXT_CHANGED);
                        }
                    }
                }
            }
            _ => {}
        }

        if event.contains(TextEvent::SELECTION_CHANGED) {
            self.clear_selection();
        }

        event
    }

    fn get_selected_text(&self) -> Option<String>;

    fn undo(&mut self) -> Option<usize>;

    fn redo(&mut self) -> Option<usize>;

    fn get_selection_range(&self) -> Option<(usize, usize)>;

    fn get_identation(&self) -> usize;
}
