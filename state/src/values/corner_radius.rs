use crate::Parse;
use freya_engine::prelude::*;
use std::f32::consts::SQRT_2;
use std::fmt;
use torin::scaled::Scaled;

#[derive(PartialEq, Clone, Debug, Default, Copy)]
pub struct CornerRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
    pub smoothing: f32,
}

// https://www.figma.com/blog/desperately-seeking-squircles/
fn compute_smooth_corner(
    corner_radius: f32,
    smoothing: f32,
    width: f32,
    height: f32,
) -> (f32, f32, f32, f32, f32, f32, f32) {
    let max_p = f32::min(width, height) / 2.0;
    let corner_radius = f32::min(corner_radius, max_p);

    let p = f32::min((1.0 + smoothing) * corner_radius, max_p);

    let angle_alpha: f32;
    let angle_beta: f32;

    if corner_radius <= max_p / 2.0 {
        angle_alpha = 45.0 * smoothing;
        angle_beta = 90.0 * (1.0 - smoothing);
    } else {
        let diff_ratio = (corner_radius - max_p / 2.0) / (max_p / 2.0);

        angle_alpha = 45.0 * smoothing * (1.0 - diff_ratio);
        angle_beta = 90.0 * (1.0 - smoothing * (1.0 - diff_ratio));
    }

    let angle_theta = (90.0 - angle_beta) / 2.0;
    let dist_p3_p4 = corner_radius * (angle_theta / 2.0).to_radians().tan();

    let l = (angle_beta / 2.0).to_radians().sin() * corner_radius * SQRT_2;
    let c = dist_p3_p4 * angle_alpha.to_radians().cos();
    let d = c * angle_alpha.to_radians().tan();
    let b = (p - l - c - d) / 3.0;
    let a = 2.0 * b;

    (a, b, c, d, l, p, corner_radius)
}

impl CornerRadius {
    pub fn fill_top(&mut self, value: f32) {
        self.top_left = value;
        self.top_right = value;
    }

    pub fn fill_bottom(&mut self, value: f32) {
        self.bottom_left = value;
        self.bottom_right = value;
    }

    pub fn fill_all(&mut self, value: f32) {
        self.fill_bottom(value);
        self.fill_top(value);
    }

