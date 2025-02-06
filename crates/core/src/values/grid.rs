use torin::prelude::{
    GridSize,
    Length,
};

use crate::parsing::{
    Parse,
    ParseError,
};

impl Parse for GridSize {
    fn parse(value: &str) -> Result<Self, ParseError> {
        if value == "auto" {
            Ok(Self::Inner)
        } else if value.contains('*') {
            Ok(Self::Stars(Length::new(
                value.replace('*', "").parse().map_err(|_| ParseError)?,
            )))
        } else {
            Ok(Self::Pixels(Length::new(
                value.parse().map_err(|_| ParseError)?,
            )))
        }
    }
}
