#[cfg(feature = "mocked-engine")]
mod mocked;

#[cfg(not(feature = "mocked-engine"))]
mod skia;

pub mod prelude {
    #[cfg(feature = "mocked-engine")]
    pub use crate::mocked::*;

    #[cfg(not(feature = "mocked-engine"))]
    pub use crate::skia::*;
}
