use std::ops::Mul;

use freya_core::{
    elements::paragraph::ParagraphHolderInner,
    prelude::*,
};
use torin::prelude::CursorPoint;

use crate::{
    EditableConfig,
    rope_editor::RopeEditor,
    text_editor::{
        TextEditor,
        TextEvent,
    },
};

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
        modifiers: Modifiers,
    },
    KeyUp {
        key: &'a Key,
    },
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
            EditableEvent::KeyDown { key, modifiers } => {
                match key {
                    // Handle dragging
                    Key::Shift => {
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
                        editor.write_if(|mut ditor| {
                            let event = ditor.process_key(
                                key,
                                &modifiers,
                                config.allow_tabs,
                                config.allow_changes,
                                config.allow_clipboard,
                            );
                            if event.contains(TextEvent::TEXT_CHANGED) {
                                *dragging.write() = TextDragging::None;
                            }
                            !event.is_empty()
                        });
                    }
                }
            }
            EditableEvent::KeyUp { key } => {
                if *key == Key::Shift {
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
