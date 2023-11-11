use std::time::Duration;

use torin::geometry::Size2D;

/// Configuration for [`crate::test_handler::TestingHandler`].
#[derive(Clone, Copy)]
pub struct TestingConfig {
    pub(crate) vdom_timeout: Duration,
    pub(crate) size: Size2D,
    pub(crate) run_ticker: bool,
}

impl Default for TestingConfig {
    fn default() -> Self {
        Self {
            vdom_timeout: Duration::from_millis(16),
            size: Size2D::from((500.0, 500.0)),
            run_ticker: true,
        }
    }
}

impl TestingConfig {
    pub fn new() -> Self {
        TestingConfig::default()
    }

    /// Specify a custom canvas size.
    pub fn with_size(&mut self, size: Size2D) -> &mut Self {
        self.size = size;
        self
    }

    /// Specify a custom duration for the VirtualDOM polling timeout, default is 16ms.
    pub fn with_vdom_timeout(&mut self, vdom_timeout: Duration) -> &mut Self {
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

    pub fn enable_ticker(&mut self, ticker: bool) {
        self.run_ticker = ticker;
    }
}
