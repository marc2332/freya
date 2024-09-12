use std::sync::{
    Arc,
    Mutex,
};

use accesskit_winit::Adapter;
use freya_common::AccessibilityDirtyNodes;
use freya_core::{
    dom::DioxusDOM,
    prelude::{
        AccessibilityFocusStrategy,
        AccessibilityTree,
        EventMessage,
        SharedAccessibilityTree,
        ACCESSIBILITY_ROOT_ID,
    },
    types::{
        AccessibilityId,
        NativePlatformSender,
    },
};
use freya_native_core::NodeId;
use torin::torin::Torin;
use winit::{
    dpi::{
        LogicalPosition,
        LogicalSize,
    },
    event::WindowEvent,
    event_loop::EventLoopProxy,
    window::Window,
};

/// Manages the accessibility integration with Accesskit.
pub struct AccessKitManager {
    accessibility_tree: SharedAccessibilityTree,
    accessibility_adapter: Adapter,
    adapter_initialized: bool,
}

impl AccessKitManager {
    pub fn new(window: &Window, proxy: EventLoopProxy<EventMessage>) -> Self {
        let accessibility_tree =
            Arc::new(Mutex::new(AccessibilityTree::new(ACCESSIBILITY_ROOT_ID)));
        let accessibility_adapter = Adapter::with_event_loop_proxy(window, proxy);
        Self {
            accessibility_tree,
            accessibility_adapter,
            adapter_initialized: false,
        }
    }

    /// Process an accessibility window event
    pub fn process_accessibility_event(&mut self, event: &WindowEvent, window: &Window) {
        self.accessibility_adapter.process_event(window, event)
    }

    /// Initialize the Accessibility Tree and update the adapter
    pub fn init_accessibility(
        &mut self,
        rdom: &DioxusDOM,
        layout: &Torin<NodeId>,
        dirty_nodes: &mut AccessibilityDirtyNodes,
    ) {
        let tree = self
            .accessibility_tree
            .lock()
            .unwrap()
            .init(rdom, layout, dirty_nodes);
        self.accessibility_adapter.update_if_active(|| {
            self.adapter_initialized = true;
            tree
        });
    }

    /// Process any pending accessibility tree update and update the adapter
    pub fn process_updates(
        &mut self,
        rdom: &DioxusDOM,
        layout: &Torin<NodeId>,
        platform_sender: &NativePlatformSender,
        window: &Window,
        dirty_nodes: &mut AccessibilityDirtyNodes,
    ) {
        let (tree, node_id) =
            self.accessibility_tree
                .lock()
                .unwrap()
                .process_updates(rdom, layout, dirty_nodes);

        // Notify the components
        platform_sender.send_modify(|state| {
            state.focused_id = tree.focus;
        });

        // Update the IME Cursor area
        self.update_ime_position(node_id, window, layout);

        if self.adapter_initialized {
            // Update the Adapter
            self.accessibility_adapter.update_if_active(|| tree);
        }
    }

    /// Focus the next accessibility node
    pub fn focus_next_node(
        &mut self,
        rdom: &DioxusDOM,
        direction: AccessibilityFocusStrategy,
        platform_sender: &NativePlatformSender,
        window: &Window,
        layout: &Torin<NodeId>,
    ) {
        let (tree, node_id) = self
            .accessibility_tree
            .lock()
            .unwrap()
            .set_focus_on_next_node(direction, rdom);

        // Notify the components
        platform_sender.send_modify(|state| {
            state.focused_id = tree.focus;
        });

        // Update the IME Cursor area
        self.update_ime_position(node_id, window, layout);

        if self.adapter_initialized {
            // Update the Adapter
            self.accessibility_adapter.update_if_active(|| tree);
        }
    }

    /// Focus a new accessibility node
    pub fn focus_node(
        &mut self,
        id: AccessibilityId,
        platform_sender: &NativePlatformSender,
        window: &Window,
        layout: &Torin<NodeId>,
    ) {
        let res = self
            .accessibility_tree
            .lock()
            .unwrap()
            .set_focus_with_update(id);

        if let Some((tree, node_id)) = res {
            // Notify the components
            platform_sender.send_modify(|state| {
                state.focused_id = tree.focus;
            });

            // Update the IME Cursor area
            self.update_ime_position(node_id, window, layout);

            if self.adapter_initialized {
                // Update the Adapter
                self.accessibility_adapter.update_if_active(|| tree);
            }
        }
    }

    /// Update the Window IME Position with the bounds of the currently focused accessibility node
    fn update_ime_position(&self, node_id: NodeId, window: &Window, layout: &Torin<NodeId>) {
        let layout_node = layout.get(node_id);
        if let Some(layout_node) = layout_node {
            let area = layout_node.visible_area();
            return window.set_ime_cursor_area(
                LogicalPosition::new(area.min_x(), area.min_y()),
                LogicalSize::new(area.width(), area.height()),
            );
        }

        window.set_ime_cursor_area(
            window.inner_position().unwrap_or_default(),
            LogicalSize::<u32>::default(),
        );
    }
}
