use freya_core::prelude::*;

use crate::theming::component_themes::AnyMap;

#[derive(Clone, Debug, PartialEq)]
pub struct CornerRadiusScale {
    /// Base radius in logical pixels.
    pub base_radius: f32,
    pub xs: CornerRadiusSize,
    pub sm: CornerRadiusSize,
    pub md: CornerRadiusSize,
    pub lg: CornerRadiusSize,
    pub xl: CornerRadiusSize,
    pub xxl: CornerRadiusSize,
    pub xxxl: CornerRadiusSize,
}

impl Default for CornerRadiusScale {
    fn default() -> Self {
        Self {
            base_radius: 8.,
            xs: CornerRadiusSize::xs(),
            sm: CornerRadiusSize::sm(),
            md: CornerRadiusSize::md(),
            lg: CornerRadiusSize::lg(),
            xl: CornerRadiusSize::xl(),
            xxl: CornerRadiusSize::xxl(),
            xxxl: CornerRadiusSize::xxxl(),
        }
    }
}

/// A named size from the shared corner radius theme.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CornerRadiusRole {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
    Xxxl,
}

impl CornerRadiusRole {
    pub const fn key(self) -> &'static str {
        match self {
            Self::Xs => "corner_radius.xs",
            Self::Sm => "corner_radius.sm",
            Self::Md => "corner_radius.md",
            Self::Lg => "corner_radius.lg",
            Self::Xl => "corner_radius.xl",
            Self::Xxl => "corner_radius.xxl",
            Self::Xxxl => "corner_radius.xxxl",
        }
    }
}

impl CornerRadiusScale {
    /// Write resolved corner radius values.
    pub fn apply_to(&self, variables: &mut AnyMap) {
        for (role, size) in [
            (CornerRadiusRole::Xs, self.xs),
            (CornerRadiusRole::Sm, self.sm),
            (CornerRadiusRole::Md, self.md),
            (CornerRadiusRole::Lg, self.lg),
            (CornerRadiusRole::Xl, self.xl),
            (CornerRadiusRole::Xxl, self.xxl),
            (CornerRadiusRole::Xxxl, self.xxxl),
        ] {
            variables.set(role.key(), size.resolve(self.base_radius));
        }
    }
}

/// A fixed corner radius or a multiplier of the theme's base radius.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CornerRadiusSize {
    Fixed(CornerRadius),
    Relative(f32),
}

impl<T: Into<CornerRadius>> From<T> for CornerRadiusSize {
    fn from(radius: T) -> Self {
        Self::Fixed(radius.into())
    }
}

impl CornerRadiusSize {
    /// Half the theme's base radius.
    pub const fn xs() -> Self {
        Self::Relative(0.5)
    }

    /// Three quarters of the theme's base radius.
    pub const fn sm() -> Self {
        Self::Relative(0.75)
    }

    /// The theme's base radius.
    pub const fn md() -> Self {
        Self::Relative(1.)
    }

    /// 1.25 times the theme's base radius.
    pub const fn lg() -> Self {
        Self::Relative(1.25)
    }

    /// 1.5 times the theme's base radius.
    pub const fn xl() -> Self {
        Self::Relative(1.5)
    }

    /// 1.75 times the theme's base radius.
    pub const fn xxl() -> Self {
        Self::Relative(1.75)
    }

    /// 2.5 times the theme's base radius.
    pub const fn xxxl() -> Self {
        Self::Relative(2.5)
    }

    /// Resolve the corner radius to actual values.
    pub fn resolve(self, base_radius: f32) -> CornerRadius {
        match self {
            Self::Fixed(radius) => radius,
            Self::Relative(multiplier) => CornerRadius::new_all(base_radius * multiplier),
        }
    }
}
