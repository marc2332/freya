use torin::scaled::Scaled;

use crate::{
    Fill,
    Parse,
    ParseError,
    Parser,
    Token,
};

#[derive(Default, Clone, Debug, PartialEq)]
pub enum ShadowPosition {
    #[default]
    Normal,
    Inset,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Shadow {
    pub position: ShadowPosition,
    pub x: f32,
    pub y: f32,
    pub blur: f32,
    pub spread: f32,
    pub fill: Fill,
}

impl Parse for Shadow {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut shadow = Shadow::default();

        if parser.try_consume(&Token::ident("none")) {
            return Ok(shadow);
        }

        if parser.try_consume(&Token::ident("inset")) {
            shadow.position = ShadowPosition::Inset;
        }

        shadow.x = parser.consume_map(Token::as_float)?;
        shadow.y = parser.consume_map(Token::as_float)?;
        shadow.blur = parser.consume_map(Token::as_float)?;

        if let Ok(spread) = parser.consume_map(Token::as_float) {
            shadow.spread = spread;
        }

        shadow.fill = Fill::parse(parser)?;

        Ok(shadow)
    }
}

impl Scaled for Shadow {
    fn scale(&mut self, scale_factor: f32) {
        self.x *= scale_factor;
        self.y *= scale_factor;
        self.spread *= scale_factor;
        self.blur *= scale_factor;
    }
}
