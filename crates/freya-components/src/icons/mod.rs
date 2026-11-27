use freya_core::prelude::*;
use torin::size::Size;

use crate::{
    define_theme,
    icons::{
        arrow::ArrowIcon,
        tick::TickIcon,
    },
    theming::component_themes::AnyMap,
};

pub mod arrow;
pub mod tick;

/// The default size of built-in icons in pixels.
pub struct IconSize(pub f32);

impl Default for IconSize {
    fn default() -> Self {
        Self(10.)
    }
}

impl IconSize {
    pub fn apply_to(&self, variables: &mut AnyMap) {
        variables.set(IconTheme::size_key(), self.0);
    }
}

define_theme! {
    for = ArrowIcon;
    theme_field = theme;
    for = TickIcon;
    theme_field = theme;

    %[component]
    pub Icon {
        %[fields]
        fill: Color,
        size: f32,
    }
}

impl IconTheme {
    /// Theme key for the default icon size in pixels.
    pub const fn size_key() -> &'static str {
        "icon.size"
    }

    fn dimension(&self, dimension: &Size) -> Size {
        match dimension {
            Size::Inner => Size::px(self.size),
            dimension => dimension.clone(),
        }
    }
}
