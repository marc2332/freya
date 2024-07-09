use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy, PartialEq, Debug, Hash)]
pub enum TagName {
    Root,
    Rect,
    Paragraph,
    Label,
    Text,
    Image,
    Svg,
}

impl TagName {
    pub fn has_intrinsic_layout(&self) -> bool {
        *self != Self::Text
    }

    pub fn has_children_with_intrinsic_layout(&self) -> bool {
        *self != Self::Paragraph && *self != Self::Label
    }

    pub fn contains_text(&self) -> bool {
        matches!(self, Self::Paragraph | Self::Label | Self::Text)
    }
}

impl FromStr for TagName {
    type Err = ();

    fn from_str(txt: &str) -> Result<Self, Self::Err> {
        match txt {
            "rect" => Ok(TagName::Rect),
            "paragraph" => Ok(TagName::Paragraph),
            "label" => Ok(TagName::Label),
            "text" => Ok(TagName::Text),
            "image" => Ok(TagName::Image),
            "svg" => Ok(TagName::Svg),
            _ => Err(()),
        }
    }
}

impl Display for TagName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TagName::Root => f.write_str("root"),
            TagName::Rect => f.write_str("rect"),
            TagName::Paragraph => f.write_str("p"),
            TagName::Label => f.write_str("label"),
            TagName::Text => f.write_str("text"),
            TagName::Image => f.write_str("img"),
            TagName::Svg => f.write_str("svg"),
        }
    }
}
