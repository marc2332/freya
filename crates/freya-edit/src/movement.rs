use std::{cell::Ref, cmp::Ordering};

use freya_engine::prelude::{Paragraph, RectHeightStyle, RectWidthStyle, TextDirection};

use crate::text_editor::TextEditor;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CursorMovement {
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

/// Moves the cursor one line up.
pub fn cursor_up(editor: &mut impl TextEditor, paragraph: Option<&Ref<Paragraph>>) -> bool {
    let pos = editor.cursor_pos();

    match paragraph {
        Some(paragraph) => {
            let Some(cursor_x_pos) = paragraph
                .get_rects_for_range(pos..pos + 1, RectHeightStyle::Tight, RectWidthStyle::Tight)
                .first()
                .map(|textbox| match textbox.direct {
                    TextDirection::LTR => textbox.rect.left,
                    TextDirection::RTL => textbox.rect.right,
                })
                .or_else(|| {
                    paragraph
                        .get_rects_for_range(
                            pos - 1..pos,
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
                // Cursor is somehow not on a glyph.
                // FIXME: This can sometimes happen with emojis when fonts don't load right.
                return false;
            };

            if editor.cursor().x_pos() < cursor_x_pos {
                editor.cursor_mut().set_x_pos(cursor_x_pos);
            }

            let current_line_number = if pos == editor.len_utf16_cu() {
                Some(paragraph.line_number() - 1)
            } else {
                paragraph.get_line_number_at(pos)
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

        // Fallback implementation for SingleLineMultipleEditors when text metrics are unavailable.
        None => {
            let old_row = editor.cursor_row();
            let old_col = editor.cursor_col();

            if pos > 0 {
                // Reached max
                if old_row == 0 {
                    editor.cursor_mut().set(0);
                } else {
                    let new_row = old_row - 1;
                    let new_row_char = editor.char_to_utf16_cu(editor.line_to_char(new_row));
                    let new_row_len = editor.line(new_row).unwrap().utf16_len();

                    if editor.cursor().x_pos() < (old_col as f32) {
                        editor.cursor_mut().set_x_pos(old_col as f32);
                    }
                    let new_col =
                        (editor.cursor().x_pos() as usize).min(new_row_len.saturating_sub(1));

                    editor.cursor_mut().set(new_row_char + new_col);
                }

                true
            } else {
                false
            }
        }
    }
}

/// Move the cursor one line down.
pub fn cursor_down(editor: &mut impl TextEditor, paragraph: Option<&Ref<Paragraph>>) -> bool {
    let pos = editor.cursor_pos();

    match paragraph {
        Some(paragraph) => {
            let Some(cursor_x_pos) = paragraph
                .get_rects_for_range(pos..pos + 1, RectHeightStyle::Tight, RectWidthStyle::Tight)
                .first()
                .map(|textbox| match textbox.direct {
                    TextDirection::LTR => textbox.rect.left,
                    TextDirection::RTL => textbox.rect.right,
                })
                .or_else(|| {
                    paragraph
                        .get_rects_for_range(
                            pos - 1..pos,
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
                // Cursor is somehow not on a glyph.
                // FIXME: This can sometimes happen with emojis when fonts don't load right.
                return false;
            };

            if editor.cursor().x_pos() < cursor_x_pos {
                editor.cursor_mut().set_x_pos(cursor_x_pos);
            }

            if let Some(current_line_number) = paragraph.get_line_number_at(pos)
                && let Some(next_line_metrics) =
                    paragraph.get_line_metrics_at(current_line_number + 1)
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

        // Fallback implementation for SingleLineMultipleEditors when text metrics are unavailable.
        None => {
            let old_row = editor.cursor_row();
            let old_col = editor.cursor_col();

            match old_row.cmp(&(editor.len_lines() - 1)) {
                Ordering::Less => {
                    // One line below
                    let new_row = old_row + 1;
                    let new_row_char = editor.char_to_utf16_cu(editor.line_to_char(new_row));
                    let mut new_row_len = editor.line(new_row).unwrap().utf16_len();

                    // Consider newline characters when calculating line length.
                    //
                    // Last line doesn't have a newline, so we don't subtract from it.
                    if editor.line(new_row + 1).is_some() {
                        new_row_len = new_row_len.saturating_sub(1);
                    }

                    if editor.cursor().x_pos() < (old_col as f32) {
                        editor.cursor_mut().set_x_pos(old_col as f32);
                    }
                    let new_col = (editor.cursor().x_pos() as usize).min(new_row_len);

                    editor.cursor_mut().set(new_row_char + (new_col as usize));

                    true
                }
                Ordering::Equal => {
                    let end = editor.len_utf16_cu();
                    if pos == end {
                        return false;
                    }

                    editor.cursor_mut().set(end);
                    true
                }
                Ordering::Greater => false,
            }
        }
    }
}

/// Move the cursor to the left
pub fn cursor_backward(
    editor: &mut impl TextEditor,
    paragraph: Option<&Ref<Paragraph>>,
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
        CursorMovement::Line => {
            if let Some(paragraph) = paragraph {
                let current_line_number = if pos == editor.len_utf16_cu() {
                    Some(paragraph.line_number() - 1)
                } else {
                    paragraph.get_line_number_at(pos)
                };

                if let Some(current_line_number) = current_line_number {
                    let line_metrics = paragraph.get_line_metrics_at(current_line_number).unwrap();

                    line_metrics.start_index
                } else {
                    return false;
                }
            } else {
                editor.line_to_char(editor.cursor_row())
            }
        }
        CursorMovement::Word => {
            if let Some(paragraph) = paragraph {
                paragraph
                    .get_word_boundary(editor.cursor_pos().saturating_sub(1) as u32)
                    .start
            } else {
                // TODO
                return false;
            }
        }
        CursorMovement::Buffer => 0,
        CursorMovement::Selection => {
            let Some(selection) = editor.get_selection() else {
                return false;
            };

            selection.0.min(selection.1)
        }
    };

    if pos != target_pos {
        editor.set_cursor_pos(target_pos);

        true
    } else {
        false
    }
}

/// Move the cursor to th right
pub fn cursor_forward(
    editor: &mut impl TextEditor,
    paragraph: Option<&Ref<Paragraph>>,
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
            if let Some(paragraph) = paragraph {
                let current_line_number = if pos == editor.len_utf16_cu() {
                    Some(paragraph.line_number() - 1)
                } else {
                    paragraph.get_line_number_at(pos)
                };

                if let Some(current_line_number) = current_line_number {
                    let line_metrics = paragraph.get_line_metrics_at(current_line_number).unwrap();

                    line_metrics.end_index
                } else {
                    return false;
                }
            } else {
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
        }
        CursorMovement::Word => {
            if let Some(paragraph) = paragraph {
                paragraph.get_word_boundary(editor.cursor_pos() as u32).end
            } else {
                // TODO
                return false;
            }
        }
        CursorMovement::Buffer => editor.len_utf16_cu(),
        CursorMovement::Selection => {
            let Some(selection) = editor.get_selection() else {
                return false;
            };

            selection.0.max(selection.1)
        }
    };

    if pos != target_pos {
        editor.set_cursor_pos(target_pos);

        true
    } else {
        false
    }
}
