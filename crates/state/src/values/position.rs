use torin::position::Position;

use crate::{
    Parse,
    ParseError,
    Parser,
};

impl Parse for Position {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|value| {
            value.as_string().and_then(|value| match value {
                "absolute" => Some(Self::new_absolute()),
                "stacked" => Some(Self::Stacked),
                _ => None,
            })
        })
    }
}
