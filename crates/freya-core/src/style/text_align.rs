use freya_engine::prelude::SkTextAlign;

/// Horizontal alignment of text within its element.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextAlign {
    /// Align text to the left edge. This is the default.
    #[default]
    Left = 0,
    /// Align text to the right edge.
    Right = 1,
    /// Center the text horizontally.
    Center = 2,
    /// Stretch each line to fill the width, except the last line.
    Justify = 3,
    /// Align text to the start edge, following the text direction.
    Start = 4,
    /// Align text to the end edge, following the text direction.
    End = 5,
}

impl From<TextAlign> for SkTextAlign {
    fn from(value: TextAlign) -> Self {
        match value {
            TextAlign::Left => SkTextAlign::Left,
            TextAlign::Right => SkTextAlign::Right,
            TextAlign::Center => SkTextAlign::Center,
            TextAlign::Justify => SkTextAlign::Justify,
            TextAlign::Start => SkTextAlign::Start,
            TextAlign::End => SkTextAlign::End,
        }
    }
}

impl TextAlign {
    pub fn pretty(&self) -> String {
        match self {
            Self::Left => "left".to_string(),
            Self::Right => "right".to_string(),
            Self::Center => "center".to_string(),
            Self::Justify => "justify".to_string(),
            Self::Start => "start".to_string(),
            Self::End => "end".to_string(),
        }
    }
}
