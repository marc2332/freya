//! # Elements
//!
//! This is an overview of the elements supported in Freya.
//!
//! > For more info check the [API Reference](freya_elements::elements#structs).
//!
//! ### `rect`
//!
//! [`rect`](freya_elements::elements::rect) is a generic element that acts as a container for other elements.
//!
//! You can specify things like **width**, **padding** or even in what **direction** the inner elements are stacked.
//!
//! Example:
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx!(
//!         rect {
//!             direction: "vertical",
//!             label { "Hi!" }
//!         }
//!     )
//! }
//! ```
//!
//! ### `label`
//!
//! [`label`](freya_elements::elements::label) simply let's you display some text.
//!
//! Example:
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx!(
//!         label {
//!             "Hello World"
//!         }
//!     )
//! }
//! ```
//!
//! ### `paragraph`
//!
//! [`paragraph`](freya_elements::elements::paragraph) element let's you build texts with different styles.
//!
//! This used used with the `text` element.
//!
//! Example:
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx!(
//!         paragraph {
//!             text {
//!                 font_size: "15",
//!                 "Hello, "
//!             }
//!             text {
//!                 font_size: "30",
//!                 "World!"
//!             }
//!         }
//!     )
//! }
//! ```
//!
//! ### `image`
//!
//! [`image`](freya_elements::elements::image) element let's you show an image.
//!
//! Example:
//!
//! ```rust
//! # use freya::prelude::*;
//! static RUST_LOGO: &[u8] = include_bytes!("./rust_logo.png");
//!
//! fn app() -> Element {
//!     let image_data = static_bytes(RUST_LOGO);
//!     rsx!(image {
//!         image_data,
//!         width: "100%",
//!         height: "100%",
//!     })
//! }
//! ```
//!
//! ### `svg`
//!
//! [`svg`](freya_elements::elements::svg) element let's you display an SVG.
//!
//! Example:
//!
//! ```rust
//! # use freya::prelude::*;
//! static FERRIS: &[u8] = include_bytes!("./ferris.svg");
//!
//! fn app() -> Element {
//!     let ferris = static_bytes(FERRIS);
//!     rsx!(svg {
//!         svg_data: ferris,
//!         width: "100%",
//!         height: "100%",
//!     })
//! }
//! ```
