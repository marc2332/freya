//! WebView types and configuration.

use std::sync::atomic::{
    AtomicU64,
    Ordering,
};

/// Unique identifier for a WebView instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WebViewId(pub u64);

impl WebViewId {
    /// Generate a new unique WebView ID.
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for WebViewId {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for a WebView instance.
#[derive(Clone, Debug, PartialEq)]
pub struct WebViewConfig {
    /// The URL to load in the WebView.
    pub url: String,
    /// Whether the WebView background should be transparent.
    pub transparent: bool,
    /// Custom user agent string.
    pub user_agent: Option<String>,
}

impl Default for WebViewConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            transparent: false,
            user_agent: None,
        }
    }
}
