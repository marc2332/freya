use crate::{LinearGradient, Parse};
use skia_safe::Color;

#[derive(Clone, Debug, PartialEq)]
pub enum Fill {
    Color(Color),
    LinearGradient(LinearGradient),
    // RadialGradient(RadialGradient),
    // ConicGradient(ConicGradient),
}

impl Default for Fill {
    fn default() -> Self {
        Self::Color(Color::default())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseFillError;

impl Parse for Fill {
    type Err = ParseFillError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        Ok(if value.starts_with("linear-gradient(") {
            Self::LinearGradient(LinearGradient::parse(value).map_err(|_| ParseFillError)?)
        } else {
            Self::Color(Color::parse(value).map_err(|_| ParseFillError)?)
        })
    }
}

#[test]
fn test() {
    println!("{:?}", Fill::parse("linear-gradient(red, blue, green)"));
    println!("{:?}", Fill::parse("red"));
}
