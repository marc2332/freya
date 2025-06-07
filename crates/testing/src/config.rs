use std::time::Duration;

use torin::geometry::Size2D;

/// Configuration for [`crate::test_handler::TestingHandler`].
#[derive(Clone)]
pub struct TestingConfig<T: 'static + Clone> {
    pub vdom_timeout: Duration,
    pub size: Size2D,
    pub event_loop_ticker: bool,
    pub state: Option<T>,
}

impl<T: 'static + Clone> Default for TestingConfig<T> {
    fn default() -> Self {
        Self {
            vdom_timeout: Duration::from_millis(16),
            size: Size2D::from((500.0, 500.0)),
            event_loop_ticker: true,
            state: None,
        }
    }
}

impl<T: 'static + Clone> TestingConfig<T> {
    pub fn new() -> Self {
        TestingConfig::default()
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
