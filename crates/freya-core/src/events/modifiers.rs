use keyboard_types::Modifiers;

/// Extension trait for [`Modifiers`] adding OS-aware helpers.
pub trait ModifiersExt {
    /// Returns `true` if the platform's command modifier is pressed.
    ///
    /// Maps to [`Modifiers::META`] (Command) on macOS and to [`Modifiers::CONTROL`]
    /// on every other platform. Useful to express shortcuts like copy, paste or
    /// select-all once and have them work natively on every OS.
    fn ctrl_or_meta(&self) -> bool;
}

impl ModifiersExt for Modifiers {
    fn ctrl_or_meta(&self) -> bool {
        if cfg!(target_os = "macos") {
            self.meta()
        } else {
            self.ctrl()
        }
    }
}
