use std::iter;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    Float(f32),
    Integer(i64),
    ParenOpen,
    ParenClose,
    Minus,
    Plus,
    Slash,
    Star,
    Pound,
    Percent,
    Comma,
}

impl Token {
    pub fn ident<T: Into<String>>(value: T) -> Self {
        Self::Ident(value.into())
    }

    pub fn is_ident(&self) -> bool {
        matches!(self, Token::Ident(_))
    }

    pub fn is_f32(&self) -> bool {
        matches!(self, Token::Float(_))
    }

    pub fn is_i64(&self) -> bool {
        matches!(self, Token::Integer(_))
    }

    pub fn is_i64_or_f32(&self) -> bool {
        matches!(self, Token::Integer(_) | Token::Float(_))
    }

    pub fn into_string(self) -> String {
        if let Token::Ident(value) = self {
            value
        } else {
            unreachable!()
        }
    }

    pub fn into_f32(self) -> f32 {
        if let Token::Float(value) = self {
            value
        } else if let Token::Integer(value) = self {
            value as f32
        } else {
            unreachable!()
        }
    }

    pub fn into_i64(self) -> i64 {
        if let Token::Integer(value) = self {
            value
        } else {
            unreachable!()
        }
    }

    pub fn as_str(&self) -> &str {
        if let Token::Ident(value) = self {
            value.as_str()
        } else {
            unreachable!()
        }
    }

    pub fn try_as_str(&self) -> Option<&str> {
        if let Token::Ident(value) = self {
            Some(value.as_str())
        } else {
            None
        }
    }

    pub fn try_as_f32(&self) -> Option<f32> {
        if let Token::Float(value) = self {
            Some(*value)
        } else if let Token::Integer(value) = self {
            Some(*value as f32)
        } else {
            None
        }
    }

    pub fn try_as_i64(&self) -> Option<i64> {
        if let Token::Integer(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn try_as_u8(&self) -> Option<u8> {
        if let Token::Integer(value) = self {
            u8::try_from(*value).ok()
        } else {
            None
        }
    }
}

pub struct Lexer;

impl Lexer {
    pub fn parse<T: AsRef<str>>(data: T) -> Vec<Token> {
        let mut tokens = vec![];
        let mut chars = data.as_ref().chars().peekable();

        while let Some(character) = chars.next() {
            match character {
                ' ' => continue,
                'A'..='z' => {
                    tokens.push(Token::Ident(
                        iter::once(character)
                            .chain(iter::from_fn(|| {
                                chars
                                    .by_ref()
                                    .next_if(|s| s.is_ascii_alphanumeric() || s == &'-')
                            }))
                            .collect::<String>()
                            .parse()
                            .unwrap(),
                    ));
                }
                '0'..='9' => {
                    let value = iter::once(character)
                        .chain(iter::from_fn(|| {
                            chars.by_ref().next_if(|s| s.is_ascii_digit() || s == &'.')
                        }))
                        .collect::<String>();

                    if value.contains('.') {
                        tokens.push(Token::Float(value.parse().unwrap()));
                    } else {
                        tokens.push(Token::Integer(value.parse().unwrap()));
                    }
                }
                '(' => tokens.push(Token::ParenOpen),
                ')' => tokens.push(Token::ParenClose),
                '+' => tokens.push(Token::Plus),
                '-' => {
                    if chars.peek().is_some_and(char::is_ascii_digit) {
                        let value = iter::once(character)
                            .chain(iter::from_fn(|| {
                                chars.by_ref().next_if(|s| s.is_ascii_digit() || s == &'.')
                            }))
                            .collect::<String>();

                        if value.contains('.') {
                            tokens.push(Token::Float(value.parse().unwrap()));
                        } else {
                            tokens.push(Token::Integer(value.parse().unwrap()));
                        }
                    } else {
                        tokens.push(Token::Minus);
                    }
                }
                '*' => tokens.push(Token::Star),
                '/' => tokens.push(Token::Slash),
                '#' => {
                    tokens.push(Token::Pound);

                    if chars.peek().is_some_and(char::is_ascii_alphanumeric) {
                        tokens.push(Token::Ident(
                            iter::from_fn(|| chars.by_ref().next_if(char::is_ascii_alphanumeric))
                                .collect::<String>(),
                        ));
                    }
                }
                '%' => tokens.push(Token::Percent),
                ',' => tokens.push(Token::Comma),
                character => {
                    if let Some(Token::Ident(data)) = tokens.last_mut() {
                        data.push(character);
                    } else {
                        tokens.push(Token::Ident(character.to_string()));
                    }
                }
            }
        }

        tokens
    }
}
