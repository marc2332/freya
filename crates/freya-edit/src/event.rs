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
    text_editor::{
        TextEditor,
        TextEvent,
    },
};

#[derive(Debug)]
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
    pub fn process<T: TextEditor>(
        self,
        mut editor: Writable<T>,
        mut dragging: Writable<TextDragging>,
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

                let mut press = EventsCombos::pressed(location);
                // A masked input has no words to offer, so a double press there means
                // what a triple one would.
                if press == PressEventType::Double && config.select_all_on_double_click {
                    press = PressEventType::Quadruple;
                }

                let char_position = paragraph.get_glyph_position_at_coordinate(
                    location.mul(*scale_factor).to_i32().to_tuple(),
                );
                let measured =
                    text_editor.measure_selection(char_position.position as usize, editor_line);
                let current_selection = text_editor.selection().clone();
                let new_selection = text_editor.press_selection(measured.pos(), press, measured);

                if current_selection != new_selection {
                    *text_editor.selection_mut() = new_selection.clone();
                }

                let mut dragging = dragging.write();
                dragging.clicked = true;
                dragging.pressed(press, &new_selection);
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

                    if editor.peek().get_selection().is_none() {
                        editor.write().selection_mut().set_as_range();
                    }

                    let drag = dragging.peek().clone();
                    let text_editor = editor.peek();
                    let current_selection = text_editor.selection().clone();
                    let pointer = text_editor.measure_selection(to, editor_line).pos();
                    let new_selection =
                        text_editor.drag_selection(pointer, &drag, current_selection.clone());
                    drop(text_editor);

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
                                config.allow_read_clipboard,
                                config.allow_write_clipboard,
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
    /// What the press that started this drag selected, and the bounds it selected.
    /// A drag extends by the same unit, see [`TextEditor::drag_selection`].
    pub press: PressEventType,
    pub anchor: (usize, usize),
}

impl TextDragging {
    /// Record what a press selected, so the drag that may follow extends from it.
    pub fn pressed(&mut self, press: PressEventType, selection: &TextSelection) {
        self.press = press;
        let (start, end) = (selection.start(), selection.end());
        self.anchor = (start.min(end), start.max(end));
    }
}
