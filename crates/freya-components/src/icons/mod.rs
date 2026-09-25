use freya_core::prelude::*;

use crate::{
    define_theme,
    icons::{
        arrow::ArrowIcon,
        tick::TickIcon,
    },
};

pub mod arrow;
pub mod tick;

define_theme! {
    for = ArrowIcon;
    theme_field = theme;
    for = TickIcon;
    theme_field = theme;

    %[component]
    pub Icon {
        %[fields]
        fill: Color,
    }
}
