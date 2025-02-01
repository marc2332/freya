use crate::{
    Parse,
    ParseError,
};

#[derive(Default, Clone, Debug, PartialEq)]
pub enum AspectRatio {
    Min,
    Max,
    #[default]
    Auto,
    None,
}

impl Parse for AspectRatio {
    fn parse(value: &str) -> Result<Self, ParseError> {
        match value {
            "min" => Ok(Self::Min),
            "max" => Ok(Self::Max),
            "auto" => Ok(Self::Auto),
            "none" => Ok(Self::None),
            _ => Err(ParseError),
        }
    }
}
