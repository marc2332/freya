use torin::{
    geometry::Length,
    size::{
        DynamicCalculation,
        Size,
    },
};

use crate::{
    parse_func,
    Parse,
    ParseError,
    Parser,
    Token,
};

impl Parse for Size {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        if parser.check(&Token::ident("calc")) {
            parse_calc(parser).map(|value| Self::DynamicCalculations(Box::new(value)))
        } else if let Ok(value) = parser.consume_if(Token::is_ident).map(Token::into_string) {
            match value.as_str() {
                "auto" => Ok(Self::Inner),
                "fill" => Ok(Self::Fill),
                "fill-min" => Ok(Self::FillMinimum),
                value => Err(ParseError::invalid_ident(
                    value,
                    &["auto", "fill", "fill-min"],
                )),
            }
        } else {
            let value = parser.consume_map(Token::try_as_f32)?;

            Ok(if parser.try_consume(&Token::Percent) {
                Size::Percentage(Length::new(value))
            } else if parser.try_consume(&Token::ident("v")) {
                Size::RootPercentage(Length::new(value))
            } else if parser.try_consume(&Token::ident("a")) {
                Size::InnerPercentage(Length::new(value))
            } else {
                Size::Pixels(Length::new(value))
            })
        }
    }
}

impl Parse for DynamicCalculation {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        if let Ok(value) = parser.consume_map(Token::try_as_f32) {
            Ok(if parser.try_consume(&Token::Percent) {
                DynamicCalculation::Percentage(value)
            } else if parser.try_consume(&Token::ident("v")) {
                DynamicCalculation::RootPercentage(value)
            } else {
                DynamicCalculation::Pixels(value)
            })
        } else {
            parser.consume_map(|token| match token {
                Token::Plus => Some(Self::Add),
                Token::Minus => Some(Self::Sub),
                Token::Slash => Some(Self::Div),
                Token::Star => Some(Self::Mul),
                _ => None,
            })
        }
    }
}

pub fn parse_calc(parser: &mut Parser) -> Result<Vec<DynamicCalculation>, ParseError> {
    parse_func(parser, "calc", |parser| {
        let mut calcs = Vec::new();

        while let Ok(value) = DynamicCalculation::from_parser(parser) {
            calcs.push(value);
        }

        Ok(calcs)
    })
}
