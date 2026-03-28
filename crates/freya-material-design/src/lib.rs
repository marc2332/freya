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
//!         .ripple() // Renders a ripple effect upon press
//!         .child("Material Button")
//! }
//! ```

pub mod button;
pub mod floating_tab;
pub mod menu;
pub mod ripple;
pub mod sidebar;

pub mod prelude {
    pub use crate::{
        button::*,
        floating_tab::*,
        menu::*,
        ripple::*,
        sidebar::*,
    };
}
