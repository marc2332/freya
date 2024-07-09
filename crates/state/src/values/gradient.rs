use std::fmt;

use freya_engine::prelude::*;
use torin::{prelude::Measure, size::Rect};

use crate::{DisplayColor, ExtSplit, Parse};

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
                gradient.angle = angle;
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

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRadialGradientError;

impl Parse for RadialGradient {
    type Err = ParseRadialGradientError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        if !value.starts_with("radial-gradient(") || !value.ends_with(')') {
            return Err(ParseRadialGradientError);
        }

        let mut gradient = RadialGradient::default();
        let mut value = value.replacen("radial-gradient(", "", 1);

        value.remove(value.rfind(')').ok_or(ParseRadialGradientError)?);

        for stop in value.split_excluding_group(',', '(', ')') {
            gradient
                .stops
                .push(GradientStop::parse(stop).map_err(|_| ParseRadialGradientError)?);
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

        let matrix = Matrix::rotate_deg_pivot(self.angle.unwrap_or(-90.0), (center.x, center.y));

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

#[derive(Debug, PartialEq, Eq)]
pub struct ParseConicGradientError;

impl Parse for ConicGradient {
    type Err = ParseConicGradientError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        if !value.starts_with("conic-gradient(") || !value.ends_with(')') {
            return Err(ParseConicGradientError);
        }

        let mut gradient = ConicGradient::default();
        let mut value = value.replacen("conic-gradient(", "", 1);

        value.remove(value.rfind(')').ok_or(ParseConicGradientError)?);

        let mut split = value.split_excluding_group(',', '(', ')');

        let angle_or_first_stop = split.next().ok_or(ParseConicGradientError)?.trim();

        if angle_or_first_stop.ends_with("deg") {
            if let Ok(angle) = angle_or_first_stop.replacen("deg", "", 1).parse::<f32>() {
                gradient.angle = Some(angle);
            }
        } else {
            gradient.stops.push(
                GradientStop::parse(angle_or_first_stop).map_err(|_| ParseConicGradientError)?,
            );
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
                gradient.stops.push(
                    GradientStop::parse(angles_or_second_stop)
                        .map_err(|_| ParseConicGradientError)?,
                );
            }
        }

        for stop in split {
            gradient
                .stops
                .push(GradientStop::parse(stop).map_err(|_| ParseConicGradientError)?);
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
