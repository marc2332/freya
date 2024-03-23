use std::str::FromStr;

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
        *self != Self::Paragraph && *self != Self::Text
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

impl ToString for TagName {
    fn to_string(&self) -> String {
        match self {
            TagName::Root => "root".to_string(),
            TagName::Rect => "rect".to_string(),
            TagName::Paragraph => "p".to_string(),
            TagName::Label => "label".to_string(),
            TagName::Text => "text".to_string(),
            TagName::Image => "img".to_string(),
            TagName::Svg => "svg".to_string(),
        }
    }
}
