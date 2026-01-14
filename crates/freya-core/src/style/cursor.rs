#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CursorStyle {
    Line = 0,
    Block = 1,
    Underline = 2,
}

impl Default for CursorStyle {
    fn default() -> Self {
        Self::Line
    }
}

impl CursorStyle {
    pub fn pretty(&self) -> String {
        match self {
            Self::Line => "line".to_string(),
            Self::Block => "block".to_string(),
            Self::Underline => "underline".to_string(),
        }
    }
}
