use torin::gaps::Gaps;

use crate::{
    Parse,
    ParseError,
    Parser,
    Token,
};

impl Parse for Gaps {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut paddings = Gaps::default();

        let value =
            parser.consume_if(|token| token == &Token::ident("none") || token.is_integer())?;

        if value == Token::ident("none") {
            return Ok(paddings);
        }

        match (
            value.into_float(),
            parser.consume_map(Token::as_float).ok(),
            parser.consume_map(Token::as_float).ok(),
            parser.consume_map(Token::as_float).ok(),
        ) {
            // Same in each directions
            (value, None, None, None) => {
                paddings.fill_all(value);
            }
            // By vertical and horizontal
            (vertical, Some(horizontal), None, None) => {
                // Vertical
                paddings.fill_vertical(vertical);

                // Horizontal
                paddings.fill_horizontal(horizontal);
            }
            // Individual vertical but same horizontal
            (top, Some(left_and_right), Some(bottom), None) => {
                paddings = Gaps::new(top, left_and_right, bottom, left_and_right);
            }
            // Each directions
            (top, Some(right), Some(bottom), Some(left)) => {
                paddings = Gaps::new(top, right, bottom, left);
            }
            _ => {}
        }

        Ok(paddings)
    }
}
