use crate::{
    Parse,
    ParseError,
    Parser,
};

#[derive(Clone, Debug, PartialEq, Default, Eq)]
pub enum Focusable {
    #[default]
    Unknown,
    Disabled,
    Enabled,
}

impl Focusable {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled)
    }
}

impl Parse for Focusable {
    fn from_parser(_: &mut Parser) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "true" => Self::Enabled,
            "false" => Self::Disabled,
            _ => Self::Unknown,
        })
    }
}
