use crate::parsing::Parse;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayerMode {
    Inherited,
    Relative(i16),
    Overlay,
}

impl Parse for LayerMode {
    fn parse(value: &str) -> Result<Self, crate::parsing::ParseError> {
        Ok(match value {
            "overlay" => Self::Overlay,
            str => {
                if let Ok(relative) = str.parse::<i16>() {
                    Self::Relative(relative)
                } else {
                    Self::Inherited
                }
            }
        })
    }
}
