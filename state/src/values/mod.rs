mod border;
mod color;
mod shadow;
mod font;
mod decoration;
mod radius;
mod padding;
mod display;
mod size;
mod cursor;

pub use display::*;
pub use border::*;
pub use color::*;
pub use shadow::*;
pub use font::*;
pub use decoration::*;
pub use radius::*;
pub use padding::*;
pub use size::*;
pub use cursor::*;

// FromStr but we own it so we can impl it on torin and skia_safe types.
pub trait Parse: Sized {
    type Err;

    fn parse(value: &str) -> Result<Self, Self::Err>;
}