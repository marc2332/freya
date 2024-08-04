use std::fmt;

use freya_engine::prelude::*;
use torin::{
    prelude::Measure,
    size::Rect,
};

use crate::{
    DisplayColor,
    Parse,
    ParseError,
    Parser,
    Token,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GradientStop {
    pub color: Color,
    pub offset: f32,
}

impl Parse for GradientStop {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        let color = Color::from_parser(parser)?;
        let offset = (parser.consume_map(Token::try_as_f32)? / 100.0).clamp(0.0, 1.0);

        parser.consume(&Token::Percent)?;

        Ok(GradientStop { color, offset })
    }
}

impl fmt::Display for GradientStop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        _ = self.color.fmt_rgb(f);
        write!(f, " {}%", self.offset * 100.0)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LinearGradient {
    pub stops: Vec<GradientStop>,
    pub angle: f32,
}

impl LinearGradient {
    pub fn into_shader(&self, bounds: Rect<f32, Measure>) -> Option<Shader> {
        let colors: Vec<Color> = self.stops.iter().map(|stop| stop.color).collect();
        let offsets: Vec<f32> = self.stops.iter().map(|stop| stop.offset).collect();

        let center = bounds.center();

        let matrix = Matrix::rotate_deg_pivot(self.angle, (center.x, center.y));

        Shader::linear_gradient(
            (
                (bounds.min_x(), bounds.min_y()),
                (bounds.max_x(), bounds.max_y()),
            ),
            GradientShaderColors::Colors(&colors[..]),
            Some(&offsets[..]),
            TileMode::Clamp,
            None,
            Some(&matrix),
        )
    }
}

impl Parse for LinearGradient {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume(&Token::ident("linear-gradient"))?;
        parser.consume(&Token::ParenOpen)?;

        let mut gradient = LinearGradient {
            angle: if let Ok(angle) = parser.consume_map(Token::try_as_i64).and_then(|value| {
                if (0..=360).contains(&value) {
                    Ok(value as f32)
                } else {
                    Err(ParseError)
                }
            }) {
                parser.consume(&Token::ident("deg"))?;
                parser.consume(&Token::Comma)?;

                angle
            } else {
                0.0
            },
            ..Default::default()
        };

        while !parser.check(&Token::ParenClose) {
            if !gradient.stops.is_empty() {
                parser.consume(&Token::Comma)?;
            }

            gradient.stops.push(GradientStop::from_parser(parser)?);
        }

        parser.consume(&Token::ParenClose)?;

        Ok(gradient)
    }
}

impl fmt::Display for LinearGradient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "linear-gradient({}deg, {})",
            self.angle,
            self.stops
                .iter()
                .map(|stop| stop.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RadialGradient {
    pub stops: Vec<GradientStop>,
}

impl RadialGradient {
    pub fn into_shader(&self, bounds: Rect<f32, Measure>) -> Option<Shader> {
        let colors: Vec<Color> = self.stops.iter().map(|stop| stop.color).collect();
        let offsets: Vec<f32> = self.stops.iter().map(|stop| stop.offset).collect();

        let center = bounds.center();

        Shader::radial_gradient(
            Point::new(center.x, center.y),
            bounds.width().max(bounds.height()),
            GradientShaderColors::Colors(&colors[..]),
            Some(&offsets[..]),
            TileMode::Clamp,
            None,
            None,
        )
    }
}

impl Parse for RadialGradient {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume(&Token::ident("radial-gradient"))?;
        parser.consume(&Token::ParenOpen)?;

        let mut gradient = RadialGradient::default();

        while !parser.check(&Token::ParenClose) {
            if !gradient.stops.is_empty() {
                parser.consume(&Token::Comma)?;
            }

            gradient.stops.push(GradientStop::from_parser(parser)?);
        }

        parser.consume(&Token::ParenClose)?;

        Ok(gradient)
    }
}

impl fmt::Display for RadialGradient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "radial-gradient({})",
            self.stops
                .iter()
                .map(|stop| stop.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConicGradient {
    pub stops: Vec<GradientStop>,
    pub angles: Option<(f32, f32)>,
    pub angle: Option<f32>,
}

impl ConicGradient {
    pub fn into_shader(&self, bounds: Rect<f32, Measure>) -> Option<Shader> {
        let colors: Vec<Color> = self.stops.iter().map(|stop| stop.color).collect();
        let offsets: Vec<f32> = self.stops.iter().map(|stop| stop.offset).collect();

        let center = bounds.center();

        let matrix =
            Matrix::rotate_deg_pivot(-90.0 + self.angle.unwrap_or(0.0), (center.x, center.y));

        Shader::sweep_gradient(
            (center.x, center.y),
            GradientShaderColors::Colors(&colors[..]),
            Some(&offsets[..]),
            TileMode::Clamp,
            self.angles,
            None,
            Some(&matrix),
        )
    }
}

impl Parse for ConicGradient {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume(&Token::ident("conic-gradient"))?;
        parser.consume(&Token::ParenOpen)?;

        let mut gradient = ConicGradient {
            angle: if let Ok(angle) = parser.consume_map(Token::try_as_i64).and_then(|value| {
                if (0..=360).contains(&value) {
                    Ok(value as f32)
                } else {
                    Err(ParseError)
                }
            }) {
                parser.consume(&Token::ident("deg"))?;
                parser.consume(&Token::Comma)?;

                Some(angle)
            } else {
                None
            },
            angles: if parser.try_consume(&Token::ident("from")) {
                let start = parser.consume_map(Token::try_as_i64).and_then(|value| {
                    if (0..=360).contains(&value) {
                        Ok(value as f32)
                    } else {
                        Err(ParseError)
                    }
                })?;

                parser.consume(&Token::ident("deg"))?;

                let end = if parser.try_consume(&Token::ident("to")) {
                    let result = parser.consume_map(Token::try_as_i64).and_then(|value| {
                        if (0..=360).contains(&value) {
                            Ok(value as f32)
                        } else {
                            Err(ParseError)
                        }
                    })?;

                    parser.consume(&Token::ident("deg"))?;

                    result
                } else {
                    360.0
                };

                parser.consume(&Token::Comma)?;

                Some((start, end))
            } else {
                None
            },
            ..Default::default()
        };

        while !parser.check(&Token::ParenClose) {
            if !gradient.stops.is_empty() {
                parser.consume(&Token::Comma)?;
            }

            gradient.stops.push(GradientStop::from_parser(parser)?);
        }

        parser.consume(&Token::ParenClose)?;

        Ok(gradient)
    }
}

impl fmt::Display for ConicGradient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "conic-gradient(")?;

        if let Some(angle) = self.angle {
            write!(f, "{angle}deg, ")?;
        }

        if let Some((start, end)) = self.angles {
            write!(f, "from {start}deg to {end}deg, ")?;
        }

        write!(
            f,
            "{})",
            self.stops
                .iter()
                .map(|stop| stop.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
