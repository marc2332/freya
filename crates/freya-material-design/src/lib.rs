//! Material Design extensions for Freya.
//!
//! # Example
//!
//! ```rust
//! use freya::{
//!     material_design::*,
//!     prelude::*,
//! };
//!
//! fn app() -> impl IntoElement {
//!     Button::new()
//!         .on_press(|_| println!("Pressed"))
//!         .ripple()
//!         .child("Material Button")
//! }
//! ```

pub mod ripple;

pub mod prelude {
    pub use crate::ripple::*;
}
