/// How text that does not fit its bounds is truncated.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Debug, PartialEq, Hash)]
pub enum TextOverflow {
    /// Cut the text off at the edge. This is the default.
    #[default]
    Clip,
    /// Replace the cut-off text with an ellipsis (`…`).
    Ellipsis,
    /// Replace the cut-off text with a custom string.
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
