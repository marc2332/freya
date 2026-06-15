//! WebView support for Freya using WRY.
//!
//! This crate provides WebView integration for Freya applications using the WRY library.
//! WebViews can be embedded in your Freya UI as regular elements.
//!
//! # Registering the plugin
//!
//! WebViews are managed by [`WebViewPlugin`](plugin::WebViewPlugin), so it must be registered in your
//! `LaunchConfig` with `with_plugin` before any [`WebView`](component::WebView) is rendered. Without
//! it the [`WebView`](component::WebView) component will panic, as it has no plugin to drive its
//! lifecycle.
//!
//! # Platform compatibility
//!
//! WebViews rely on each platform's native web engine, so support is limited to:
//!
//! - **Windows**: WebView2.
//! - **macOS**: WKWebView.
//! - **Linux (x11)**: WebKitGTK.
//! - **Linux (Wayland)**: Not supported**.
//! - **Android**: **Not supported**.
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
//!             // The plugin must be registered for WebViews to work.
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
