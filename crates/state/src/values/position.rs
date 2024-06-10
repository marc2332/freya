use torin::position::Position;

use crate::Parse;

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePositionError;

impl Parse for Position {
    type Err = ParsePositionError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "absolute" => Position::new_absolute(),
            _ => Position::Stacked,
        })
    }
}
