use std::ops::Mul;

use freya_core::{elements::paragraph::ParagraphHolderInner, prelude::*};
use freya_engine::prelude::{Paragraph, RectHeightStyle, RectWidthStyle, TextDirection};
use keyboard_types::Code;
use torin::prelude::CursorPoint;

use crate::{
    EditableConfig,
    rope_editor::RopeEditor,
    text_editor::{TextEditor, TextEvent},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum CursorMovement {
    /// Move one character left/right.
    Glyph,

    /// Move one word boundary left/right.
    Word,

    /// Move to start/end of line.
    Line,

    /// Move to start/end of buffer.
    Buffer,

    /// Move to start/end of selection.
    Selection,
}

pub enum EditableEvent<'a> {
    Release,
    Move {
        location: CursorPoint,
        editor_id: usize,
        holder: &'a ParagraphHolder,
    },
    Down {
        location: CursorPoint,
        editor_id: usize,
        holder: &'a ParagraphHolder,
    },
    KeyDown {
        key: &'a Key,
        code: Code,
        modifiers: Modifiers,
        holder: &'a ParagraphHolder,
    },
    KeyUp {
        code: Code,
    },
}

/// Move the cursor 1 line up
fn cursor_up(editor: &mut impl TextEditor, paragraph: &Paragraph) -> bool {
    let cursor_pos = editor.cursor_pos();
    let Some(cursor_x_pos) = paragraph
        .get_rects_for_range(
            cursor_pos..cursor_pos + 1,
            RectHeightStyle::Tight,
            RectWidthStyle::Tight,
        )
        .first()
        .map(|textbox| match textbox.direct {
            TextDirection::LTR => textbox.rect.left,
            TextDirection::RTL => textbox.rect.right,
        })
        .or_else(|| {
            paragraph
                .get_rects_for_range(
                    cursor_pos - 1..cursor_pos,
                    RectHeightStyle::Tight,
                    RectWidthStyle::Tight,
                )
                .first()
                .map(|textbox| match textbox.direct {
                    TextDirection::LTR => textbox.rect.right,
                    TextDirection::RTL => textbox.rect.left,
                })
        })
    else {
        #[cfg(debug_assertions)]
        panic!("Cursor is somehow not on a glyph.");
        return false;
    };

    if editor.cursor().x_pos() < cursor_x_pos {
        editor.cursor_mut().set_x_pos(cursor_x_pos);
    }

    let current_line_number = if cursor_pos == editor.len_utf16_cu() {
        Some(paragraph.line_number() - 1)
    } else {
        paragraph.get_line_number_at(cursor_pos)
    };

    if let Some(current_line_number) = current_line_number
        && current_line_number > 0
    {
        let prev_line_metrics = paragraph
            .get_line_metrics_at(current_line_number - 1)
            .unwrap();
        let nearest_glyph_on_prev_line = paragraph.get_glyph_position_at_coordinate((
            editor.cursor().x_pos(),
            prev_line_metrics.baseline as f32,
        ));

        editor
            .cursor_mut()
            .set(nearest_glyph_on_prev_line.position as usize);
    } else {
        editor.cursor_mut().set(0);
    }

    true
}

/// Move the cursor 1 line down
fn cursor_down(editor: &mut impl TextEditor, paragraph: &Paragraph) -> bool {
    let cursor_pos = editor.cursor_pos();
    let Some(cursor_x_pos) = paragraph
        .get_rects_for_range(
            cursor_pos..cursor_pos + 1,
            RectHeightStyle::Tight,
            RectWidthStyle::Tight,
        )
        .first()
        .map(|textbox| match textbox.direct {
            TextDirection::LTR => textbox.rect.left,
            TextDirection::RTL => textbox.rect.right,
        })
        .or_else(|| {
            paragraph
                .get_rects_for_range(
                    cursor_pos - 1..cursor_pos,
                    RectHeightStyle::Tight,
                    RectWidthStyle::Tight,
                )
                .first()
                .map(|textbox| match textbox.direct {
                    TextDirection::LTR => textbox.rect.right,
                    TextDirection::RTL => textbox.rect.left,
                })
        })
    else {
        #[cfg(debug_assertions)]
        panic!("Cursor is somehow not on a glyph.");
        return false;
    };

    if editor.cursor().x_pos() < cursor_x_pos {
        editor.cursor_mut().set_x_pos(cursor_x_pos);
    }

    if let Some(current_line_number) = paragraph.get_line_number_at(cursor_pos)
        && let Some(next_line_metrics) = paragraph.get_line_metrics_at(current_line_number + 1)
    {
        let nearest_glyph_on_next_line = paragraph.get_glyph_position_at_coordinate((
            editor.cursor().x_pos(),
            next_line_metrics.baseline as f32,
        ));

        editor
            .cursor_mut()
            .set(nearest_glyph_on_next_line.position as usize);
    } else {
        let end_of_buffer = editor.len_utf16_cu();
        editor.cursor_mut().set(end_of_buffer);
    }

    true
}

