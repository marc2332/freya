//! # Accessibility
//!
//! Freya accessibility support works around the [`use_focus`](freya_hooks::use_focus) and the `a11y` attributes.
//!
//! ### `use_focus` hook
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     // Create a focus instance
//!     let mut my_focus = use_focus();
//!
//!     rsx!(
//!         rect {
//!             // Bind the focus to this `rect`
//!             a11y_id: my_focus.attribute(),
//!             // This will focus this element and effectively cause a rerender updating the returned value of `is_focused()`
//!             onclick: move |_| my_focus.focus(),
//!             label {
//!                 "Am I focused? {my_focus.is_focused()}"
//!             }
//!         }
//!     )
//! }
//! ```
