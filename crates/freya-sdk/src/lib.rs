pub mod timeout;
#[cfg(feature = "tokio")]
pub mod tokio;

pub mod prelude {
    pub use crate::timeout::*;
    #[cfg(feature = "tokio")]
    pub use crate::tokio::watch::*;
}
