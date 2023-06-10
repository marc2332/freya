use accesskit::NodeId as AccessibilityId;
use accesskit_winit::Adapter;
use freya_common::EventMessage;
use freya_core::{
    prelude::{
        AccessibilityFocusDirection, AccessibilityProvider, AccessibilityState,
        SharedAccessibilityState, ROOT_ID,
    },
    FocusSender,
};
use winit::{event::WindowEvent, event_loop::EventLoopProxy, window::Window};

/// Manages the accessibility integration with Accesskit.
pub struct NativeAccessibility {
    accessibility_state: SharedAccessibilityState,
    accessibility_adapter: Adapter,
}

impl NativeAccessibility {
    pub fn new(window: &Window, proxy: EventLoopProxy<EventMessage>) -> Self {
        let title = window.title();
        let accessibility_state = AccessibilityState::new().wrap();
        let accessibility_adapter = {
            let accessibility_state = accessibility_state.clone();
            Adapter::new(
                window,
                move || {
                    let mut accessibility_state = accessibility_state.lock().unwrap();
                    accessibility_state.process(ROOT_ID, title.as_str())
                },
                proxy,
            )
        };
        Self {
            accessibility_state,
            accessibility_adapter,
        }
    }

    pub fn accessibility_state(&self) -> &SharedAccessibilityState {
        &self.accessibility_state
    }

    /// Focus a new accessibility node
    pub fn set_accessibility_focus(&mut self, id: AccessibilityId) {
        let tree = self
            .accessibility_state
            .lock()
            .unwrap()
            .set_focus_with_update(Some(id));
        if let Some(tree) = tree {
            self.accessibility_adapter.update(tree);
        }
    }

    /// Validate a winit event for accessibility
    pub fn on_accessibility_window_event(&mut self, window: &Window, event: &WindowEvent) -> bool {
        self.accessibility_adapter.on_event(window, event)
    }

    /// Remove the accessibility nodes
    pub fn clear_accessibility(&mut self) {
        self.accessibility_state.lock().unwrap().clear();
    }

    /// Process the accessibility nodes
    pub fn render_accessibility(&mut self, title: &str) {
        let tree = self
            .accessibility_state
            .lock()
            .unwrap()
            .process(ROOT_ID, title);
        self.accessibility_adapter.update(tree);
    }

    /// Focus the next accessibility node
    pub fn focus_next_node(
        &mut self,
        direction: AccessibilityFocusDirection,
        focus_sender: &FocusSender,
    ) {
        let tree = self
            .accessibility_state
            .lock()
            .unwrap()
            .set_focus_on_next_node(direction, focus_sender);
        if let Some(tree) = tree {
            self.accessibility_adapter.update(tree);
        }
    }
}
