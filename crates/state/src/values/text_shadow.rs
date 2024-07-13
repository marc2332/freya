use freya_engine::prelude::*;

use crate::{
    Parse,
    ParseError,
    Parser,
    Token,
};

// Same as shadow, but no inset or spread.
impl Parse for TextShadow {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        Ok(TextShadow {
            offset: (
                parser.consume_map(Token::as_float)?,
                parser.consume_map(Token::as_float)?,
            )
                .into(),
            blur_sigma: parser.consume_map(Token::as_float)? as f64 / 2.0,
            color: Color::parse(parser)?,
        })
    }
}
