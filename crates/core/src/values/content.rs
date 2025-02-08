use torin::content::Content;

use crate::parsing::{
    Parse,
    ParseError,
};

impl Parse for Content {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "fit" => Content::Fit,
            "flex" => Content::Flex,
            "grid" => Content::Grid {
                columns: Vec::new(),
                rows: Vec::new(),
            },
            _ => Content::Normal,
        })
    }
}
