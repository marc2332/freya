//! # Accessibility
//!
//! TODO
//!
//! ### `use_focus` hook
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     let mut my_focus = use_focus();
//!
//!     rsx!(
//!         rect {
//!             width: "100%",
//!             height: "100%",
//!             a11y_id: my_focus.attribute(),
//!             onclick: move |_| my_focus.focus(),
//!             label {
//!                 "{my_focus.is_focused()}"
//!             }
//!         }
//!     )
//! }
//! ```
