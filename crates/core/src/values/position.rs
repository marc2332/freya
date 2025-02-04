use torin::position::Position;

use crate::parsing::{
    Parse,
    ParseError,
};

impl Parse for Position {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "absolute" => Position::new_absolute(),
            "global" => Position::new_global(),
            _ => Position::Stacked,
        })
    }
}
