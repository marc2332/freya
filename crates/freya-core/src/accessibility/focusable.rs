#[derive(Clone, Debug, PartialEq, Default, Eq)]
pub enum Focusable {
    #[default]
    Unknown,
    Disabled,
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
