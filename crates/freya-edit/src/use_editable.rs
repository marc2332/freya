use std::time::Duration;

use freya_core::prelude::*;

use crate::{
    EditableConfig,
    EditableEvent,
    EditableMode,
    TextDragging,
    editor_history::EditorHistory,
    rope_editor::RopeEditor,
    text_editor::TextCursor,
};

/// Manage an editable text.
#[derive(Clone, Copy, PartialEq)]
pub struct UseEditable {
    pub(crate) editor: State<RopeEditor>,
    pub(crate) dragging: State<TextDragging>,
    pub(crate) config: EditableConfig,
}

impl UseEditable {
    /// Manually create an editable content instead of using [use_editable].
    pub fn create(content: String, config: EditableConfig, mode: EditableMode) -> Self {
        let editor = State::create(RopeEditor::new(
            content,
            TextCursor::default(),
            config.identation,
            mode,
            EditorHistory::new(Duration::from_millis(10)),
        ));
        let dragging = State::create(TextDragging::None);

        UseEditable {
            editor,
            dragging,
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

    /// Process a [`EditableEvent`] event.
    pub fn process_event(&mut self, edit_event: EditableEvent) {
        edit_event.process(self.editor, self.dragging, &self.config);
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
    mode: EditableMode,
) -> UseEditable {
    use_hook(|| UseEditable::create(content(), config(), mode))
}
