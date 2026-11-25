use freya_core::prelude::*;
use torin::size::Size;

use crate::{
    define_theme,
    icons::{
        arrow::ArrowIcon,
        tick::TickIcon,
    },
};

pub mod arrow;
pub mod tick;

/// Theme variable for the default width and height of built-in icons, in pixels.
pub const ICON_SIZE_VAR: &str = "icon.size";

fn icon_dimension(dimension: &Size, default: f32) -> Size {
    match dimension {
        Size::Inner => Size::px(default),
        dimension => dimension.clone(),
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
