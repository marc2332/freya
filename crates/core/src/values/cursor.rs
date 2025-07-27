use std::fmt;

use crate::parsing::{
    Parse,
    ParseError,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CursorMode {
    None,
    Editable,
}

impl Parse for CursorMode {
    fn parse(value: &str) -> Result<Self, ParseError> {
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
