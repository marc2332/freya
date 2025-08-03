use accesskit_winit::Adapter;
use freya_core::{
    accessibility::{
        AccessibilityDirtyNodes,
        AccessibilityTree,
        ACCESSIBILITY_ROOT_ID,
    },
    dom::DioxusDOM,
    event_loop_messages::EventLoopMessage,
    states::AccessibilityState,
    types::{
        EventEmitter,
        NativePlatformSender,
    },
};
use freya_native_core::{
    prelude::NodeImmutable,
    NodeId,
};
use torin::torin::Torin;
use winit::{
    dpi::{
        LogicalPosition,
        LogicalSize,
    },
    event::WindowEvent,
    event_loop::{
        ActiveEventLoop,
        EventLoopProxy,
    },
    window::Window,
};

/// Manages the accessibility integration of Accesskit and Winit.
pub struct WinitAcessibilityTree {
    accessibility_tree: AccessibilityTree,
    accessibility_adapter: Adapter,
    adapter_initialized: bool,
}

impl WinitAcessibilityTree {
    pub fn new(
        event_loop: &ActiveEventLoop,
        window: &Window,
        proxy: EventLoopProxy<EventLoopMessage>,
    ) -> Self {
        let accessibility_tree = AccessibilityTree::new(ACCESSIBILITY_ROOT_ID);
        let accessibility_adapter = Adapter::with_event_loop_proxy(event_loop, window, proxy);
        Self {
            accessibility_tree,
            accessibility_adapter,
            adapter_initialized: false,
        }
    }

    pub fn focused_node_id(&self) -> Option<NodeId> {
        self.accessibility_tree.focused_node_id()
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
        let tree = self.accessibility_tree.init(rdom, layout, dirty_nodes);
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
        event_emitter: &EventEmitter,
    ) {
        let (tree, node_id) =
            self.accessibility_tree
                .process_updates(rdom, layout, dirty_nodes, event_emitter);

        // Notify the components
        platform_sender.send_modify(|state| {
            state.focused_accessibility_id = tree.focus;
            let node_ref = rdom.get(node_id).unwrap();
            let node_accessibility = node_ref.get::<AccessibilityState>().unwrap();
            let layout_node = layout.get(node_id).unwrap();
            state.focused_accessibility_node =
                AccessibilityTree::create_node(&node_ref, layout_node, &node_accessibility)
        });

        // Update the Window IME Position
        let layout_node = layout.get(node_id);
        if let Some(layout_node) = layout_node {
            let area = layout_node.visible_area();
            window.set_ime_cursor_area(
                LogicalPosition::new(area.min_x(), area.min_y()),
                LogicalSize::new(area.width(), area.height()),
            );
        } else {
            window.set_ime_cursor_area(
                window.inner_position().unwrap_or_default(),
                LogicalSize::<u32>::default(),
            );
        }

        if self.adapter_initialized {
            // Update the Adapter
            self.accessibility_adapter.update_if_active(|| tree);
        }
    }
}
