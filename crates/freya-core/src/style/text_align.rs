use freya_engine::prelude::SkTextAlign;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextAlign {
    Left = 0,
    Right = 1,
    Center = 2,
    Justify = 3,
    Start = 4,
    End = 5,
}

impl Default for TextAlign {
    fn default() -> Self {
        Self::Left
    }
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
