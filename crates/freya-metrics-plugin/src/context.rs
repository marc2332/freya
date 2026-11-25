use std::{
    cell::Cell,
    rc::Rc,
};

use freya_core::prelude::try_consume_root_context;
use freya_winit::renderer::LaunchHandle;

#[derive(Clone, Copy, Default, PartialEq)]
pub(crate) struct OverlayVisibility {
    pub(crate) performance: bool,
    pub(crate) stats: bool,
}

impl OverlayVisibility {
    pub(crate) fn any_visible(self) -> bool {
        self.performance || self.stats
    }
}

#[derive(Clone)]
pub struct MetricsContext {
    pub(crate) visibility: Rc<Cell<OverlayVisibility>>,
    pub(crate) renderer: LaunchHandle,
}

impl MetricsContext {
    /// Get the metrics context.
    pub fn try_get() -> Option<Self> {
        try_consume_root_context()
    }

    pub fn is_performance_visible(&self) -> bool {
        self.visibility.get().performance
    }

    pub fn is_stats_visible(&self) -> bool {
        self.visibility.get().stats
    }

    pub fn set_performance_visible(&self, visible: bool) {
        self.set_visibility(OverlayVisibility {
            performance: visible,
            ..self.visibility.get()
        });
    }

    pub fn set_stats_visible(&self, visible: bool) {
        self.set_visibility(OverlayVisibility {
            stats: visible,
            ..self.visibility.get()
        });
    }

    pub(crate) fn set_visibility(&self, visibility: OverlayVisibility) {
        if self.visibility.replace(visibility) == visibility {
            return;
        }

        drop(self.renderer.post_callback(|renderer| {
            for window in renderer.windows().values() {
                window.window().request_redraw();
            }
        }));
    }
}
