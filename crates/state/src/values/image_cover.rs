use crate::{
    Parse,
    ParseError,
};

#[derive(Default, Clone, Debug, PartialEq)]
pub enum ImageCover {
    #[default]
    Fill,

    Center,
}

impl Parse for ImageCover {
    fn parse(value: &str) -> Result<Self, ParseError> {
        match value {
            "center" => Ok(ImageCover::Center),
            _ => Ok(ImageCover::Fill),
        }
    }
}
