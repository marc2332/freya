use std::f32::consts::SQRT_2;

use freya_engine::prelude::*;
use torin::scaled::Scaled;

/// Radius applied to each corner of an element to round it, plus an optional
/// `smoothing` factor (`0.0..=1.0`) that turns sharp rounding into a squircle.
///
/// Prefer the constructors [`CornerRadius::new_all`], [`CornerRadius::new_symmetric`] and [`CornerRadius::new`].
/// It also implements `From<f32>`, `From<(f32, f32)>` and `From<(f32, f32, f32, f32)>`.
///
/// ```
/// # use freya::prelude::*;
/// let all = CornerRadius::new_all(8.0);
/// let symmetric = CornerRadius::new_symmetric(8.0, 0.0); // (top, bottom)
/// let each = CornerRadius::new(1.0, 2.0, 3.0, 4.0); // (top_left, top_right, bottom_right, bottom_left)
/// let squircle = CornerRadius::new_all(8.0).with_smoothing(0.6);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Clone, Debug, Default, Copy)]
pub struct CornerRadius {
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
    smoothing: f32,
}

impl From<f32> for CornerRadius {
    fn from(radius: f32) -> Self {
        CornerRadius::new_all(radius)
    }
}

impl From<(f32, f32)> for CornerRadius {
    fn from((top, bottom): (f32, f32)) -> Self {
        CornerRadius::new_symmetric(top, bottom)
    }
}

impl From<(f32, f32, f32, f32)> for CornerRadius {
    fn from((top_left, top_right, bottom_right, bottom_left): (f32, f32, f32, f32)) -> Self {
        CornerRadius::new(top_left, top_right, bottom_right, bottom_left)
    }
}

impl CornerRadius {
    /// Create a [`CornerRadius`] with an individual radius for each corner, in
    /// `(top_left, top_right, bottom_right, bottom_left)` order.
    pub const fn new(top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32) -> Self {
        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
            smoothing: 0.,
        }
    }

    /// Create a [`CornerRadius`] with the same radius on all four corners.
    pub const fn new_all(radius: f32) -> Self {
        Self::new(radius, radius, radius, radius)
    }

    /// Create a [`CornerRadius`] with one radius for the two top corners and another for the two bottom corners.
    pub const fn new_symmetric(top: f32, bottom: f32) -> Self {
        Self::new(top, top, bottom, bottom)
    }

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

    /// Return a copy of this [`CornerRadius`] with the given `smoothing` (clamped to `0.0..=1.0`).
    pub fn with_smoothing(mut self, smoothing: f32) -> Self {
        self.smoothing = smoothing.clamp(0.0, 1.0);
        self
    }

    pub fn top_left(&self) -> f32 {
        self.top_left
    }

    pub fn top_right(&self) -> f32 {
        self.top_right
    }

    pub fn bottom_right(&self) -> f32 {
        self.bottom_right
    }

    pub fn bottom_left(&self) -> f32 {
        self.bottom_left
    }

    pub fn smoothing(&self) -> f32 {
        self.smoothing
    }

    // https://github.com/aloisdeniel/figma_squircle/blob/main/lib/src/path_smooth_corners.dart
    pub fn smoothed_path(&self, rect: RRect) -> Path {
        let mut path = PathBuilder::new();

        let left = rect.rect().left();
        let top = rect.rect().top();
        let width = rect.width();
        let height = rect.height();

        let top_right = rect.radii(SkCorner::UpperRight).x;
        if top_right > 0.0 {
            let (a, b, c, d, l, p, radius) =
                compute_smooth_corner(top_right, self.smoothing, width, height);

            path.move_to((f32::max(width / 2.0, width - p), 0.0))
                .cubic_to(
                    (width - (p - a), 0.0),
                    (width - (p - a - b), 0.0),
                    (width - (p - a - b - c), d),
                )
                .r_arc_to(
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

        let bottom_right = rect.radii(SkCorner::LowerRight).x;
        if bottom_right > 0.0 {
            let (a, b, c, d, l, p, radius) =
                compute_smooth_corner(bottom_right, self.smoothing, width, height);

            path.line_to((width, f32::max(height / 2.0, height - p)))
                .cubic_to(
                    (width, height - (p - a)),
                    (width, height - (p - a - b)),
                    (width - d, height - (p - a - b - c)),
                )
                .r_arc_to(
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

        let bottom_left = rect.radii(SkCorner::LowerLeft).x;
        if bottom_left > 0.0 {
            let (a, b, c, d, l, p, radius) =
                compute_smooth_corner(bottom_left, self.smoothing, width, height);

            path.line_to((f32::min(width / 2.0, p), height))
                .cubic_to(
                    (p - a, height),
                    (p - a - b, height),
                    (p - a - b - c, height - d),
                )
                .r_arc_to(
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

        let top_left = rect.radii(SkCorner::UpperLeft).x;
        if top_left > 0.0 {
            let (a, b, c, d, l, p, radius) =
                compute_smooth_corner(top_left, self.smoothing, width, height);

            path.line_to((0.0, f32::min(height / 2.0, p)))
                .cubic_to((0.0, p - a), (0.0, p - a - b), (d, p - a - b - c))
                .r_arc_to(
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

        path.detach()
            .make_transform(&Matrix::translate((left, top)))
    }

    pub fn pretty(&self) -> String {
        format!(
            "({}, {}, {}, {})",
            self.top_left, self.top_right, self.bottom_right, self.bottom_left
        )
    }

    pub fn is_round(&self) -> bool {
        self.top_left > 0. || self.top_right > 0. || self.bottom_right > 0. || self.bottom_left > 0.
    }
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

impl Scaled for CornerRadius {
    fn scale(&mut self, scale: f32) {
        self.top_left *= scale;
        self.top_right *= scale;
        self.bottom_left *= scale;
        self.bottom_right *= scale;
    }
}
