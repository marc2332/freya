use freya_core::prelude::*;

use crate::theming::{
    component_themes::{
        ColorsSheet,
        Theme,
    },
    hooks::get_theme_or_default,
    macros::{
        Preference,
        ResolvablePreference,
    },
};

/// Set under the `"corner_radius"` theme key to customize the shared radius scale.
#[derive(Clone, Debug, PartialEq)]
pub struct CornerRadiusThemePreference {
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

impl Default for CornerRadiusThemePreference {
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

impl CornerRadiusThemePreference {
    pub fn corner_radius(&self, role: CornerRadiusRole) -> CornerRadius {
        let size = match role {
            CornerRadiusRole::Xs => self.xs,
            CornerRadiusRole::Sm => self.sm,
            CornerRadiusRole::Md => self.md,
            CornerRadiusRole::Lg => self.lg,
            CornerRadiusRole::Xl => self.xl,
            CornerRadiusRole::Xxl => self.xxl,
            CornerRadiusRole::Xxxl => self.xxxl,
        };
        size.resolve(self.base_radius)
    }
}

#[doc(hidden)]
pub type ResolvedCornerRadius = CornerRadius;

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

    /// Resolve using a base in logical pixels. Fixed radii ignore the base.
    pub fn resolve(self, base_radius: f32) -> CornerRadius {
        match self {
            Self::Fixed(radius) => radius,
            Self::Relative(multiplier) => CornerRadius::new_all(base_radius * multiplier),
        }
    }
}

impl ResolvablePreference<CornerRadius> for Preference<CornerRadiusSize> {
    fn resolve(&self, _colors_sheet: &ColorsSheet) -> CornerRadius {
        match self {
            Self::Specific(CornerRadiusSize::Fixed(radius)) => *radius,
            Self::Specific(CornerRadiusSize::Relative(_)) | Self::CornerRadius(_) => {
                panic!("Named and relative radii require a full theme.")
            }
            _ => panic!("Radius sizes must be fixed values or relative multipliers."),
        }
    }

    fn resolve_with_theme(&self, theme: &Theme) -> CornerRadius {
        match self {
            Self::Specific(radius @ CornerRadiusSize::Relative(_)) => {
                radius.resolve(theme.corner_radius_theme().base_radius)
            }
            Self::CornerRadius(role) => theme.corner_radius_theme().corner_radius(*role),
            _ => self.resolve(&theme.colors),
        }
    }
}

pub trait ThemeCornerRadiusExt: DecorationExt {
    /// Apply a fixed radius or a multiplier of the active theme's base radius.
    fn theme_corner_radius(self, radius: CornerRadiusSize) -> Self {
        self.corner_radius(
            Preference::Specific(radius).resolve_with_theme(&get_theme_or_default().read()),
        )
    }
}

impl<T: DecorationExt> ThemeCornerRadiusExt for T {}
