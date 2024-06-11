use torin::alignment::Alignment;

use crate::Parse;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseAlignmentError;

impl Parse for Alignment {
    type Err = ParseAlignmentError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "center" => Alignment::Center,
            "end" => Alignment::End,
            _ => Alignment::Start,
        })
    }
}
