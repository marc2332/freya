use freya_engine::prelude::SkTextAlign;

use crate::parsing::{
    Parse,
    ParseError,
};

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

impl Parse for TextAlign {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "center" => TextAlign::Center,
            "justify" => TextAlign::Justify,
            "start" => TextAlign::Start,
            "end" => TextAlign::End,
            "left" => TextAlign::Left,
            "right" => TextAlign::Right,
            _ => TextAlign::default(),
        })
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
