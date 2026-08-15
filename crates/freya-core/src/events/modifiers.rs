use keyboard_types::Modifiers;

/// Extension trait for [`Modifiers`] adding OS-aware helpers.
pub trait ModifiersExt {
    /// Returns the platform's command modifier.
    ///
    /// Maps to [`Modifiers::META`] (Command) on macOS and to [`Modifiers::CONTROL`]
    /// on every other platform. Useful to express shortcuts like copy, paste or
    /// select-all once and have them work natively on every OS.
    fn ctrl_or_meta() -> Modifiers;

    /// Returns the platform's word navigation modifier.
    ///
    /// Maps to [`Modifiers::ALT`] (Option) on macOS and to [`Modifiers::CONTROL`]
    /// on every other platform. Used for word-wise cursor movement and deletion.
    fn ctrl_or_alt() -> Modifiers;
}

impl ModifiersExt for Modifiers {
    fn ctrl_or_meta() -> Modifiers {
        if cfg!(target_os = "macos") {
            Modifiers::META
        } else {
            Modifiers::CONTROL
        }
    }

    fn ctrl_or_alt() -> Modifiers {
        if cfg!(target_os = "macos") {
            Modifiers::ALT
        } else {
            Modifiers::CONTROL
        }
    }
}
