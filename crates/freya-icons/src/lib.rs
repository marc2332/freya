//! Icon assets for Freya UI.
//!
//! Currently the `lucide` feature enables the Lucide icons.
//!
//! # Example
//!
//! The example below shows using an icon with the `svg` element.
//!
//! See `examples/feature_icons.rs` for a live example.
//!
//! ```rust
//! # use freya::prelude::*;
//! # #[cfg(feature = "lucide")]
//! fn app() -> impl IntoElement {
//!     svg(freya_icons::lucide::antenna())
//!         .color((120, 50, 255))
//!         .width(Size::px(48.))
//!         .height(Size::px(48.))
//! }
//! ```

#[cfg(feature = "lucide")]
pub mod lucide {
    include!(concat!(env!("OUT_DIR"), "/lucide.rs"));
}

#[macro_export]
macro_rules! generate_svg {
    ($name:ident, $path:expr) => {
        #[allow(unused)]
        pub fn $name() -> bytes::Bytes {
            bytes::Bytes::from_static(include_bytes!($path))
        }
    };
}
