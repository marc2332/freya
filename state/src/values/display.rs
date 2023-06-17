use torin::display::DisplayMode;
use crate::Parse;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDisplayModeError;

impl Parse for DisplayMode {
    type Err = ParseDisplayModeError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
		Ok(match value {
			"center" => DisplayMode::Center,
			_ => DisplayMode::Normal,
		})
    }
}