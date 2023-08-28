use crate::Parse;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum OverflowMode {
    #[default]
    None,
    Clip,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseOverflowError;

impl Parse for OverflowMode {
    type Err = ();

    fn parse(value: &str) -> Result<Self, Self::Err> {
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
