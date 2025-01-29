use torin::prelude::{
    Length,
    VisibleSize,
};

use crate::{
    Parse,
    ParseError,
};

impl Parse for VisibleSize {
    fn parse(value: &str) -> Result<Self, ParseError> {
        if value.contains('%') {
            Ok(VisibleSize::InnerPercentage(Length::new(
                value
                    .replace('%', "")
                    .parse::<f32>()
                    .map_err(|_| ParseError)?,
            )))
        } else {
            Err(ParseError)
        }
    }
}
