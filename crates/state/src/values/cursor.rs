use std::fmt;

use crate::{
    Parse,
    ParseError,
    Parser,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CursorMode {
    None,
    Editable,
}

impl Parse for CursorMode {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume_map(|value| {
            value.try_as_str().and_then(|value| match value {
                "none" => Some(Self::None),
                "editable" => Some(Self::Editable),
                _ => None,
            })
        })
    }
}

impl fmt::Display for CursorMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            CursorMode::Editable => "editable",
            CursorMode::None => "none",
        })
    }
}
