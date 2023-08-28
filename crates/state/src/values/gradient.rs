use crate::{DisplayColor, ExtSplit, Parse};
use freya_engine::prelude::*;
use std::fmt;
use torin::{prelude::Measure, size::Rect};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GradientStop {
    pub color: Color,
    pub offset: f32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGradientStopError;

impl Parse for GradientStop {
    type Err = ParseGradientStopError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        let mut split = value.split_ascii_whitespace_excluding_group('(', ')');
        let color_str = split.next().ok_or(ParseGradientStopError)?;

        let offset_str = split.next().ok_or(ParseGradientStopError)?.trim();
        if !offset_str.ends_with('%') || split.next().is_some() {
            return Err(ParseGradientStopError);
        }

        let offset = offset_str
            .replacen('%', "", 1)
            .parse::<f32>()
            .map_err(|_| ParseGradientStopError)?
            / 100.0;

        Ok(GradientStop {
            color: Color::parse(color_str).map_err(|_| ParseGradientStopError)?,
            offset,
        })
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

#[derive(Debug, PartialEq, Eq)]
pub struct ParseLinearGradientError;

impl Parse for LinearGradient {
    type Err = ParseLinearGradientError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        if !value.starts_with("linear-gradient(") || !value.ends_with(')') {
            return Err(ParseLinearGradientError);
        }

        let mut gradient = LinearGradient::default();
        let mut value = value.replacen("linear-gradient(", "", 1);
        value.remove(value.rfind(')').ok_or(ParseLinearGradientError)?);

        let mut split = value.split_excluding_group(',', '(', ')');

        let angle_or_first_stop = split.next().ok_or(ParseLinearGradientError)?.trim();

        if angle_or_first_stop.ends_with("deg") {
            if let Ok(angle) = angle_or_first_stop.replacen("deg", "", 1).parse::<f32>() {
                gradient.angle = angle.to_radians();
            }
        } else {
            gradient.stops.push(
                GradientStop::parse(angle_or_first_stop).map_err(|_| ParseLinearGradientError)?,
            );
        }

        for stop in split {
            gradient
                .stops
                .push(GradientStop::parse(stop).map_err(|_| ParseLinearGradientError)?);
        }

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
