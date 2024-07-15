use std::{
    iter::Peekable,
    str::CharIndices,
    vec::IntoIter,
};

use freya_native_core::prelude::OwnedAttributeView;

use crate::{
    CustomAttributeValues,
    Lexer,
    Token,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ParseError;

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
            Err(ParseError)
        }
    }

    /// Consumes the current token if it exists and is equal to one of the values inside `values`, otherwise returning `ParseError`.
    pub fn consume_one_of(&mut self, values: &[Token]) -> Result<Token, ParseError> {
        if self.check_if(|value| values.contains(value)) {
            Ok(self.next().unwrap())
        } else {
            Err(ParseError)
        }
    }

    /// Consumes the current token if it exists and the result of `func` is `true`, otherwise returning `ParseError`.
    pub fn consume_if<F: Fn(&Token) -> bool>(&mut self, func: F) -> Result<Token, ParseError> {
        if self.check_if(func) {
            Ok(self.next().unwrap())
        } else {
            Err(ParseError)
        }
    }

    /// Consumes the current token if it exists and the result of the `func` is `Some(T)`, otherwise returning `ParseError`.
    pub fn consume_map<T, F: Fn(&Token) -> Option<T>>(&mut self, func: F) -> Result<T, ParseError> {
        if let Some(value) = self.peek().and_then(func) {
            self.next();

            Ok(value)
        } else {
            Err(ParseError)
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

    /// Consumes the current token and returns it wrapped in `Some` if the result of the `func` function is `true`, otherwise returning `None`.
    pub fn next_if<F: Fn(&Token) -> bool>(&mut self, func: F) -> Option<Token> {
        if self.check_if(func) {
            self.next()
        } else {
            None
        }
    }
}

// FromStr but we own it so we can impl it on torin and skia_safe types.
pub trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError>;

    fn parse_value(value: &str) -> Result<Self, ParseError> {
        let mut parser = Parser::new(Lexer::parse(value));

        let value = Self::parse(&mut parser);

        if parser.tokens.len() > 0 {
            Err(ParseError)
        } else {
            value
        }
    }

    fn parse_values(value: &str, separator: &Token) -> Result<Vec<Self>, ParseError> {
        let mut parser = Parser::new(Lexer::parse(value));

        let mut values = vec![Self::parse(&mut parser)?];

        while parser.try_consume(separator) {
            values.push(Self::parse(&mut parser)?);
        }

        if parser.tokens.len() > 0 {
            Err(ParseError)
        } else {
            Ok(values)
        }
    }
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

            if self.parse_attribute(attr).is_err() {
                panic!(
                    "Failed to parse attribute '{:?}' with value '{:?}'",
                    error_attr.attribute, error_attr.value,
                );
            }
        }

        #[cfg(not(debug_assertions))]
        self.parse_attribute(attr).ok()
    }
}

pub trait ExtSplit {
    fn split_excluding_group(
        &self,
        delimiter: char,
        group_start: char,
        group_end: char,
    ) -> SplitExcludingGroup<'_>;
    fn split_ascii_whitespace_excluding_group(
        &self,
        group_start: char,
        group_end: char,
    ) -> SplitAsciiWhitespaceExcludingGroup<'_>;
}

impl ExtSplit for str {
    fn split_excluding_group(
        &self,
        delimiter: char,
        group_start: char,
        group_end: char,
    ) -> SplitExcludingGroup<'_> {
        SplitExcludingGroup {
            text: self,
            chars: self.char_indices(),
            delimiter,
            group_start,
            group_end,
            trailing_empty: true,
        }
    }

    fn split_ascii_whitespace_excluding_group(
        &self,
        group_start: char,
        group_end: char,
    ) -> SplitAsciiWhitespaceExcludingGroup<'_> {
        SplitAsciiWhitespaceExcludingGroup {
            text: self,
            chars: self.char_indices(),
            group_start,
            group_end,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SplitExcludingGroup<'a> {
    pub text: &'a str,
    pub chars: CharIndices<'a>,
    pub delimiter: char,
    pub group_start: char,
    pub group_end: char,
    pub trailing_empty: bool,
}

impl<'a> Iterator for SplitExcludingGroup<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let first = self.chars.next();

        let (start, mut prev) = match first {
            None => {
                if self.text.ends_with(self.delimiter) && self.trailing_empty {
                    self.trailing_empty = false;
                    return Some("");
                }
                return None;
            }
            Some((_, c)) if c == self.delimiter => return Some(""),
            Some(v) => v,
        };

        let mut in_group = false;
        let mut nesting = -1;

        loop {
            if prev == self.group_start {
                if nesting == -1 {
                    in_group = true;
                }
                nesting += 1;
            } else if prev == self.group_end {
                nesting -= 1;
                if nesting == -1 {
                    in_group = false;
                }
            }

            prev = match self.chars.next() {
                None => return Some(&self.text[start..]),
                Some((end, c)) if c == self.delimiter && !in_group => {
                    return Some(&self.text[start..end])
                }
                Some((_, c)) => c,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct SplitAsciiWhitespaceExcludingGroup<'a> {
    pub text: &'a str,
    pub chars: CharIndices<'a>,
    pub group_start: char,
    pub group_end: char,
}

impl<'a> Iterator for SplitAsciiWhitespaceExcludingGroup<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let first = self.chars.next();

        let (start, mut prev) = match first {
            None => return None,
            Some((_, c)) if c.is_ascii_whitespace() => return self.next(),
            Some(v) => v,
        };

        let mut in_group = false;
        let mut nesting = -1;

        loop {
            if prev == self.group_start {
                if nesting == -1 {
                    in_group = true;
                }
                nesting += 1;
            } else if prev == self.group_end {
                nesting -= 1;
                if nesting == -1 {
                    in_group = false;
                }
            }

            prev = match self.chars.next() {
                None => return Some(&self.text[start..]),
                Some((end, c)) if c.is_ascii_whitespace() && !in_group => {
                    return Some(&self.text[start..end])
                }
                Some((_, c)) => c,
            }
        }
    }
}
