use crate::Parse;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CursorMode {
    None,
    Editable,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCursorError;

impl Parse for CursorMode {
    type Err = ParseCursorError;

    fn parse(value: &str, _scale_factor: Option<f32>) -> Result<Self, Self::Err> {
        Ok(match value {
            "editable" => CursorMode::Editable,
            _ => CursorMode::None,
        })
    }
}
