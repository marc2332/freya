mod accessibility;
mod border;
mod color;
mod cursor;
mod decoration;
mod display;
mod font;
mod padding;
mod radius;
mod shadow;
mod size;

pub use accessibility::*;
pub use border::*;
pub use color::*;
pub use cursor::*;
pub use decoration::*;
pub use display::*;
pub use font::*;
pub use padding::*;
pub use radius::*;
pub use shadow::*;
pub use size::*;

// FromStr but we own it so we can impl it on torin and skia_safe types.
pub trait Parse: Sized {
    type Err;

    fn parse(value: &str) -> Result<Self, Self::Err>;
}
