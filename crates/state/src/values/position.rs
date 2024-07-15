use torin::position::Position;

use crate::{
    Parse,
    ParseError,
    Parser,
};

impl Parse for Position {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|value| {
            value.try_as_str().and_then(|value| match value {
                "absolute" => Some(Self::new_absolute()),
                "stacked" => Some(Self::Stacked),
                _ => None,
            })
        })
    }
}
