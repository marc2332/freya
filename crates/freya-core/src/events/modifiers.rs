use keyboard_types::Modifiers;

use crate::platform::TargetPlatform;

/// Extension trait for [`Modifiers`] adding platform-aware helpers.
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
        match TargetPlatform::get() {
            TargetPlatform::MacOs => Modifiers::META,
            _ => Modifiers::CONTROL,
        }
    }

    fn ctrl_or_alt() -> Modifiers {
        match TargetPlatform::get() {
            TargetPlatform::MacOs => Modifiers::ALT,
            _ => Modifiers::CONTROL,
        }
    }
}
