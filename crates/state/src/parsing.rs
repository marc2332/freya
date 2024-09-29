use std::{
    iter::Peekable,
    ops::{
        Bound,
        RangeBounds,
    },
    vec::IntoIter,
};

use freya_native_core::prelude::OwnedAttributeView;

use crate::{
    CustomAttributeValues,
    Lexer,
    Token,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ParseError(pub String);

impl ParseError {
    pub fn expected_token(expected: &Token, found: Option<&Token>) -> Self {
        if let Some(found) = found {
            Self(format!("expected {expected}, found {found}"))
        } else {
            Self(format!("expected {expected}, found nothing"))
        }
    }

    pub fn unexpected_token(value: Option<&Token>) -> Self {
        if let Some(value) = value {
            Self(format!("unexpected {value}"))
        } else {
            Self("unexpected nothing".into())
        }
    }

    pub fn invalid_ident(value: &str, expected: &[&str]) -> Self {
        let expected = match expected.len() {
            0 => "nothing".into(),
            1 => expected[0].into(),
            size => {
                let other = expected[0..(size - 1)].join(", ");
                let last = expected[size - 1];

                format!("{other} or {last}")
            }
        };

        Self(format!("invalid ident {value} (expected {expected})"))
    }

    pub fn too_much_tokens(count: usize) -> Self {
        Self(format!(
            "found more than zero ({count}) tokens after parsing"
        ))
    }
}

pub struct Parser {
    pub(crate) tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    /// Consumes the current token only if it exists and is equal to `value`.
    pub fn try_consume(&mut self, value: &Token) -> bool {
        if self.peek().is_some_and(|v| v == value) {
            self.next();

            true
        } else {
            false
        }
    }

    /// Checks if the next token exists and it is equal to `value`.
    pub fn check(&mut self, value: &Token) -> bool {
        self.peek().is_some_and(|v| v == value)
    }

    /// Returns the `bool` result of `func` if the next token exists.
    pub fn check_if<F: Fn(&Token) -> bool>(&mut self, func: F) -> bool {
        self.peek().is_some_and(func)
    }

    /// Consumes the current token if it exists and is equal to `value`, otherwise returning `ParseError`.
    pub fn consume(&mut self, value: &Token) -> Result<Token, ParseError> {
        if self.check(value) {
            Ok(self.next().unwrap())
        } else {
            Err(ParseError::expected_token(value, self.peek()))
        }
    }

    /// Consumes the current token if it exists and the result of `func` is `true`, otherwise returning `ParseError`.
    pub fn consume_if<F: Fn(&Token) -> bool>(&mut self, func: F) -> Result<Token, ParseError> {
        if self.check_if(func) {
            Ok(self.next().unwrap())
        } else {
            Err(ParseError::unexpected_token(self.peek()))
        }
    }

    /// Consumes the current token if it exists and the result of the `func` is `Some(T)`, otherwise returning `ParseError`.
    pub fn consume_map<T, F: Fn(&Token) -> Option<T>>(&mut self, func: F) -> Result<T, ParseError> {
        if let Some(value) = self.peek().and_then(func) {
            self.next();

            Ok(value)
        } else {
            Err(ParseError::unexpected_token(self.peek()))
        }
    }

    /// Consumes the current token and returns it wrapped in `Some` if it exists, otherwise returning `None`.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    /// Peeks the current token and returns a reference to it wrapped in `Some` if it exists, otherwise returning `None`.
    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
}

// FromStr but we own it so we can impl it on torin and skia_safe types.
pub trait Parse: Sized {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError>;
    fn from_parser_multiple(
        parser: &mut Parser,
        separator: &Token,
    ) -> Result<Vec<Self>, ParseError> {
        let mut values = vec![Self::from_parser(parser)?];

        while parser.try_consume(separator) {
            values.push(Self::from_parser(parser)?);
        }

        Ok(values)
    }

    fn parse_with_separator(value: &str, separator: &Token) -> Result<Vec<Self>, ParseError> {
        let mut parser = Parser::new(Lexer::parse(value));

        let values = Self::from_parser_multiple(&mut parser, separator)?;

        if parser.tokens.len() > 0 {
            Err(ParseError::too_much_tokens(parser.tokens.len()))
        } else {
            Ok(values)
        }
    }

    fn parse(value: &str) -> Result<Self, ParseError> {
        let mut parser = Parser::new(Lexer::parse(value));

        let value = Self::from_parser(&mut parser);

        if parser.tokens.len() > 0 {
            Err(ParseError::too_much_tokens(parser.tokens.len()))
        } else {
            value
        }
    }
}

pub fn parse_angle(parser: &mut Parser) -> Result<f32, ParseError> {
    let value = parser.consume_map(Token::try_as_i64)?;

    parser.consume(&Token::ident("deg"))?;

    Ok((value % 360) as f32)
}

pub fn parse_range(parser: &mut Parser, range: impl RangeBounds<i64>) -> Result<f32, ParseError> {
    let value = parser.consume_map(Token::try_as_i64)?;

    if range.contains(&value) {
        Ok(value as f32)
    } else {
        let start = match range.start_bound() {
            Bound::Included(value) => Some(format!("greater than or equal to {value}")),
            _ => None,
        };

        let end = match range.end_bound() {
            Bound::Included(value) => Some(format!("less than or equal to {value}")),
            Bound::Excluded(value) => Some(format!("less than {value}")),
            Bound::Unbounded => None,
        };

        Err(match [start, end] {
            [Some(start), Some(end)] => ParseError(format!("{value} must be {start} and {end}")),
            [Some(start), None] => ParseError(format!("{value} must be {start}")),
            [None, Some(end)] => ParseError(format!("{value} must be {end}")),
            [None, None] => unreachable!(),
        })
    }
}

pub fn parse_func<T: AsRef<str>, F: FnOnce(&mut Parser) -> Result<O, ParseError>, O>(
    parser: &mut Parser,
    name: T,
    body: F,
) -> Result<O, ParseError> {
    parser.consume(&Token::ident(name.as_ref()))?;

    parser.consume(&Token::ParenOpen)?;

    let value = body(parser)?;

    parser.consume(&Token::ParenClose)?;

    Ok(value)
}

pub trait ParseAttribute: Sized {
    fn parse_attribute(
        &mut self,
        attr: OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), ParseError>;

    fn parse_safe(&mut self, attr: OwnedAttributeView<CustomAttributeValues>) {
        #[cfg(debug_assertions)]
        {
            let error_attr = attr.clone();

            if let Err(ParseError(message)) = self.parse_attribute(attr) {
                panic!(
                    "Failed to parse attribute '{:?}' with value '{:?}': {message}",
                    error_attr.attribute, error_attr.value
                );
            }
        }

        #[cfg(not(debug_assertions))]
        self.parse_attribute(attr).ok();
    }
}
