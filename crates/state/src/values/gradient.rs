use std::{
    f32::consts::FRAC_PI_2,
    fmt,
};

use freya_engine::prelude::*;
use torin::{
    prelude::Measure,
    size::Rect,
};

use crate::{
    parse_angle,
    parse_func,
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

        let (dy, dx) = (self.angle.to_radians() + FRAC_PI_2).sin_cos();
        let farthest_corner = Point::new(
            if dx > 0.0 { bounds.width() } else { 0.0 },
            if dy > 0.0 { bounds.height() } else { 0.0 },
        );
        let delta = farthest_corner - Point::new(bounds.width(), bounds.height()) / 2.0;
        let u = delta.x * dy - delta.y * dx;
        let endpoint = farthest_corner + Point::new(-u * dy, u * dx);

        let origin = Point::new(bounds.min_x(), bounds.min_y());
        Shader::linear_gradient(
            (
                Point::new(bounds.width(), bounds.height()) - endpoint + origin,
                endpoint + origin,
            ),
            GradientShaderColors::Colors(&colors[..]),
            Some(&offsets[..]),
            TileMode::Clamp,
            None,
            None,
        )
    }
}

impl Parse for LinearGradient {
    fn from_parser(parser: &mut Parser) -> Result<Self, ParseError> {
        parse_func(parser, "linear-gradient", |parser| {
            let mut gradient = Self::default();

            if let Ok(angle) = parse_angle(parser) {
                parser.consume(&Token::Comma)?;

                gradient.angle = angle.to_radians();
            }

            gradient.stops = GradientStop::from_parser_multiple(parser, &Token::Comma)?;

            Ok(gradient)
        })
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
        parse_func(parser, "radial-gradient", |parser| {
            GradientStop::from_parser_multiple(parser, &Token::Comma).map(|stops| Self { stops })
        })
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
        parse_func(parser, "conic-gradient", |parser| {
            let mut gradient = Self::default();

            if let Ok(angle) = parse_angle(parser) {
                gradient.angle = Some(angle.to_radians());

                parser.consume(&Token::Comma)?;
            }

            if parser.try_consume(&Token::ident("from")) {
                let start = parse_angle(parser)?;

                let end = if parser.try_consume(&Token::ident("to")) {
                    parse_angle(parser)?
                } else {
                    360.0
                };

                gradient.angles = Some((start, end));

                parser.consume(&Token::Comma)?;
            }

            gradient.stops = GradientStop::from_parser_multiple(parser, &Token::Comma)?;

            Ok(gradient)
        })
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
