//! WebView support for Freya using WRY.
//!
//! This crate provides WebView integration for Freya applications using the WRY library.
//! WebViews can be embedded in your Freya UI as regular elements.
//!
//! # Example
//!
//! ```rust,no_run
//! use freya::prelude::*;
//! use freya_webview::prelude::*;
//!
//! fn main() {
//!     launch(
//!         LaunchConfig::new()
//!             .with_plugin(WebViewPlugin::new())
//!             .with_window(WindowConfig::new(app)),
//!     )
//! }
//!
//! fn app() -> impl IntoElement {
//!     WebView::new("https://example.com").expanded()
//! }
//! ```

pub mod component;
mod element;
pub mod lifecycle;
pub mod plugin;
pub mod registry;

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::{
        component::WebView,
        lifecycle::WebViewManager,
        plugin::WebViewPlugin,
        registry::{
            WebViewConfig,
            WebViewId,
        },
    };
}
