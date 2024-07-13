use crate::{
    Parse,
    ParseError,
    Parser,
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
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|value| {
            value.as_string().and_then(|value| match value {
                "expanded" => Some(Self::Expanded),
                "fit" => Some(Self::Fit),
                _ => None,
            })
        })
    }
}
