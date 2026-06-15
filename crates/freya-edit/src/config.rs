#[derive(Clone, Copy, PartialEq, Debug)]
pub struct EditableConfig {
    pub(crate) indentation: u8,
    pub(crate) allow_tabs: bool,
    pub(crate) allow_changes: bool,
    pub(crate) allow_read_clipboard: bool,
    pub(crate) allow_write_clipboard: bool,
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
            indentation: 4,
            allow_tabs: false,
            allow_changes: true,
            allow_read_clipboard: true,
            allow_write_clipboard: true,
        }
    }

    /// Specify a custom indentation
    pub fn with_indentation(mut self, indentation: u8) -> Self {
        self.indentation = indentation;
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

    /// Allow reading from the clipboard (paste).
    pub fn with_allow_read_clipboard(mut self, allow_read_clipboard: bool) -> Self {
        self.allow_read_clipboard = allow_read_clipboard;
        self
    }

    /// Allow writing to the clipboard (copy and cut).
    pub fn with_allow_write_clipboard(mut self, allow_write_clipboard: bool) -> Self {
        self.allow_write_clipboard = allow_write_clipboard;
        self
    }
}
