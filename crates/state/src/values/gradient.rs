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
    DisplayColor,
    ExtSplit,
    Parse,
    ParseError,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GradientStop {
    pub color: Color,
    pub offset: f32,
}

impl Parse for GradientStop {
    fn parse(value: &str) -> Result<Self, ParseError> {
        let mut split = value.split_ascii_whitespace_excluding_group('(', ')');
        let color_str = split.next().ok_or(ParseError)?;

        let offset_str = split.next().ok_or(ParseError)?.trim();
        if !offset_str.ends_with('%') || split.next().is_some() {
            return Err(ParseError);
        }

        let offset = offset_str
            .replacen('%', "", 1)
            .parse::<f32>()
            .map_err(|_| ParseError)?
            / 100.0;

        Ok(GradientStop {
            color: Color::parse(color_str).map_err(|_| ParseError)?,
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
    fn parse(value: &str) -> Result<Self, ParseError> {
        if !value.starts_with("linear-gradient(") || !value.ends_with(')') {
            return Err(ParseError);
        }

        let mut gradient = LinearGradient::default();
        let mut value = value.replacen("linear-gradient(", "", 1);
        value.remove(value.rfind(')').ok_or(ParseError)?);

        let mut split = value.split_excluding_group(',', '(', ')');

        let angle_or_first_stop = split.next().ok_or(ParseError)?.trim();

        if angle_or_first_stop.ends_with("deg") {
            if let Ok(angle) = angle_or_first_stop.replacen("deg", "", 1).parse::<f32>() {
                gradient.angle = angle;
            }
        } else {
            gradient
                .stops
                .push(GradientStop::parse(angle_or_first_stop)?);
        }

        for stop in split {
            gradient.stops.push(GradientStop::parse(stop)?);
        }

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
    fn parse(value: &str) -> Result<Self, ParseError> {
        if !value.starts_with("radial-gradient(") || !value.ends_with(')') {
            return Err(ParseError);
        }

        let mut gradient = RadialGradient::default();
        let mut value = value.replacen("radial-gradient(", "", 1);

        value.remove(value.rfind(')').ok_or(ParseError)?);

        for stop in value.split_excluding_group(',', '(', ')') {
            gradient.stops.push(GradientStop::parse(stop)?);
        }

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
    fn parse(value: &str) -> Result<Self, ParseError> {
        if !value.starts_with("conic-gradient(") || !value.ends_with(')') {
            return Err(ParseError);
        }

        let mut gradient = ConicGradient::default();
        let mut value = value.replacen("conic-gradient(", "", 1);

        value.remove(value.rfind(')').ok_or(ParseError)?);

        let mut split = value.split_excluding_group(',', '(', ')');

        let angle_or_first_stop = split.next().ok_or(ParseError)?.trim();

        if angle_or_first_stop.ends_with("deg") {
            if let Ok(angle) = angle_or_first_stop.replacen("deg", "", 1).parse::<f32>() {
                gradient.angle = Some(angle);
            }
        } else {
            gradient
                .stops
                .push(GradientStop::parse(angle_or_first_stop).map_err(|_| ParseError)?);
        }

        if let Some(angles_or_second_stop) = split.next().map(str::trim) {
            if angles_or_second_stop.starts_with("from ") && angles_or_second_stop.ends_with("deg")
            {
                if let Some(start) = angles_or_second_stop
                    .find("deg")
                    .and_then(|index| angles_or_second_stop.get(5..index))
                    .and_then(|slice| slice.parse::<f32>().ok())
                {
                    let end = angles_or_second_stop
                        .find(" to ")
                        .and_then(|index| angles_or_second_stop.get(index + 4..))
                        .and_then(|slice| slice.find("deg").and_then(|index| slice.get(0..index)))
                        .and_then(|slice| slice.parse::<f32>().ok())
                        .unwrap_or(360.0);

                    gradient.angles = Some((start, end));
                }
            } else {
                gradient
                    .stops
                    .push(GradientStop::parse(angles_or_second_stop)?);
            }
        }

        for stop in split {
            gradient.stops.push(GradientStop::parse(stop)?);
        }

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
