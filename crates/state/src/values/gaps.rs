use torin::gaps::Gaps;

use crate::{
    Parse,
    ParseError,
    Parser,
    Token,
};

impl Parse for Gaps {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut paddings = Gaps::default();

        if parser.try_consume(&Token::ident("none")) {
            return Ok(paddings);
        }

        match (
            parser.consume_map(Token::try_as_f32)?,
            parser.consume_map(Token::try_as_f32).ok(),
            parser.consume_map(Token::try_as_f32).ok(),
            parser.consume_map(Token::try_as_f32).ok(),
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
