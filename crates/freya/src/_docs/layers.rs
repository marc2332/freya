//! # Layers
//!
//! Layers control the rendering order of elements. Elements in higher layers are rendered on top of those in lower layers.
//!
//! Use the `.layer()` method with one of three variants:
//!
//! - [`Layer::Relative(i16)`](freya_core::prelude::Layer::Relative) *(default)*: relative layer to the parent's layer. `0` keeps the normal stacking order, negative values move behind siblings and positive values in front.
//! - [`Layer::Overlay`](freya_core::prelude::Layer::Overlay): adds a big layer jump relative to the parent's layer. You may stack multiple overlays on top of each other.
//! - [`Layer::OverlayLevel(u8)`](freya_core::prelude::Layer::OverlayLevel): paint on a specific, numbered overlay level, regardless of the parent's layer. There are up to 16 levels you can use, anything above will be capped at 16.
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