/// Move the cursor to the left
fn cursor_left(
    editor: &mut impl TextEditor,
    paragraph: &Paragraph,
    movement: CursorMovement,
) -> bool {
    let pos = editor.cursor_pos();

    let target_pos = match movement {
        CursorMovement::Glyph => {
            if pos > 0 {
                pos - 1
            } else {
                return false;
            }
        }
        CursorMovement::Line => editor.line_to_char(editor.cursor_row()),
        CursorMovement::Word => {
            paragraph
                .get_word_boundary(editor.cursor_pos().saturating_sub(1) as u32)
                .start
        }
        CursorMovement::Buffer => 0,
        CursorMovement::Selection => {
            let Some(selection) = editor.get_selection() else {
                return false;
            };

            let selection_min = selection.0.min(selection.1);

            if editor.cursor_pos() == selection_min {
                return false;
            }

            selection_min
        }
        _ => unimplemented!(),
    };

    if pos != target_pos {
        editor.set_cursor_pos(target_pos);

        true
    } else {
        false
    }
}

/// Move the cursor to th right
fn cursor_right(
    editor: &mut impl TextEditor,
    paragraph: &Paragraph,
    movement: CursorMovement,
) -> bool {
    let pos = editor.cursor_pos();
    let target_pos = match movement {
        CursorMovement::Glyph => {
            if pos < editor.len_utf16_cu() {
                pos + 1
            } else {
                return false;
            }
        }
        CursorMovement::Line => {
            let cursor_row = editor.cursor_row();
            let current_line = editor.line(cursor_row).unwrap();
            let current_line_start = editor.line_to_char(cursor_row);

            let mut target = current_line_start + current_line.utf16_len();

            // Freya currently has no concept of cursor affinity and counts newlines as
            // characters. because of this, we need to subtract off the newline character from
            // our jump or else the cursor will end up on the next line. The final line has no
            // explicit trailing linebreak though, meaning we shouldn't subtract if we're
            // jumping to the end of the last line in the buffer.
            if editor.line(cursor_row + 1).is_some() {
                target = target.saturating_sub(1);
            }

            target
        }
        CursorMovement::Word => paragraph.get_word_boundary(editor.cursor_pos() as u32).end,
        CursorMovement::Buffer => editor.len_utf16_cu(),
        CursorMovement::Selection => {
            let Some(selection) = editor.get_selection() else {
                return false;
            };

            let selection_max = selection.0.max(selection.1);

            if editor.cursor_pos() == selection_max {
                return false;
            }

            selection_max
        }
    };

    if pos != target_pos {
        editor.set_cursor_pos(target_pos);

        true
    } else {
        false
    }
}

