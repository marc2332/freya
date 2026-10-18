use std::ops::Mul;

use freya_core::{
    elements::paragraph::{
        ParagraphCursorExt,
        ParagraphHolderInner,
    },
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
        editor_line: Option<EditorLine>,
        holder: Option<&'a ParagraphHolder>,
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
                let location = holder.visible_location(location);
                let holder = holder.0.borrow();
                let ParagraphHolderInner {
                    paragraph,
                    scale_factor,
                    ..
                } = holder.as_ref().unwrap();

                let mut text_editor = editor.write();

                if dragging.peek().shift || dragging.peek().clicked {
                    text_editor.selection_mut().set_as_range();
                } else {
                    text_editor.clear_selection();
                }

                let press_type = EventsCombos::<()>::pressed(location);
                let press_type = if press_type.is_double() && config.select_all_on_double_click {
                    PressEventType::Quadruple
                } else {
                    press_type
                };
                let char_position =
                    paragraph.cursor_index_at_point(location.mul(*scale_factor).to_tuple());
                let press_selection = text_editor.measure_selection(char_position, editor_line);
                let new_selection = match press_type {
                    PressEventType::Single => press_selection,
                    PressEventType::Double => TextSelection::new_range(
                        text_editor.find_word_boundaries(press_selection.pos()),
                    ),
                    PressEventType::Triple => TextSelection::new_range(
                        text_editor.find_line_boundaries(press_selection.pos()),
                    ),
                    PressEventType::Quadruple => {
                        TextSelection::new_range((0, text_editor.len_utf16_cu()))
                    }
                };

                dragging
                    .write()
                    .start_selection(press_type, new_selection.clone());
                if *text_editor.selection() != new_selection {
                    *text_editor.selection_mut() = new_selection;
                }
            }
            EditableEvent::Move {
                location,
                editor_line,
                holder,
            } => {
                let location = holder.visible_location(location);
                if dragging.peek().clicked {
                    EventsCombos::<()>::moved(location);

                    let paragraph = holder.0.borrow();
                    let ParagraphHolderInner {
                        paragraph,
                        scale_factor,
                        ..
                    } = paragraph.as_ref().unwrap();

                    let dist_position = location.mul(*scale_factor);

                    // Calculate the end of the highlighting
                    let to = paragraph.cursor_index_at_point(dist_position.to_tuple());

                    if editor.peek().get_selection().is_none() {
                        editor.write().selection_mut().set_as_range();
                    }

                    let current_selection = editor.peek().selection().clone();

                    let new_selection =
                        dragging
                            .peek()
                            .measure_selection(&*editor.peek(), to, editor_line);

                    // Update the cursor if it has changed
                    if current_selection != new_selection {
                        let mut text_editor = editor.write();
                        *text_editor.selection_mut() = new_selection;
                    }
                }
            }
            EditableEvent::Release => {
                dragging.write().clicked = false;
                EventsCombos::<()>::released();
            }
            EditableEvent::KeyDown {
                key,
                modifiers,
                editor_line,
                holder,
            } => {
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
                                editor_line,
                                holder,
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
    multi_click_selection: Option<(PressEventType, TextSelection)>,
}

impl TextDragging {
    /// Remember the selection granularity and anchor of a pointer press.
    pub fn start_selection(&mut self, press_type: PressEventType, selection: TextSelection) {
        self.clicked = true;
        self.multi_click_selection = if press_type.is_single() {
            None
        } else {
            Some((press_type, selection))
        };
    }

    /// Extend a drag by the original press granularity.
    pub fn measure_selection<T: TextEditor>(
        &self,
        editor: &T,
        to: usize,
        editor_line: EditorLine,
    ) -> TextSelection {
        let selection = editor.measure_selection(to, editor_line);
        let Some((press_type, anchor)) = &self.multi_click_selection else {
            return selection;
        };
        let position = selection.pos();
        if press_type.is_quadruple() || (position >= anchor.start() && position <= anchor.end()) {
            return anchor.clone();
        }
        let (start, end) = match press_type {
            PressEventType::Double => editor.find_word_boundaries(position),
            PressEventType::Triple => editor.find_line_boundaries(position),
            _ => return selection,
        };
        if position < anchor.start() {
            TextSelection::new_range((anchor.end(), start))
        } else {
            TextSelection::new_range((anchor.start(), end))
        }
    }
}
