use torin::content::Content;

use crate::{
    Parse,
    ParseError,
};

impl Parse for Content {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "fit" => Content::Fit,
            _ => Content::Normal,
        })
    }
}
