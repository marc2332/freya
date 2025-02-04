use std::fmt;

use freya_engine::prelude::Color;
use torin::scaled::Scaled;

use super::Fill;
use crate::parsing::{
    ExtSplit,
    Parse,
    ParseError,
};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Border {
    pub fill: Fill,
    pub width: BorderWidth,
    pub alignment: BorderAlignment,
}

impl Border {
    #[inline]
    pub fn is_visible(&self) -> bool {
        !(self.width.top == 0.0
            && self.width.left == 0.0
            && self.width.bottom == 0.0
            && self.width.right == 0.0)
            && self.fill != Fill::Color(Color::TRANSPARENT)
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct BorderWidth {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Scaled for BorderWidth {
    fn scale(&mut self, scale_factor: f32) {
        self.top *= scale_factor;
        self.left *= scale_factor;
        self.bottom *= scale_factor;
        self.right *= scale_factor;
    }
}

impl fmt::Display for BorderWidth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.top, self.right, self.bottom, self.left,
        )
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum BorderAlignment {
    #[default]
    Inner,
    Outer,
    Center,
}

impl Parse for BorderAlignment {
    fn parse(value: &str) -> Result<Self, ParseError> {
        Ok(match value {
            "inner" => BorderAlignment::Inner,
            "outer" => BorderAlignment::Outer,
            "center" => BorderAlignment::Center,
            _ => BorderAlignment::default(),
        })
    }
}

impl fmt::Display for BorderAlignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            BorderAlignment::Inner => "inner",
            BorderAlignment::Outer => "outer",
            BorderAlignment::Center => "center",
        })
    }
}

impl Parse for Border {
    fn parse(value: &str) -> Result<Self, ParseError> {
        if value == "none" {
            return Ok(Self::default());
        }

        let mut border_values = value.split_ascii_whitespace_excluding_group('(', ')');

        Ok(match border_values.clone().count() {
            // <width> <style> <fill>
            3 => {
                let width = border_values
                    .next()
                    .ok_or(ParseError)?
                    .parse::<f32>()
                    .map_err(|_| ParseError)?;

                Border {
                    width: BorderWidth {
                        top: width,
                        left: width,
                        bottom: width,
                        right: width,
                    },
                    alignment: BorderAlignment::parse(border_values.next().ok_or(ParseError)?)?,
                    fill: Fill::parse(&border_values.collect::<Vec<&str>>().join(" "))
                        .map_err(|_| ParseError)?,
                }
            }

            // <vertical> <horizontal> <solid> <fill>
            4 => {
                let vertical_width = border_values
                    .next()
                    .ok_or(ParseError)?
                    .parse::<f32>()
                    .map_err(|_| ParseError)?;
                let horizontal_width = border_values
                    .next()
                    .ok_or(ParseError)?
                    .parse::<f32>()
                    .map_err(|_| ParseError)?;

                Border {
                    width: BorderWidth {
                        top: vertical_width,
                        left: horizontal_width,
                        bottom: vertical_width,
                        right: horizontal_width,
                    },
                    alignment: BorderAlignment::parse(border_values.next().ok_or(ParseError)?)?,
                    fill: Fill::parse(&border_values.collect::<Vec<&str>>().join(" "))
                        .map_err(|_| ParseError)?,
                }
            }
            // <top> <horizontal> <bottom> <style> <fill>
            5 => {
                let top_width = border_values
                    .next()
                    .ok_or(ParseError)?
                    .parse::<f32>()
                    .map_err(|_| ParseError)?;
                let horizontal_width = border_values
                    .next()
                    .ok_or(ParseError)?
                    .parse::<f32>()
                    .map_err(|_| ParseError)?;
                let bottom_width = border_values
                    .next()
                    .ok_or(ParseError)?
                    .parse::<f32>()
                    .map_err(|_| ParseError)?;

                Border {
                    width: BorderWidth {
                        top: top_width,
                        left: horizontal_width,
                        bottom: bottom_width,
                        right: horizontal_width,
                    },
                    alignment: BorderAlignment::parse(border_values.next().ok_or(ParseError)?)?,
                    fill: Fill::parse(&border_values.collect::<Vec<&str>>().join(" "))
                        .map_err(|_| ParseError)?,
                }
            }
            // <top> <right> <bottom> <left> <style> <fill>
            6 => Border {
                width: BorderWidth {
                    top: border_values
                        .next()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                    right: border_values
                        .next()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                    bottom: border_values
                        .next()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                    left: border_values
                        .next()
                        .ok_or(ParseError)?
                        .parse::<f32>()
                        .map_err(|_| ParseError)?,
                },
                alignment: BorderAlignment::parse(border_values.next().ok_or(ParseError)?)?,
                fill: Fill::parse(&border_values.collect::<Vec<&str>>().join(" "))
                    .map_err(|_| ParseError)?,
            },
            _ => return Err(ParseError),
        })
    }
}

impl fmt::Display for Border {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.width, self.alignment, self.fill,)
    }
}

impl Scaled for Border {
    fn scale(&mut self, scale_factor: f32) {
        self.width.scale(scale_factor);
    }
}