impl EditableEvent<'_> {
    pub fn process<'a, 'b>(
        self,
        mut editor: impl MutView<'b, RopeEditor>,
        mut dragging: impl MutView<'b, TextDragging>,
        config: &'_ EditableConfig,
    ) {
        match self {
            EditableEvent::Down {
                location,
                editor_id,
                holder,
            } => {
                let holder = holder.0.borrow();
                let ParagraphHolderInner {
                    paragraph,
                    scale_factor,
                } = holder.as_ref().unwrap();

                dragging
                    .write()
                    .set_cursor_coords(location.mul(*scale_factor));

                let mut text_editor = editor.write();
                text_editor.clear_selection();

                let char_position = paragraph.get_glyph_position_at_coordinate(
                    location.mul(*scale_factor).to_i32().to_tuple(),
                );
                let new_cursor =
                    text_editor.measure_new_cursor(char_position.position as usize, editor_id);

                // Only update and clear the selection if the cursor has changed
                if *text_editor.cursor() != new_cursor {
                    *text_editor.cursor_mut() = new_cursor;
                    if let TextDragging::FromCursorToPoint { cursor: from, .. } = &*dragging.peek()
                    {
                        let to = text_editor.cursor_pos();
                        text_editor.set_selection((*from, to));
                    } else {
                        text_editor.clear_selection();
                    }
                }
            }
            EditableEvent::Move {
                location,
                editor_id,
                holder,
            } => {
                if let Some(origin) = dragging.peek().get_cursor_coords() {
                    let paragraph = holder.0.borrow();
                    let ParagraphHolderInner {
                        paragraph,
                        scale_factor,
                    } = paragraph.as_ref().unwrap();

                    let origin_position = origin;
                    let dist_position = location.mul(*scale_factor);

                    // Calculate the start of the highlighting
                    let origin_char = paragraph
                        .get_glyph_position_at_coordinate(origin_position.to_i32().to_tuple());
                    // Calculate the end of the highlighting
                    let dist_char = paragraph
                        .get_glyph_position_at_coordinate(dist_position.to_i32().to_tuple());
                    let from = origin_char.position as usize;
                    let to = dist_char.position as usize;

                    let current_cursor = editor.peek().cursor().clone();
                    let current_selection = editor.peek().get_selection();

                    let maybe_new_cursor = editor.peek().measure_new_cursor(to, editor_id);
                    let maybe_new_selection =
                        editor.peek().measure_new_selection(from, to, editor_id);

                    // Update the text selection if it has changed
                    if let Some(current_selection) = current_selection {
                        if current_selection != maybe_new_selection {
                            let mut text_editor = editor.write();
                            text_editor.set_selection(maybe_new_selection);
                        }
                    } else {
                        let mut text_editor = editor.write();
                        text_editor.set_selection(maybe_new_selection);
                    }

                    // Update the cursor if it has changed
                    if current_cursor != maybe_new_cursor {
                        let mut text_editor = editor.write();
                        *text_editor.cursor_mut() = maybe_new_cursor;
                    }
                }
            }
            EditableEvent::Release => {
                let dragging = &mut *dragging.write();
                match dragging {
                    TextDragging::FromCursorToPoint { shift, clicked, .. } if *shift => {
                        *clicked = false;
                    }
                    _ => {
                        *dragging = TextDragging::None;
                    }
                }
            }
            EditableEvent::KeyDown {
                code,
                key,
                modifiers,
                holder,
            } => {
                let paragraph = holder.0.borrow();
                let paragraph = &paragraph.as_ref().unwrap().paragraph;

                match code {
                    // Handle dragging
                    Code::ShiftLeft => {
                        let dragging = &mut *dragging.write();
                        match dragging {
                            TextDragging::FromCursorToPoint {
                                shift: shift_pressed,
                                ..
                            } => {
                                *shift_pressed = true;
                            }
                            TextDragging::None => {
                                *dragging = TextDragging::FromCursorToPoint {
                                    shift: true,
                                    clicked: false,
                                    cursor: editor.peek().cursor_pos(),
                                    dist: None,
                                }
                            }
                            _ => {}
                        }
                    }
                    // Handle editing
                    _ => {
                        editor.write_if(|mut editor| {
                            let mut event = TextEvent::empty();

                            let shift = modifiers.contains(Modifiers::SHIFT);
                            let meta_or_ctrl = if cfg!(target_os = "macos") {
                                modifiers.meta()
                            } else {
                                modifiers.ctrl()
                            };

                            match key {
                                Key::Shift => {}
                                Key::Control => {}
                                Key::Alt => {}
                                Key::Escape => {
                                    editor.clear_selection();
                                    event.insert(TextEvent::SELECTION_CHANGED);
                                }
                                Key::ArrowLeft => {
                                    // TODO: Modifier (Ctrl/Meta) should move one grapheme left
                                    let initial_selection = editor.get_selection();

                                    if shift {
                                        editor.expand_selection_to_cursor();

                                        if cursor_left(
                                            &mut *editor,
                                            paragraph,
                                            if meta_or_ctrl {
                                                CursorMovement::Word
                                            } else {
                                                CursorMovement::Glyph
                                            },
                                        ) {
                                            event.insert(TextEvent::CURSOR_CHANGED);
                                            editor.expand_selection_to_cursor();
                                        }
                                    } else {
                                        let movement = if meta_or_ctrl {
                                            CursorMovement::Word
                                        } else if editor.has_any_selection() {
                                            CursorMovement::Selection
                                        } else {
                                            CursorMovement::Glyph
                                        };

                                        // If we have an active selection, move to the start of that selection and clear it.
                                        if cursor_left(&mut *editor, paragraph, movement) {
                                            event.insert(TextEvent::CURSOR_CHANGED);
                                        }

                                        editor.clear_selection();
                                    }

                                    if initial_selection != editor.get_selection() {
                                        event.insert(TextEvent::SELECTION_CHANGED);
                                    }
                                }
                                Key::ArrowRight => {
                                    // TODO: Modifier (Ctrl/Meta) should move one grapheme right
                                    let initial_selection = editor.get_selection();

                                    if shift {
                                        editor.expand_selection_to_cursor();

                                        if cursor_right(
                                            &mut *editor,
                                            paragraph,
                                            if meta_or_ctrl {
                                                CursorMovement::Word
                                            } else {
                                                CursorMovement::Glyph
                                            },
                                        ) {
                                            event.insert(TextEvent::CURSOR_CHANGED);
                                            editor.expand_selection_to_cursor();
                                        }
                                    } else {
                                        let movement = if meta_or_ctrl {
                                            CursorMovement::Word
                                        } else if editor.has_any_selection() {
                                            CursorMovement::Selection
                                        } else {
                                            CursorMovement::Glyph
                                        };

                                        // If we have an active selection, move to the end of that selection and clear it.
                                        if cursor_right(&mut *editor, paragraph, movement) {
                                            event.insert(TextEvent::CURSOR_CHANGED);
                                        }

                                        editor.clear_selection();
                                    }

                                    if initial_selection != editor.get_selection() {
                                        event.insert(TextEvent::SELECTION_CHANGED);
                                    }
                                }
                                Key::ArrowUp => {
                                    let initial_selection = editor.get_selection();

                                    if shift {
                                        editor.expand_selection_to_cursor();
                                    } else {
                                        editor.clear_selection();
                                    }

                                    if cursor_up(&mut *editor, paragraph) {
                                        event.insert(TextEvent::CURSOR_CHANGED);
                                    }

                                    if shift {
                                        editor.expand_selection_to_cursor();
                                    }

                                    if initial_selection != editor.get_selection() {
                                        event.insert(TextEvent::SELECTION_CHANGED);
                                    }
                                }
                                Key::ArrowDown => {
                                    if shift {
                                        editor.expand_selection_to_cursor();
                                    } else {
                                        editor.clear_selection();
                                    }

                                    if cursor_down(&mut *editor, paragraph) {
                                        event.insert(TextEvent::CURSOR_CHANGED);
                                    }

                                    if shift {
                                        editor.expand_selection_to_cursor();
                                    }
                                }
                                Key::Home => {
                                    let initial_selection = editor.get_selection();

                                    if shift {
                                        editor.expand_selection_to_cursor();
                                    }

                                    // Move to either start of line or start of buffer depending on if ctrl is pressed.
                                    if cursor_left(
                                        &mut *editor,
                                        paragraph,
                                        if meta_or_ctrl {
                                            CursorMovement::Buffer
                                        } else {
                                            CursorMovement::Line
                                        },
                                    ) {
                                        event.insert(TextEvent::CURSOR_CHANGED);
                                    }

                                    if shift {
                                        editor.expand_selection_to_cursor();
                                    }

                                    if initial_selection != editor.get_selection() {
                                        event.insert(TextEvent::SELECTION_CHANGED);
                                    }
                                }
                                Key::End => {
                                    let initial_selection = editor.get_selection();

                                    if shift {
                                        editor.expand_selection_to_cursor();
                                    }

                                    // Move to either end of line or end of buffer depending on if ctrl is pressed.
                                    if cursor_right(
                                        &mut *editor,
                                        paragraph,
                                        if meta_or_ctrl {
                                            CursorMovement::Buffer
                                        } else {
                                            CursorMovement::Line
                                        },
                                    ) {
                                        event.insert(TextEvent::CURSOR_CHANGED);
                                    }

                                    if shift {
                                        editor.expand_selection_to_cursor();
                                    }

                                    if initial_selection != editor.get_selection() {
                                        event.insert(TextEvent::SELECTION_CHANGED);
                                    }
                                }
                                Key::Backspace if config.allow_changes => {
                                    let cursor_pos = editor.cursor_pos();
                                    let selection = editor.get_selection_range();

                                    if let Some((start, end)) = selection {
                                        editor.remove(start..end);
                                        editor.set_cursor_pos(start);
                                        event.insert(TextEvent::TEXT_CHANGED);
                                    } else if cursor_pos > 0 {
                                        // Remove the character to the left if there is any
                                        let removed_text_len =
                                            editor.remove(cursor_pos - 1..cursor_pos);
                                        editor.set_cursor_pos(cursor_pos - removed_text_len);
                                        event.insert(TextEvent::TEXT_CHANGED);
                                    }
                                }
                                Key::Delete if config.allow_changes => {
                                    let cursor_pos = editor.cursor_pos();
                                    let selection = editor.get_selection_range();

                                    if let Some((start, end)) = selection {
                                        editor.remove(start..end);
                                        editor.set_cursor_pos(start);
                                        event.insert(TextEvent::TEXT_CHANGED);
                                    } else if cursor_pos < editor.len_utf16_cu() {
                                        // Remove the character to the right if there is any
                                        editor.remove(cursor_pos..cursor_pos + 1);
                                        event.insert(TextEvent::TEXT_CHANGED);
                                    }
                                }
                                Key::Enter if config.allow_changes => {
                                    // Breaks the line
                                    let cursor_pos = editor.cursor_pos();
                                    editor.insert_char('\n', cursor_pos);
                                    cursor_right(&mut *editor, paragraph, CursorMovement::Glyph);

                                    event.insert(TextEvent::TEXT_CHANGED);
                                }
                                Key::Tab if config.allow_tabs && config.allow_changes => {
                                    // Inserts a tab
                                    let text = " ".repeat(editor.get_identation().into());
                                    let cursor_pos = editor.cursor_pos();
                                    editor.insert(&text, cursor_pos);
                                    editor.set_cursor_pos(cursor_pos + text.chars().count());

                                    event.insert(TextEvent::TEXT_CHANGED);
                                }
                                Key::Character(character) => {
                                    match code {
                                        Code::Delete if config.allow_changes => {}
                                        Code::Space if config.allow_changes => {
                                            let selection = editor.get_selection_range();
                                            if let Some((start, end)) = selection {
                                                editor.remove(start..end);
                                                editor.set_cursor_pos(start);
                                                event.insert(TextEvent::TEXT_CHANGED);
                                            }

                                            // Simply adds an space
                                            let cursor_pos = editor.cursor_pos();
                                            editor.insert_char(' ', cursor_pos);
                                            cursor_right(
                                                &mut *editor,
                                                paragraph,
                                                CursorMovement::Glyph,
                                            );

                                            event.insert(TextEvent::TEXT_CHANGED);
                                        }

                                        // Select all text
                                        Code::KeyA if meta_or_ctrl => {
                                            let len = editor.len_utf16_cu();
                                            editor.set_selection((0, len));

                                            if cursor_right(&mut *editor, paragraph, CursorMovement::Buffer) {
                                                event.insert(TextEvent::CURSOR_CHANGED);
                                            }

                                            event.insert(TextEvent::SELECTION_CHANGED);
                                        }

                                        // Copy selected text
                                        Code::KeyC if meta_or_ctrl && config.allow_clipboard => {
                                            let selected = editor.get_selected_text();
                                            if let Some(selected) = selected {
                                                editor.get_clipboard().set(selected).ok();
                                            }
                                        }

                                        // Cut selected text
                                        Code::KeyX
                                            if meta_or_ctrl
                                                && config.allow_changes
                                                && config.allow_clipboard =>
                                        {
                                            let selection = editor.get_selection_range();
                                            if let Some((start, end)) = selection {
                                                let text = editor.get_selected_text().unwrap();
                                                editor.remove(start..end);
                                                editor.get_clipboard().set(text).ok();
                                                editor.set_cursor_pos(start);
                                                event.insert(TextEvent::TEXT_CHANGED);
                                            }
                                        }

                                        // Paste copied text
                                        Code::KeyV
                                            if meta_or_ctrl
                                                && config.allow_changes
                                                && config.allow_clipboard =>
                                        {
                                            let copied_text = editor.get_clipboard().get();
                                            if let Ok(copied_text) = copied_text {
                                                let selection = editor.get_selection_range();
                                                if let Some((start, end)) = selection {
                                                    editor.remove(start..end);
                                                    editor.set_cursor_pos(start);
                                                }
                                                let cursor_pos = editor.cursor_pos();
                                                editor.insert(&copied_text, cursor_pos);
                                                let last_idx =
                                                    copied_text.encode_utf16().count() + cursor_pos;
                                                editor.set_cursor_pos(last_idx);
                                                event.insert(TextEvent::TEXT_CHANGED);
                                            }
                                        }

                                        // Undo last change
                                        Code::KeyZ if meta_or_ctrl && config.allow_changes => {
                                            let undo_result = editor.undo();

                                            if let Some(idx) = undo_result {
                                                editor.set_cursor_pos(idx);
                                                event.insert(TextEvent::TEXT_CHANGED);
                                            }
                                        }

                                        // Redo last change
                                        Code::KeyY if meta_or_ctrl && config.allow_changes => {
                                            let redo_result = editor.redo();

                                            if let Some(idx) = redo_result {
                                                editor.set_cursor_pos(idx);
                                                event.insert(TextEvent::TEXT_CHANGED);
                                            }
                                        }

                                        _ if config.allow_changes => {
                                            // Remove selected text
                                            let selection = editor.get_selection_range();
                                            if let Some((start, end)) = selection {
                                                editor.remove(start..end);
                                                editor.set_cursor_pos(start);
                                                event.insert(TextEvent::TEXT_CHANGED);
                                            }

                                            if let Ok(ch) = character.parse::<char>() {
                                                // Inserts a character
                                                let cursor_pos = editor.cursor_pos();
                                                let inserted_text_len =
                                                    editor.insert_char(ch, cursor_pos);
                                                editor
                                                    .set_cursor_pos(cursor_pos + inserted_text_len);

                                                event.insert(TextEvent::TEXT_CHANGED);
                                            } else {
                                                // Inserts a text
                                                let cursor_pos = editor.cursor_pos();
                                                let inserted_text_len =
                                                    editor.insert(character, cursor_pos);
                                                editor
                                                    .set_cursor_pos(cursor_pos + inserted_text_len);

                                                event.insert(TextEvent::TEXT_CHANGED);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }

                            if event.contains(TextEvent::TEXT_CHANGED) {
                                editor.clear_selection();
                                *dragging.write() = TextDragging::None;
                            }

                            !event.is_empty()
                        });
                    }
                }
            }
            EditableEvent::KeyUp { code } => {
                if code == Code::ShiftLeft {
                    if let TextDragging::FromCursorToPoint { shift, .. } = &mut *dragging.write() {
                        *shift = false;
                    }
                } else {
                    *dragging.write() = TextDragging::None;
                }
            }
        };
    }
}

/// Indicates the type of text dragging being done.
#[derive(Debug, PartialEq, Clone)]
pub enum TextDragging {
    None,
    FromPointToPoint {
        src: CursorPoint,
    },
    FromCursorToPoint {
        shift: bool,
        clicked: bool,
        cursor: usize,
        dist: Option<CursorPoint>,
    },
}

impl TextDragging {
    pub fn has_cursor_coords(&self) -> bool {
        match self {
            Self::None => false,
            Self::FromPointToPoint { .. } => true,
            Self::FromCursorToPoint { dist, .. } => dist.is_some(),
        }
    }

    pub fn set_cursor_coords(&mut self, cursor: CursorPoint) {
        match self {
            Self::FromPointToPoint { src } => *src = cursor,
            Self::FromCursorToPoint {
                dist, shift: true, ..
            } => *dist = Some(cursor),
            _ => *self = Self::FromPointToPoint { src: cursor },
        }
    }

    pub fn get_cursor_coords(&self) -> Option<CursorPoint> {
        match self {
            Self::None => None,
            Self::FromPointToPoint { src } => Some(*src),
            Self::FromCursorToPoint { dist, clicked, .. } => {
                if *clicked {
                    *dist
                } else {
                    None
                }
            }
        }
    }
}
