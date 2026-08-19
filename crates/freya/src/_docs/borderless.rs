//! # Borderless Windows
//!
//! Windows created with `.with_decorations(false)` lose the titlebar and the resize borders
//! the OS would otherwise draw, leaving the app in charge of both. Freya ships helpers for this:
//!
//! - The `BorderlessPlugin` plugin from the `freya-borderless-plugin` crate, gated behind
//!   the `borderless` feature.
//! - The [TitlebarButton](crate::components::TitlebarButton) component (`titlebar` feature) and the
//!   [window_drag](freya_winit::WindowDragExt::window_drag) method to build a custom titlebar.
//!
//! Register the plugin when launching:
//!
//! ```rust,no_run
//! use freya::{
//!     borderless::BorderlessPlugin,
//!     prelude::*,
//! };
//! # fn app() -> impl IntoElement { rect() }
//!
//! fn main() {
//!     launch(
//!         LaunchConfig::new()
//!             .with_plugin(BorderlessPlugin::new().with_corner_radius(12.))
//!             .with_window(
//!                 WindowConfig::new(app)
//!                     .with_decorations(false)
//!                     .with_transparency(true)
//!                     .with_background(Color::TRANSPARENT),
//!             ),
//!     )
//! }
//! ```
