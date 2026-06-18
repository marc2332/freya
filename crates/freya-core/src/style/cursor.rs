/// Shape of the text cursor drawn in a [`paragraph`](crate::elements::paragraph).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CursorStyle {
    /// A thin vertical line between glyphs. This is the default.
    #[default]
    Line = 0,
    /// A filled block covering the glyph.
    Block = 1,
    /// A line under the glyph.
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
