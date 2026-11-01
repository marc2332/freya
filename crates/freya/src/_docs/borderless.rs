//! # Borderless Windows
//!
//! Windows created with `.with_decorations(false)` have no titlebar and no resize borders,
//! so the app provides them:
//!
//! - `BorderlessPlugin`, from the `freya-borderless-plugin` crate, behind the `borderless` feature.
//! - [TitlebarButton](crate::components::TitlebarButton) (`titlebar` feature) and
//!   [window_drag](freya_winit::WindowDragExt::window_drag), to build a custom titlebar.
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
//!
//! ## Platforms
//!
//! - **Linux** and **Windows**: the plugin overlays invisible resize bands on top of the app, so
//!   dragging the window borders resizes it and hovering them shows the resize cursors.
//! - **macOS**: no bands, resizing is left to the system.
//! - **All**: an optional corner radius clips the whole canvas, overlay layers included.
