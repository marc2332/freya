use freya_engine::prelude::SkTextDecoration;

/// A line drawn through, under or over text.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextDecoration {
    /// No decoration. This is the default.
    #[default]
    None,
    /// A line below the text.
    Underline,
    /// A line above the text.
    Overline,
    /// A line through the middle of the text.
    LineThrough,
}

impl TextDecoration {
    pub fn pretty(&self) -> String {
        match self {
            Self::None => "none".to_string(),
            Self::Underline => "underline".to_string(),
            Self::Overline => "overline".to_string(),
            Self::LineThrough => "line-through".to_string(),
        }
    }
}

impl From<TextDecoration> for SkTextDecoration {
    fn from(value: TextDecoration) -> Self {
        match value {
            TextDecoration::None => SkTextDecoration::NO_DECORATION,
            TextDecoration::Underline => SkTextDecoration::UNDERLINE,
            TextDecoration::Overline => SkTextDecoration::OVERLINE,
            TextDecoration::LineThrough => SkTextDecoration::LINE_THROUGH,
        }
    }
}
