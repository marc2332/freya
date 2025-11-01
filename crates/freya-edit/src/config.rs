#[derive(Clone, Copy, PartialEq, Debug)]
pub struct EditableConfig {
    pub(crate) identation: u8,
    pub(crate) allow_tabs: bool,
    pub(crate) allow_changes: bool,
    pub(crate) allow_clipboard: bool,
}

impl Default for EditableConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl EditableConfig {
    /// Create a [`EditableConfig`].
    pub fn new() -> Self {
        Self {
            identation: 4,
            allow_tabs: false,
            allow_changes: true,
            allow_clipboard: true,
        }
    }

    /// Specify a custom identation
    pub fn with_identation(mut self, identation: u8) -> Self {
        self.identation = identation;
        self
    }

    /// Specify whether you want to allow tabs to be inserted
    pub fn with_allow_tabs(mut self, allow_tabs: bool) -> Self {
        self.allow_tabs = allow_tabs;
        self
    }

    /// Allow changes through keyboard events or not
    pub fn with_allow_changes(mut self, allow_changes: bool) -> Self {
        self.allow_changes = allow_changes;
        self
    }

    /// Allow clipboard keyboard events
    pub fn with_allow_clipboard(mut self, allow_clipboard: bool) -> Self {
        self.allow_clipboard = allow_clipboard;
        self
    }
}
