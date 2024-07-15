use torin::content::Content;

use crate::{
    Parse,
    ParseError,
    Parser,
};

impl Parse for Content {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|value| {
            value.try_as_str().and_then(|value| match value {
                "normal" => Some(Self::Normal),
                "fit" => Some(Self::Fit),
                _ => None,
            })
        })
    }
}
