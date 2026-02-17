/// Vertical alignment for Paragraph text rendering.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub enum VerticalAlign {
    /// Text is aligned to the top of the paragraph area.
    #[default]
    Start,
    /// Text is vertically centered within the paragraph area.
    Center,
}

impl VerticalAlign {
    pub fn pretty(&self) -> String {
        match self {
            Self::Start => "start".to_string(),
            Self::Center => "center".to_string(),
        }
    }
}
