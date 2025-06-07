use std::fmt;

use crate::parsing::{
    Parse,
    ParseError,
};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum OverflowMode {
    #[default]
    None,
    Clip,
}

impl Parse for OverflowMode {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "clip" => OverflowMode::Clip,
            _ => OverflowMode::None,
        })
    }
}

impl fmt::Display for OverflowMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            OverflowMode::Clip => "clip",
            OverflowMode::None => "none",
        })
    }
}
