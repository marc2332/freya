use keyboard_types::Modifiers;

/// Extension trait for [`Modifiers`] adding OS-aware helpers.
pub trait ModifiersExt {
    /// Returns the platform's command modifier.
    ///
    /// Maps to [`Modifiers::META`] (Command) on macOS and to [`Modifiers::CONTROL`]
    /// on every other platform. Useful to express shortcuts like copy, paste or
    /// select-all once and have them work natively on every OS.
    fn ctrl_or_meta() -> Modifiers;
}

impl ModifiersExt for Modifiers {
    fn ctrl_or_meta() -> Modifiers {
        if cfg!(target_os = "macos") {
            Modifiers::META
        } else {
            Modifiers::CONTROL
        }
    }
}
