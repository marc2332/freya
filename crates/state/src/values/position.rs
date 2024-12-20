use torin::position::Position;

use crate::{
    Parse,
    ParseError,
};

impl Parse for Position {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "absolute" => Position::new_absolute(),
            "fixed" => Position::new_fixed(),
            _ => Position::Stacked,
        })
    }
}
