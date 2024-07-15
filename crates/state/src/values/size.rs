use torin::{
    geometry::Length,
    size::{
        DynamicCalculation,
        Size,
    },
};

use crate::{
    Parse,
    ParseError,
    Parser,
    Token,
};

impl Parse for Size {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let value = parser.consume_if(|token| token.is_ident() || token.is_i64_or_f32())?;

        if value.is_ident() {
            value
                .try_as_str()
                .and_then(|value| match value {
                    "auto" => Some(Self::Inner),
                    "fill" => Some(Self::Fill),
                    "fill-min" => Some(Self::FillMinimum),
                    "calc" => parse_calc(parser)
                        .map(|value| Self::DynamicCalculations(Box::new(value)))
                        .ok(),
                    _ => None,
                })
                .ok_or(ParseError)
        } else {
            let value = value.into_f32();

            Ok(if parser.try_consume(&Token::Percent) {
                Size::Percentage(Length::new(value))
            } else if parser.try_consume(&Token::ident("v")) {
                Size::RootPercentage(Length::new(value))
            } else {
                Size::Pixels(Length::new(value))
            })
        }
    }
}

pub fn parse_calc(parser: &mut Parser) -> Result<Vec<DynamicCalculation>, ParseError> {
    parser.consume(&Token::ParenOpen)?;

    let mut calcs = vec![];

    while let Ok(value) = parser.consume_if(|token| {
        token.is_i64_or_f32()
            || matches!(
                token,
                Token::Plus | Token::Minus | Token::Slash | Token::Star
            )
    }) {
        if value.is_i64_or_f32() {
            let value = value.into_f32();

            calcs.push(if parser.try_consume(&Token::Percent) {
                DynamicCalculation::Percentage(value)
            } else {
                DynamicCalculation::Pixels(value)
            });
        } else {
            match value {
                Token::Plus => calcs.push(DynamicCalculation::Add),
                Token::Minus => calcs.push(DynamicCalculation::Sub),
                Token::Slash => calcs.push(DynamicCalculation::Div),
                Token::Star => calcs.push(DynamicCalculation::Mul),
                _ => {}
            }
        }
    }

    parser.consume(&Token::ParenClose)?;

    Ok(calcs)
}
