use torin::alignment::Alignment;

use crate::{
    Parse,
    ParseError,
};

impl Parse for Alignment {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "center" => Alignment::Center,
            "end" => Alignment::End,
            "space-between" => Alignment::SpaceBetween,
            "space-evenly" => Alignment::SpaceEvenly,
            "space-around" => Alignment::SpaceAround,
            _ => Alignment::Start,
        })
    }
}
