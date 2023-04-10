use skia_safe::Color;

mod cursor;
mod custom_attributes;
mod font_style;
mod references;
mod scroll;
mod size;
mod style;
mod transform;

pub use cursor::*;
pub use custom_attributes::*;
pub use font_style::*;
pub use references::*;
pub use scroll::*;
pub use size::*;
pub use style::*;
pub use transform::*;

pub enum AttributeType<'a> {
    Color(&'a Color),
    Size(&'a SizeMode),
    Measure(f32),
    Measures((f32, f32, f32, f32)),
    Direction(&'a DirectionMode),
    Display(&'a DisplayMode),
    Shadow(&'a ShadowSettings),
    Text(String),
}

pub fn parse_rgb(color: &str) -> Option<Color> {
    let color = color.replace("rgb(", "").replace(')', "");
    let mut colors = color.split(',');

    let r = colors.next()?.trim().parse().ok()?;
    let g = colors.next()?.trim().parse().ok()?;
    let b = colors.next()?.trim().parse().ok()?;
    let a: Option<&str> = colors.next();
    if let Some(a) = a {
        let a = a.trim().parse::<u8>().ok()?;
        Some(Color::from_argb(a, r, g, b))
    } else {
        Some(Color::from_rgb(r, g, b))
    }
}

pub fn parse_color(color: &str) -> Option<Color> {
    match color {
        "inherit" => None,
        "red" => Some(Color::RED),
        "green" => Some(Color::GREEN),
        "blue" => Some(Color::BLUE),
        "yellow" => Some(Color::YELLOW),
        "black" => Some(Color::BLACK),
        "gray" => Some(Color::GRAY),
        "white" => Some(Color::WHITE),
        "orange" => Some(Color::from_rgb(255, 165, 0)),
        _ => parse_rgb(color),
    }
}
