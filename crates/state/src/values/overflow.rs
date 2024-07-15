use std::fmt;

use crate::{
    Parse,
    ParseError,
    Parser,
};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum OverflowMode {
    #[default]
    None,
    Clip,
}

impl Parse for OverflowMode {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|value| {
            value.try_as_str().and_then(|value| match value {
                "clip" => Some(Self::Clip),
                "none" => Some(Self::None),
                _ => None,
            })
        })
    }
}

impl fmt::Display for OverflowMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Clip => "clip",
            Self::None => "none",
        })
    }
}
