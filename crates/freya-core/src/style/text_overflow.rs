#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Debug, PartialEq)]
pub enum TextOverflow {
    #[default]
    Clip,
    Ellipsis,
    Custom(String),
}

impl TextOverflow {
    pub fn get_ellipsis(&self) -> Option<&str> {
        match self {
            Self::Clip => None,
            Self::Ellipsis => Some("…"),
            Self::Custom(custom) => Some(custom),
        }
    }

    pub fn pretty(&self) -> String {
        match self {
            TextOverflow::Clip => "clip".to_string(),
            TextOverflow::Ellipsis => "ellipsis".to_string(),
            TextOverflow::Custom(text_overflow) => text_overflow.to_string(),
        }
    }
}
