use std::{
    f32::consts::FRAC_PI_2,
    fmt::{
        self,
        Debug,
    },
};

use freya_engine::prelude::*;
use torin::prelude::Area;

use crate::style::color::Color;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GradientStop {
    color: Color,
    offset: f32,
}

impl fmt::Display for GradientStop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        _ = self.color.fmt(f);
        write!(f, " {}%", self.offset * 100.0)
    }
}

impl GradientStop {
    pub fn new(color: impl Into<Color>, offset: f32) -> Self {
        Self {
            color: color.into(),
            offset: offset / 100.,
        }
    }
}

impl<C: Into<Color>> From<(C, f32)> for GradientStop {
    fn from((color, offset): (C, f32)) -> Self {
        GradientStop::new(color, offset)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LinearGradient {
    stops: Vec<GradientStop>,
    angle: f32,
}

impl LinearGradient {
    /// Create an empty [LinearGradient] with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a single stop.
    pub fn stop(mut self, stop: impl Into<GradientStop>) -> Self {
        self.stops.push(stop.into());
        self
    }

    /// Add multiple stops.
    pub fn stops<I>(mut self, stops: I) -> Self
    where
        I: IntoIterator<Item = GradientStop>,
    {
        self.stops.extend(stops);
        self
    }

    /// Set angle (degrees).
    pub fn angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }

    pub fn into_shader(&self, bounds: Area) -> Option<Shader> {
        let colors: Vec<SkColor> = self.stops.iter().map(|stop| stop.color.into()).collect();
        let offsets: Vec<f32> = self.stops.iter().map(|stop| stop.offset).collect();

        let (dy, dx) = (self.angle.to_radians() + FRAC_PI_2).sin_cos();
        let farthest_corner = SkPoint::new(
            if dx > 0.0 { bounds.width() } else { 0.0 },
            if dy > 0.0 { bounds.height() } else { 0.0 },
        );
        let delta = farthest_corner - SkPoint::new(bounds.width(), bounds.height()) / 2.0;
        let u = delta.x * dy - delta.y * dx;
        let endpoint = farthest_corner + SkPoint::new(-u * dy, u * dx);

        let origin = SkPoint::new(bounds.min_x(), bounds.min_y());
        Shader::linear_gradient(
            (
                SkPoint::new(bounds.width(), bounds.height()) - endpoint + origin,
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RadialGradient {
    stops: Vec<GradientStop>,
}

impl RadialGradient {
    /// Create an empty [RadialGradient] with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a single stop.
    pub fn stop(mut self, stop: impl Into<GradientStop>) -> Self {
        self.stops.push(stop.into());
        self
    }

    /// Add multiple stops.
    pub fn stops<I>(mut self, stops: I) -> Self
    where
        I: IntoIterator<Item = GradientStop>,
    {
        self.stops.extend(stops);
        self
    }

    pub fn into_shader(&self, bounds: Area) -> Option<Shader> {
        let colors: Vec<SkColor> = self.stops.iter().map(|stop| stop.color.into()).collect();
        let offsets: Vec<f32> = self.stops.iter().map(|stop| stop.offset).collect();

        let center = bounds.center();

        Shader::radial_gradient(
            SkPoint::new(center.x, center.y),
            bounds.width().max(bounds.height()) / 2.0,
            GradientShaderColors::Colors(&colors[..]),
            Some(&offsets[..]),
            TileMode::Clamp,
            None,
            None,
        )
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConicGradient {
    stops: Vec<GradientStop>,
    angles: Option<(f32, f32)>,
    angle: Option<f32>,
}

impl ConicGradient {
    /// Create an empty [ConicGradient] with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a single stop.
    pub fn stop(mut self, stop: impl Into<GradientStop>) -> Self {
        self.stops.push(stop.into());
        self
    }

    /// Add multiple stops.
    pub fn stops<I>(mut self, stops: I) -> Self
    where
        I: IntoIterator<Item = GradientStop>,
    {
        self.stops.extend(stops);
        self
    }

    /// Set explicit angle (degrees) for the gradient.
    pub fn angle(mut self, angle: f32) -> Self {
        self.angle = Some(angle);
        self
    }

    /// Set start/end angles (degrees).
    pub fn angles(mut self, start: f32, end: f32) -> Self {
        self.angles = Some((start, end));
        self
    }

    pub fn into_shader(&self, bounds: Area) -> Option<Shader> {
        let colors: Vec<SkColor> = self.stops.iter().map(|stop| stop.color.into()).collect();
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
