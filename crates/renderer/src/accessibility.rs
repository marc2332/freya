use accesskit::NodeId as AccessibilityId;
use accesskit_winit::Adapter;
use freya_common::EventMessage;
use freya_core::{
    prelude::{
        AccessibilityFocusDirection, AccessibilityProvider, AccessibilityState,
        SharedAccessibilityState, ACCESSIBILITY_ROOT_ID,
    },
    types::FocusSender,
};
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event::WindowEvent,
    event_loop::EventLoopProxy,
    window::Window,
};

/// Manages the accessibility integration with Accesskit.
pub struct NativeAccessibility {
    accessibility_state: SharedAccessibilityState,
    accessibility_adapter: Adapter,
}

impl NativeAccessibility {
    pub fn new(window: &Window, proxy: EventLoopProxy<EventMessage>) -> Self {
        let title = window.title();
        let accessibility_state = AccessibilityState::new(ACCESSIBILITY_ROOT_ID).wrap();
        let accessibility_adapter = {
            let accessibility_state = accessibility_state.clone();
            Adapter::new(
                window,
                move || {
                    let mut accessibility_state = accessibility_state.lock().unwrap();
                    accessibility_state.process(ACCESSIBILITY_ROOT_ID, title.as_str())
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
    pub fn set_accessibility_focus(&self, id: AccessibilityId, window: &Window) {
        let tree = self
            .accessibility_state
            .lock()
            .unwrap()
            .set_focus_with_update(id);
        if let Some(tree) = tree {
            // Update the IME Cursor area
            self.update_ime_position(tree.focus, window);

            // Update the adapter
            self.accessibility_adapter.update_if_active(|| tree);
        }
    }

    fn update_ime_position(&self, accessibility_id: AccessibilityId, window: &Window) {
        let accessibility_state = self.accessibility_state.lock().unwrap();
        let node = accessibility_state
            .nodes()
            .find_map(|(id, n)| {
                if *id == accessibility_id {
                    Some(n)
                } else {
                    None
                }
            })
            .unwrap();
        let node_bounds = node.bounds();
        if let Some(node_bounds) = node_bounds {
            window.set_ime_cursor_area(
                LogicalPosition::new(node_bounds.min_x(), node_bounds.min_y()),
                LogicalSize::new(node_bounds.width(), node_bounds.height()),
            )
        }
    }

    /// Process an Accessibility event
    pub fn process_accessibility_event(&mut self, window: &Window, event: &WindowEvent) {
        self.accessibility_adapter.process_event(window, event)
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
            .process(ACCESSIBILITY_ROOT_ID, title);
        self.accessibility_adapter.update_if_active(|| tree);
    }

    /// Focus the next accessibility node
    pub fn focus_next_node(
        &self,
        direction: AccessibilityFocusDirection,
        focus_sender: &FocusSender,
        window: &Window,
    ) {
        let tree = self
            .accessibility_state
            .lock()
            .unwrap()
            .set_focus_on_next_node(direction);

        // Update the App
        focus_sender
            .send(tree.focus)
            .expect("Failed to focus the Node.");

        // Update the IME Cursor area
        self.update_ime_position(tree.focus, window);

        // Update the Adapter
        self.accessibility_adapter.update_if_active(|| tree);
    }
}
