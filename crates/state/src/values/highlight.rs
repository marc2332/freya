use crate::Parse;

#[derive(Default, Clone, Debug, PartialEq)]
pub enum HighlightMode {
    #[default]
    /// Highlight considering the actual measure text bounds.
    Fit,
    /// Highlight considering the `paragraph` element bounds.
    Expanded,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HighlightModeError;

impl Parse for HighlightMode {
    type Err = HighlightModeError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        match value {
            "expanded" => Ok(HighlightMode::Expanded),
            _ => Ok(HighlightMode::Fit),
        }
    }
}
