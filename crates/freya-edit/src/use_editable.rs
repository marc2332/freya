use std::time::Duration;

use freya_core::prelude::*;

use crate::{
    EditableConfig,
    EditableEvent,
    TextDragging,
    TouchEditEvent,
    TouchSelection,
    editor_history::EditorHistory,
    rope_editor::RopeEditor,
    text_editor::TextSelection,
};

/// Manage an editable text.
#[derive(Clone, Copy, PartialEq)]
pub struct UseEditable {
    pub(crate) editor: State<RopeEditor>,
    pub(crate) dragging: State<TextDragging>,
    pub(crate) touch: State<TouchSelection>,
    pub(crate) config: EditableConfig,
}

impl UseEditable {
    /// Manually create an editable content instead of using [use_editable].
    pub fn create(content: String, config: EditableConfig) -> Self {
        let editor = State::create(RopeEditor::new(
            content,
            TextSelection::new_cursor(0),
            config.indentation,
            EditorHistory::new(Duration::from_millis(10)),
        ));
        let dragging = State::create(TextDragging::default());
        let touch = State::create(TouchSelection::default());

        UseEditable {
            editor,
            dragging,
            touch,
            config,
        }
    }

    /// Reference to the editor.
    pub fn editor(&self) -> &State<RopeEditor> {
        &self.editor
    }

    /// Mutable reference to the editor.
    pub fn editor_mut(&mut self) -> &mut State<RopeEditor> {
        &mut self.editor
    }

    /// Touch state: the selection handles and toolbar to show.
    pub fn touch(&self) -> &State<TouchSelection> {
        &self.touch
    }

    /// Mutable touch state.
    pub fn touch_mut(&mut self) -> &mut State<TouchSelection> {
        &mut self.touch
    }

    /// Process a [`EditableEvent`] event.
    pub fn process_event(&mut self, edit_event: EditableEvent) {
        // Typing or using a mouse puts touch controls away.
        if matches!(
            edit_event,
            EditableEvent::Down { .. } | EditableEvent::KeyDown { .. }
        ) && self.touch.peek().handles_visible()
        {
            self.touch.write().hide();
        }
        edit_event.process(
            self.editor.into_writable(),
            self.dragging.into_writable(),
            &self.config,
        );
    }

    /// Process a [`TouchEditEvent`] event.
    pub fn process_touch(&mut self, touch_event: TouchEditEvent) {
        touch_event.process(self.editor.into_writable(), self.touch.into_writable());
    }
}

/// Hook to create an editable text.
///
/// For manual creation use [UseEditable::create].
///
/// **This is a low level hook and is not expected to be used by the common user, in fact,
/// you might be looking for something like the `Input` component instead.**
pub fn use_editable(
    content: impl FnOnce() -> String,
    config: impl FnOnce() -> EditableConfig,
) -> UseEditable {
    use_hook(|| UseEditable::create(content(), config()))
}
