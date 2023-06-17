use skia_safe::Color;
use crate::Parse;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseColorError;

impl Parse for Color {
    type Err = ParseColorError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        match value {
            "red" => Ok(Color::RED),
            "green" => Ok(Color::GREEN),
            "blue" => Ok(Color::BLUE),
            "yellow" => Ok(Color::YELLOW),
            "black" => Ok(Color::BLACK),
            "gray" => Ok(Color::GRAY),
            "white" => Ok(Color::WHITE),
            "orange" => Ok(Color::from_rgb(255, 165, 0)),
            "transparent" => Ok(Color::TRANSPARENT),
            _ => parse_rgb(value)
        }
    }
}

fn parse_rgb(color: &str) -> Result<Color, ParseColorError> {
    let color = color.replace("rgb(", "").replace(')', "");
    let mut colors = color.split(',');

    let r = colors.next().ok_or(ParseColorError)?.trim().parse::<u8>().map_err(|_| ParseColorError)?;
    let g = colors.next().ok_or(ParseColorError)?.trim().parse::<u8>().map_err(|_| ParseColorError)?;
    let b = colors.next().ok_or(ParseColorError)?.trim().parse::<u8>().map_err(|_| ParseColorError)?;
    let a: Option<&str> = colors.next();

    if let Some(a) = a {
        let a = a.trim().parse::<u8>().map_err(|_| ParseColorError)?;
        Ok(Color::from_argb(a, r, g, b))
    } else {
        Ok(Color::from_rgb(r, g, b))
    }
}