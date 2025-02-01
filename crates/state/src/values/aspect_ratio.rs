use crate::{
    Parse,
    ParseError,
};

#[derive(Default, Clone, Debug, PartialEq)]
pub enum AspectRatio {
    #[default]
    Min,
    Max,
    Fit,
    None,
}

impl Parse for AspectRatio {
    fn parse(value: &str) -> Result<Self, ParseError> {
        match value {
            "min" => Ok(Self::Min),
            "max" => Ok(Self::Max),
            "fit" => Ok(Self::Fit),
            "none" => Ok(Self::None),
            _ => Err(ParseError),
        }
    }
}
