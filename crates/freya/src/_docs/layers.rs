//! # Layers
//!
//! Layers control the rendering order of elements in your UI. Elements in higher layers are rendered on top of elements in lower layers.
//!
//! ## Layer Enum
//!
//! The [`Layer`](freya_core::prelude::Layer) enum has three variants:
//!
//! - **[`Layer::Relative(i16)`](freya_core::prelude::Layer::Relative)**: Positions the element relative to its parent's layer. This is the default. Values can be positive (render higher) or negative (render lower).
//! - **[`Layer::Overlay`](freya_core::prelude::Layer::Overlay)**: Renders the element at a high layer value, useful for modals, tooltips, and other overlay UI that should appear above everything else.
//! - **[`Layer::RelativeOverlay(u8)`](freya_core::prelude::Layer::RelativeOverlay)**: Renders at either layer 0 or 1 (clamped), useful for overlay elements that should be near their parent.
//!
//! ## Usage
//!
//! Use the [`.layer()`](freya_core::prelude::layer) method on any element:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! fn app() -> impl IntoElement {
//!     rect()
//!         .layer(1) // Renders above default layer 0
//!         .child("This is above other content")
//! }
//! ```
//!
//! For overlays:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! fn modal() -> impl IntoElement {
//!     rect()
//!         .layer(Layer::Overlay) // Appears above everything
//!         .child("I'm a modal!")
//! }
//! ```
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! fn tooltip() -> impl IntoElement {
//!     rect()
//!         .layer(Layer::RelativeOverlay(1)) // Near overlay level
//!         .child("I'm a tooltip")
//! }
//! ```
//!
//! ## Important: Unstable Order Within Same Layer
//!
//! **The rendering order of elements within the same layer is NOT guaranteed.** If you need a specific rendering order among siblings, use different layer values:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! fn ordered_layers() -> impl IntoElement {
//!     rect()
//!         .child(
//!             rect()
//!                 .layer(-1) // Renders first (bottom)
//!                 .child("Background"),
//!         )
//!         .child(
//!             rect()
//!                 .layer(0) // Renders second
//!                 .child("Content"),
//!         )
//!         .child(
//!             rect()
//!                 .layer(1) // Renders third (top)
//!                 .child("Foreground"),
//!         )
//! }
//! ```
//!
//! Freya uses a hashing-based algorithm for sorting elements within the same layer, which provides good average performance but does not preserve insertion order. Always use explicit layer values when order matters.
