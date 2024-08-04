use freya_engine::prelude::*;

use crate::{
    Parse,
    ParseError,
    Parser,
    Token,
};

// Same as shadow, but no inset or spread.
impl Parse for TextShadow {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        Ok(TextShadow {
            offset: (
                parser.consume_map(Token::try_as_f32)?,
                parser.consume_map(Token::try_as_f32)?,
            )
                .into(),
            blur_sigma: parser.consume_map(Token::try_as_f32)? as f64 / 2.0,
            color: Color::from_parser(parser)?,
        })
    }
}
