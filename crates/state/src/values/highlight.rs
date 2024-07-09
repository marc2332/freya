use crate::{
    Parse,
    ParseError,
};

#[derive(Default, Clone, Debug, PartialEq)]
pub enum HighlightMode {
    #[default]
    /// Highlight considering the actual measure text bounds.
    Fit,
    /// Highlight considering the `paragraph` element bounds.
    Expanded,
}

impl Parse for HighlightMode {
    fn parse(value: &str) -> Result<Self, ParseError> {
        match value {
            "expanded" => Ok(HighlightMode::Expanded),
            _ => Ok(HighlightMode::Fit),
        }
    }
}
