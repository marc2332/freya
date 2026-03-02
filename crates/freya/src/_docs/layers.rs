//! # Layers
//!
//! Layers control the rendering order of elements. Elements in higher layers are rendered on top of those in lower layers.
//!
//! Use the `.layer()` method with one of three variants:
//!
//! - [`Layer::Relative(i16)`](freya_core::prelude::Layer::Relative) *(default)*: offset relative to the parent's layer. Positive values render higher, negative values render lower.
//! - [`Layer::Overlay`](freya_core::prelude::Layer::Overlay): jumps to a very high layer, useful for modals or tooltips that must appear above everything.
//! - [`Layer::RelativeOverlay(u8)`](freya_core::prelude::Layer::RelativeOverlay): similar to `Overlay` but clamped to 0 or 1.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! fn app() -> impl IntoElement {
//!     rect()
//!         .child(rect().layer(-1).child("Background")) // below default
//!         .child(rect().child("Content")) // default layer
//!         .child(rect().layer(1).child("Foreground")) // above default
//!         .child(rect().layer(Layer::Overlay).child("Modal")) // on top of everything
//! }
//! ```
//!
//! > **Note:** The rendering order of elements within the same layer is not guaranteed. Always use distinct layer values when order matters.
