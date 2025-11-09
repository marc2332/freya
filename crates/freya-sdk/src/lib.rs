#[cfg(feature = "tokio")]
pub mod tokio;

pub mod prelude {
    #[cfg(feature = "tokio")]
    pub use crate::tokio::watch::*;
}
