/// Whether an element can receive keyboard focus.
///
/// Converts from a `bool`, where `true` is [`Focusable::Enabled`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Focusable {
    /// Focusability is not specified, the platform decides. This is the default.
    #[default]
    Unknown,
    /// The element cannot be focused.
    Disabled,
    /// The element can be focused.
    Enabled,
}

impl From<bool> for Focusable {
    fn from(value: bool) -> Self {
        if value {
            Focusable::Enabled
        } else {
            Focusable::Disabled
        }
    }
}

impl Focusable {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled)
    }
}
