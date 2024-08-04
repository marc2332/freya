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
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut shadow = Shadow::default();

        if parser.try_consume(&Token::ident("none")) {
            return Ok(shadow);
        }

        if parser.try_consume(&Token::ident("inset")) {
            shadow.position = ShadowPosition::Inset;
        }

        shadow.x = parser.consume_map(Token::try_as_f32)?;
        shadow.y = parser.consume_map(Token::try_as_f32)?;
        shadow.blur = parser.consume_map(Token::try_as_f32)?;

        if let Ok(spread) = parser.consume_map(Token::try_as_f32) {
            shadow.spread = spread;
        }

        shadow.fill = Fill::from_parser(parser)?;

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
