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
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let color = Color::parse(parser)?;
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

        let (dx, dy) = (-self.angle).sin_cos();
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
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.consume(&Token::ident("linear-gradient"))?;
        parser.consume(&Token::ParenOpen)?;

        let mut gradient = LinearGradient {
            angle: if let Some(angle) = parser.next_if(Token::is_i64).map(Token::into_f32) {
                parser.consume(&Token::ident("deg"))?;
                parser.consume(&Token::Comma)?;

                angle.to_radians()
            } else {
                0.0
            },
            ..Default::default()
        };

        while !parser.check(&Token::ParenClose) {
            if !gradient.stops.is_empty() {
                parser.consume(&Token::Comma)?;
            }

            gradient.stops.push(GradientStop::parse(parser)?);
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
            self.angle.to_degrees(),
            self.stops
                .iter()
                .map(|stop| stop.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
