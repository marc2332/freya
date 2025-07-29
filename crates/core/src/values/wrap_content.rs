use torin::wrap_content::WrapContent;

use crate::parsing::{
    Parse,
    ParseError,
};

impl Parse for WrapContent {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "no-wrap" => WrapContent::NoWrap,
            "wrap" => WrapContent::Wrap,
            _ => WrapContent::NoWrap,
        })
    }
}
