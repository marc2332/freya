mod accessibility;
mod border;
mod color;
mod corner_radius;
mod cursor;
mod decoration;
mod display;
mod font;
mod gaps;
mod overflow;
mod shadow;
mod size;
mod text_shadow;

pub use accessibility::*;
pub use border::*;
pub use color::*;
pub use corner_radius::*;
pub use cursor::*;
pub use decoration::*;
pub use display::*;
pub use font::*;
pub use gaps::*;
pub use overflow::*;
pub use shadow::*;
pub use size::*;
pub use text_shadow::*;

// FromStr but we own it so we can impl it on torin and skia_safe types.
pub trait Parse: Sized {
    type Err;

    fn parse(value: &str) -> Result<Self, Self::Err>;
}
