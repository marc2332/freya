use crate::Parse;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CursorMode {
    None,
    Editable,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCursorError;

impl Parse for CursorMode {
    type Err = ParseCursorError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "editable" => CursorMode::Editable,
            _ => CursorMode::None,
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
