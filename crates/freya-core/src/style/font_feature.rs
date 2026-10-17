use std::borrow::Cow;

/// An OpenType feature of the font, enabled with a value.
///
/// Implements `From<&'static str>`, which enables the feature with a value of `1`,
/// and `From<(&'static str, i32)>`. Common features are available as constants,
/// like [`FontFeature::TABULAR_NUMBERS`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct FontFeature {
    pub name: Cow<'static, str>,
    pub value: i32,
}

impl FontFeature {
    pub const fn new(name: &'static str, value: i32) -> Self {
        FontFeature {
            name: Cow::Borrowed(name),
            value,
        }
    }

    /// Every digit shares the same width, so changing numbers do not shift the layout.
    pub const TABULAR_NUMBERS: Self = Self::new("tnum", 1);
    /// Lowercase letters render as small capitals.
    pub const SMALL_CAPS: Self = Self::new("smcp", 1);
    /// Zero renders with a slash or dot to tell it apart from the letter O.
    pub const SLASHED_ZERO: Self = Self::new("zero", 1);
    /// Sequences like `1/2` render as fractions.
    pub const FRACTIONS: Self = Self::new("frac", 1);
    /// Standard ligatures like `fi` are disabled.
    pub const NO_LIGATURES: Self = Self::new("liga", 0);
}

impl From<&'static str> for FontFeature {
    fn from(name: &'static str) -> Self {
        Self::new(name, 1)
    }
}

impl<T: Into<Cow<'static, str>>> From<(T, i32)> for FontFeature {
    fn from((name, value): (T, i32)) -> Self {
        FontFeature {
            name: name.into(),
            value,
        }
    }
}
