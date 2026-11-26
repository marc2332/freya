use torin::gaps::Gaps;

use crate::theming::{
    component_themes::AnyMap,
    macros::Preference,
};

/// Scales the spacing and supported components in a theme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Density {
    /// Use `1.` to keep the default sizes.
    pub factor: f32,
}

impl Default for Density {
    fn default() -> Self {
        Self { factor: 1. }
    }
}

impl Density {
    /// Theme key for the density factor.
    pub const fn key() -> &'static str {
        "density"
    }

    pub fn apply_to(&self, variables: &mut AnyMap) {
        variables.set(Self::key(), self.factor);
        for role in PaddingRole::ALL {
            let [vertical, horizontal] = role.axes.map(|value| value * self.factor);
            variables.set(role.key(), Gaps::new_symmetric(vertical, horizontal));
        }
    }
}

/// A shared padding value in the built-in themes.
#[derive(Clone, Copy, Debug)]
pub struct PaddingRole {
    key: &'static str,
    axes: [f32; 2],
}

impl PaddingRole {
    pub const TIGHT: Self = Self::new("padding.tight", 4., 4.);
    pub const COMPACT: Self = Self::new("padding.compact", 6., 6.);
    pub const STANDARD: Self = Self::new("padding.standard", 8., 8.);
    pub const RELAXED: Self = Self::new("padding.relaxed", 12., 12.);
    pub const SPACIOUS: Self = Self::new("padding.spacious", 16., 16.);
    pub const TIGHT_WIDE: Self = Self::new("padding.tight_wide", 4., 8.);
    pub const COMPACT_WIDE: Self = Self::new("padding.compact_wide", 6., 12.);
    pub const STANDARD_WIDE: Self = Self::new("padding.standard_wide", 8., 12.);
    pub const RELAXED_WIDE: Self = Self::new("padding.relaxed_wide", 10., 16.);

    const ALL: &[Self] = &[
        Self::TIGHT,
        Self::COMPACT,
        Self::STANDARD,
        Self::RELAXED,
        Self::SPACIOUS,
        Self::TIGHT_WIDE,
        Self::COMPACT_WIDE,
        Self::STANDARD_WIDE,
        Self::RELAXED_WIDE,
    ];

    const fn new(key: &'static str, vertical: f32, horizontal: f32) -> Self {
        Self {
            key,
            axes: [vertical, horizontal],
        }
    }

    /// The theme variable key for this role.
    pub const fn key(self) -> &'static str {
        self.key
    }
}

impl From<PaddingRole> for Preference<Gaps> {
    fn from(role: PaddingRole) -> Self {
        Self::Reference(role.key())
    }
}
