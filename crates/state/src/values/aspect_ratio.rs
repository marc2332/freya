use crate::{
    Parse,
    ParseError,
};

#[derive(Default, Clone, Debug, PartialEq)]
pub enum AspectRatio {
    Min,
    Max,
    #[default]
    None,
}

impl Parse for AspectRatio {
    fn parse(value: &str) -> Result<Self, ParseError> {
        match value {
            "min" => Ok(Self::Min),
            "max" => Ok(Self::Max),
            "none" => Ok(Self::None),
            _ => Err(ParseError),
        }
    }
}
