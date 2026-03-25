use freya_engine::prelude::SkTextDecoration;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextDecoration {
    #[default]
    None,
    Underline,
    Overline,
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
