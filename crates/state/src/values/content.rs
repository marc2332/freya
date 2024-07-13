use torin::content::Content;

use crate::{
    Parse,
    ParseError,
    Parser,
};

impl Parse for Content {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|value| {
            value.as_string().and_then(|value| match value {
                "normal" => Some(Self::Normal),
                "fit" => Some(Self::Fit),
                _ => None,
            })
        })
    }
}