    // https://github.com/aloisdeniel/figma_squircle/blob/main/lib/src/path_smooth_corners.dart
    pub fn smoothed_path(&self, rect: RRect) -> Path {
        let mut path = Path::new();

        let width = rect.width();
        let height = rect.height();

        let top_right = rect.radii(Corner::UpperRight).x;
        if top_right > 0.0 {
            let (a, b, c, d, l, p, radius) =
                compute_smooth_corner(top_right, self.smoothing, width, height);

            path.move_to((f32::max(width / 2.0, width - p), 0.0))
                .cubic_to(
                    (width - (p - a), 0.0),
                    (width - (p - a - b), 0.0),
                    (width - (p - a - b - c), d),
                )
                .r_arc_to_rotated(
                    (radius, radius),
                    0.0,
                    ArcSize::Small,
                    PathDirection::CW,
                    (l, l),
                )
                .cubic_to(
                    (width, p - a - b),
                    (width, p - a),
                    (width, f32::min(height / 2.0, p)),
                );
        } else {
            path.move_to((width / 2.0, 0.0))
                .line_to((width, 0.0))
                .line_to((width, height / 2.0));
        }

        let bottom_right = rect.radii(Corner::LowerRight).x;
        if bottom_right > 0.0 {
            let (a, b, c, d, l, p, radius) =
                compute_smooth_corner(bottom_right, self.smoothing, width, height);

            path.line_to((width, f32::max(height / 2.0, height - p)))
                .cubic_to(
                    (width, height - (p - a)),
                    (width, height - (p - a - b)),
                    (width - d, height - (p - a - b - c)),
                )
                .r_arc_to_rotated(
                    (radius, radius),
                    0.0,
                    ArcSize::Small,
                    PathDirection::CW,
                    (-l, l),
                )
                .cubic_to(
                    (width - (p - a - b), height),
                    (width - (p - a), height),
                    (f32::max(width / 2.0, width - p), height),
                );
        } else {
            path.line_to((width, height)).line_to((width / 2.0, height));
        }

        let bottom_left = rect.radii(Corner::LowerLeft).x;
        if bottom_left > 0.0 {
            let (a, b, c, d, l, p, radius) =
                compute_smooth_corner(bottom_left, self.smoothing, width, height);

            path.line_to((f32::min(width / 2.0, p), height))
                .cubic_to(
                    (p - a, height),
                    (p - a - b, height),
                    (p - a - b - c, height - d),
                )
                .r_arc_to_rotated(
                    (radius, radius),
                    0.0,
                    ArcSize::Small,
                    PathDirection::CW,
                    (-l, -l),
                )
                .cubic_to(
                    (0.0, height - (p - a - b)),
                    (0.0, height - (p - a)),
                    (0.0, f32::max(height / 2.0, height - p)),
                );
        } else {
            path.line_to((0.0, height)).line_to((0.0, height / 2.0));
        }

        let top_left = rect.radii(Corner::UpperLeft).x;
        if top_left > 0.0 {
            let (a, b, c, d, l, p, radius) =
                compute_smooth_corner(top_left, self.smoothing, width, height);

            path.line_to((0.0, f32::min(height / 2.0, p)))
                .cubic_to((0.0, p - a), (0.0, p - a - b), (d, p - a - b - c))
                .r_arc_to_rotated(
                    (radius, radius),
                    0.0,
                    ArcSize::Small,
                    PathDirection::CW,
                    (l, -l),
                )
                .cubic_to(
                    (p - a - b, 0.0),
                    (p - a, 0.0),
                    (f32::min(width / 2.0, p), 0.0),
                );
        } else {
            path.line_to((0.0, 0.0));
        }

        path.close();
        path
    }

    pub fn pretty(&self) -> String {
        format!(
            "({}, {}, {}, {})",
            self.top_left, self.top_right, self.bottom_right, self.bottom_left
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCornerRadiusError;

impl Parse for CornerRadius {
    type Err = ParseCornerRadiusError;

    fn parse(value: &str) -> Result<Self, Self::Err> {
        let mut radius = CornerRadius::default();
        let mut values = value.split_ascii_whitespace();

        match values.clone().count() {
            // Same in all corners
            1 => {
                radius.fill_all(
                    values
                        .next()
                        .ok_or(ParseCornerRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseCornerRadiusError)?,
                );
            }
            // By Top and Bottom
            2 => {
                // Top
                radius.fill_top(
                    values
                        .next()
                        .ok_or(ParseCornerRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseCornerRadiusError)?,
                );

                // Bottom
                radius.fill_bottom(
                    values
                        .next()
                        .ok_or(ParseCornerRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseCornerRadiusError)?,
                )
            }
            // Each corner
            4 => {
                radius = CornerRadius {
                    top_left: values
                        .next()
                        .ok_or(ParseCornerRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseCornerRadiusError)?,
                    top_right: values
                        .next()
                        .ok_or(ParseCornerRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseCornerRadiusError)?,
                    bottom_left: values
                        .next()
                        .ok_or(ParseCornerRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseCornerRadiusError)?,
                    bottom_right: values
                        .next()
                        .ok_or(ParseCornerRadiusError)?
                        .parse::<f32>()
                        .map_err(|_| ParseCornerRadiusError)?,
                    ..Default::default()
                }
            }
            _ => return Err(ParseCornerRadiusError),
        }

        Ok(radius)
    }
}

impl fmt::Display for CornerRadius {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.top_left, self.top_right, self.bottom_left, self.bottom_right
        )
    }
}

impl Scaled for CornerRadius {
    fn scale(&mut self, scale: f32) {
        self.top_left *= scale;
        self.top_right *= scale;
        self.bottom_left *= scale;
        self.bottom_right *= scale;
    }
}
