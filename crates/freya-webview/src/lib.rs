//! WebView support for Freya using WRY.
//!
//! This crate provides WebView integration for Freya applications using the WRY library.
//! WebViews can be embedded in your Freya UI as regular elements that participate in the
//! layout system.
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
//!     webview("https://example.com")
//!         .width(Size::fill())
//!         .height(Size::fill())
//! }
//! ```

mod element;
mod plugin;
mod registry;

pub use element::{
    WebView,
    WebViewElement,
    webview,
};
pub use plugin::WebViewPlugin;
pub use registry::{
    WebViewConfig,
    WebViewId,
};

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::{
        element::webview,
        plugin::WebViewPlugin,
        registry::{
            WebViewConfig,
            WebViewId,
        },
    };
}
