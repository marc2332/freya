use torin::position::Position;

use crate::{
    Parse,
    ParseError,
};

impl Parse for Position {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "absolute" => Position::new_absolute(),
            _ => Position::Stacked,
        })
    }
}
