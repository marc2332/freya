use torin::content::Content;

use crate::Parse;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseContentError;

impl Parse for Content {
    type Err = ParseContentError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "fit" => Content::Fit,
            _ => Content::Normal,
        })
    }
}
