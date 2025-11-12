pub mod anim_color;
pub mod anim_num;
pub mod anim_sequential;
pub mod easing;
pub mod hook;

pub mod prelude {
    pub use crate::{
        anim_color::*,
        anim_num::*,
        anim_sequential::*,
        easing::*,
        hook::*,
    };
}
