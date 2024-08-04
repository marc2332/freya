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
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|value| {
            value.try_as_str().and_then(|value| match value {
                "expanded" => Some(Self::Expanded),
                "fit" => Some(Self::Fit),
                _ => None,
            })
        })
    }
}
