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
pub mod card;
pub mod chip;
pub mod floating_tab;
pub mod menu;
pub mod ripple;
pub mod segmented_button;
pub mod sidebar;

pub mod prelude {
    pub use crate::{
        button::*,
        card::*,
        chip::*,
        floating_tab::*,
        menu::*,
        ripple::*,
        segmented_button::*,
        sidebar::*,
    };
}
