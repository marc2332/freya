#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CursorStyle {
    #[default]
    Line = 0,
    Block = 1,
    Underline = 2,
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

/// Determines how the cursor and highlights are positioned within a Paragraph.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub enum CursorMode {
    /// Cursor and highlights use the paragraph's visible_area.
    /// VerticalAlign affects cursor/highlight positions.
    #[default]
    Fit,
    /// Cursor and highlights use the paragraph's inner_area.
    /// VerticalAlign does NOT affect cursor/highlight positions.
    Expanded,
}

impl CursorMode {
    pub fn pretty(&self) -> String {
        match self {
            Self::Fit => "fit".to_string(),
            Self::Expanded => "expanded".to_string(),
        }
    }
}
