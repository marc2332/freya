//! WebView types and configuration.

use std::sync::Arc;

use freya_core::prelude::UseId;

/// Unique identifier for a WebView instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WebViewId(pub usize);

impl WebViewId {
    /// Generate a new unique WebView ID.
    pub fn new() -> Self {
        Self(UseId::<Self>::get_in_hook())
    }
}

impl Default for WebViewId {
    fn default() -> Self {
        Self::new()
    }
}

/// Callback type for WebView customization.
pub type WebViewCallback =
    Arc<Box<dyn Fn(wry::WebViewBuilder) -> wry::WebViewBuilder + Send + 'static>>;

/// Configuration for a WebView instance.
#[derive(Clone, Default)]
pub struct WebViewConfig {
    /// The URL to load in the WebView.
    pub url: String,
    /// Whether the WebView background should be transparent.
    pub transparent: bool,
    /// Custom user agent string.
    pub user_agent: Option<String>,
    /// Optional callback called when the WebView is created.
    pub on_created: Option<WebViewCallback>,
}

impl PartialEq for WebViewConfig {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
            && self.transparent == other.transparent
            && self.user_agent == other.user_agent
            && self.on_created.is_none() == other.on_created.is_none()
    }
}

impl std::fmt::Debug for WebViewConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebViewConfig")
            .field("url", &self.url)
            .field("transparent", &self.transparent)
            .field("user_agent", &self.user_agent)
            .field(
                "on_created",
                &self.on_created.as_ref().map(|_| "<callback>"),
            )
            .finish()
    }
}
