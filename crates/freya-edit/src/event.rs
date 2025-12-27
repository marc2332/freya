use std::ops::Mul;

use freya_core::{
    elements::paragraph::ParagraphHolderInner,
    prelude::*,
};
use keyboard_types::NamedKey;
use torin::prelude::CursorPoint;

use crate::{
    EditableConfig,
    EditorLine,
    TextSelection,
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
        editor_line: EditorLine,
        holder: &'a ParagraphHolder,
    },
    Down {
        location: CursorPoint,
        editor_line: EditorLine,
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
                editor_line,
                holder,
            } => {
                let holder = holder.0.borrow();
                let ParagraphHolderInner {
                    paragraph,
                    scale_factor,
                } = holder.as_ref().unwrap();

                let mut text_editor = editor.write();

                if dragging.peek().shift || dragging.peek().clicked {
                    text_editor.selection_mut().set_as_range();
                } else {
                    text_editor.clear_selection();
                }

                dragging.write().clicked = true;

                match EventsCombos::pressed(location) {
                    PressEventType::Triple => {
                        let current_selection = text_editor.selection().clone();

                        let char_position = paragraph.get_glyph_position_at_coordinate(
                            location.mul(*scale_factor).to_i32().to_tuple(),
                        );
                        let press_selection = text_editor
                            .measure_selection(char_position.position as usize, editor_line);

                        // Get the line start char and its length
                        let line = text_editor.rope().char_to_line(press_selection.pos());
                        let line_char = text_editor.rope().line_to_char(line);
                        let line_len = text_editor.rope().line(line).len_utf16_cu();
                        let new_selection =
                            TextSelection::new_range((line_char, line_char + line_len));

                        // Select the whole line
                        if current_selection != new_selection {
                            *text_editor.selection_mut() = new_selection;
                        }
                    }
                    PressEventType::Double => {
                        let current_selection = text_editor.selection().clone();

                        let char_position = paragraph.get_glyph_position_at_coordinate(
                            location.mul(*scale_factor).to_i32().to_tuple(),
                        );
                        let press_selection = text_editor
                            .measure_selection(char_position.position as usize, editor_line);

                        // Find word boundaries
                        let range = text_editor.find_word_boundaries(press_selection.pos());
                        let new_selection = TextSelection::new_range(range);

                        // Select the word
                        if current_selection != new_selection {
                            *text_editor.selection_mut() = new_selection;
                        }
                    }
                    PressEventType::Single => {
                        let current_selection = text_editor.selection().clone();

                        let char_position = paragraph.get_glyph_position_at_coordinate(
                            location.mul(*scale_factor).to_i32().to_tuple(),
                        );
                        let new_selection = text_editor
                            .measure_selection(char_position.position as usize, editor_line);

                        // Move the cursor
                        if current_selection != new_selection {
                            *text_editor.selection_mut() = new_selection;
                        }
                    }
                }
            }
            EditableEvent::Move {
                location,
                editor_line,
                holder,
            } => {
                if dragging.peek().clicked {
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

                    if !editor.peek().selection.is_range() {
                        editor.write().selection_mut().set_as_range();
                    }

                    let current_selection = editor.peek().selection().clone();

                    let new_selection = editor.peek().measure_selection(to, editor_line);

                    // Update the cursor if it has changed
                    if current_selection != new_selection {
                        let mut text_editor = editor.write();
                        *text_editor.selection_mut() = new_selection;
                    }
                }
            }
            EditableEvent::Release => {
                dragging.write().clicked = false;
            }
            EditableEvent::KeyDown { key, modifiers } => {
                match key {
                    // Handle dragging
                    Key::Named(NamedKey::Shift) => {
                        dragging.write().shift = true;
                    }
                    // Handle editing
                    _ => {
                        editor.write_if(|mut editor| {
                            let event = editor.process_key(
                                key,
                                &modifiers,
                                config.allow_tabs,
                                config.allow_changes,
                                config.allow_clipboard,
                            );
                            if event.contains(TextEvent::TEXT_CHANGED) {
                                *dragging.write() = TextDragging::default();
                            }
                            !event.is_empty()
                        });
                    }
                }
            }
            EditableEvent::KeyUp { key, .. } => {
                if *key == Key::Named(NamedKey::Shift) {
                    dragging.write().shift = false;
                }
            }
        };
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TextDragging {
    pub shift: bool,
    pub clicked: bool,
}
