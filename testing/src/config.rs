use std::time::Duration;

use freya_common::Size2D;

/// Configuration for a [`TestingHandler`].
pub struct TestingConfig {
    vdom_timeout: Duration,
    size: Size2D,
}

impl Default for TestingConfig {
    fn default() -> Self {
        Self {
            vdom_timeout: Duration::from_millis(16),
            size: Size2D::from((500.0, 500.0)),
        }
    }
}

impl TestingConfig {
    pub fn new() -> Self {
        TestingConfig::default()
    }

    /// Specify a custom canvas size.
    pub fn with_size(mut self, size: Size2D) -> Self {
        self.size = size;
        self
    }

    /// Specify a custom duration for the VirtualDOM polling timeout, default is 16ms.
    pub fn with_vdom_timeout(mut self, vdom_timeout: Duration) -> Self {
        self.vdom_timeout = vdom_timeout;
        self
    }

    /// Get the canvas size.
    pub fn size(&self) -> Size2D {
        self.size
    }

    /// Get the VirtualDOM polling timeout.
    pub fn vdom_timeout(&self) -> Duration {
        self.vdom_timeout
    }
}
