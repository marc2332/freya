use std::sync::Arc;

use dioxus_core::{
    prelude::consume_context,
    use_hook,
    AttributeValue,
};
use dioxus_hooks::{
    use_context,
    use_memo,
};
use dioxus_signals::{
    Memo,
    ReadOnlySignal,
    Readable,
    Signal,
    Writable,
};
use freya_common::{
    AccessibilityFocusStrategy,
    AccessibilityGenerator,
};
use freya_core::{
    accessibility::ACCESSIBILITY_ROOT_ID,
    platform_state::NavigationMode,
    prelude::EventMessage,
    types::{
        AccessibilityId,
        AccessibilityNode,
    },
};
use freya_elements::events::{
    keyboard::Code,
    KeyboardEvent,
};
use freya_node_state::CustomAttributeValues;

use crate::{
    use_platform,
    NavigationMark,
    UsePlatform,
};

/// Manage the focus operations of given Node
#[derive(Clone, Copy)]
pub struct UseFocus {
    id: AccessibilityId,
    is_focused_with_keyboard: Memo<bool>,
    is_focused: Memo<bool>,
    navigation_mode: Signal<NavigationMode>,
    navigation_mark: Signal<NavigationMark>,
    platform: UsePlatform,
    focused_id: Signal<AccessibilityId>,
    focused_node: Signal<AccessibilityNode>,
}

impl UseFocus {
    pub fn new_id() -> AccessibilityId {
        let accessibility_generator = consume_context::<Arc<AccessibilityGenerator>>();

        AccessibilityId(accessibility_generator.new_id())
    }

    /// Focus this node
    pub fn focus(&mut self) {
        if !*self.is_focused.peek() {
            self.platform
                .focus(AccessibilityFocusStrategy::Node(self.id));
        }
    }

    /// Get the node focus ID
    pub fn id(&self) -> AccessibilityId {
        self.id
    }

    /// Create a node focus ID attribute
    pub fn attribute(&self) -> AttributeValue {
        Self::attribute_for_id(self.id)
    }

    /// Create a node focus ID attribute
    pub fn attribute_for_id(id: AccessibilityId) -> AttributeValue {
        AttributeValue::any_value(CustomAttributeValues::AccessibilityId(id))
    }

    /// Check if this node is currently focused
    pub fn is_focused(&self) -> bool {
        *self.is_focused.read()
    }

    /// Check if this node is currently selected
    pub fn is_focused_with_keyboard(&self) -> bool {
        *self.is_focused_with_keyboard.read()
            && *self.navigation_mode.read() == NavigationMode::Keyboard
    }

    /// Unfocus the currently focused node.
    pub fn unfocus(&mut self) {
        self.platform
            .send(EventMessage::FocusAccessibilityNode(
                AccessibilityFocusStrategy::Node(ACCESSIBILITY_ROOT_ID),
            ))
            .ok();
    }

    /// Validate a `keydown` event.
    pub fn validate_keydown(&self, e: &KeyboardEvent) -> bool {
        (e.data.code == Code::Enter || e.data.code == Code::Space)
            && self.is_focused_with_keyboard()
    }

    /// Prevent navigating the accessible nodes with the keyboard.
    /// You must use this this inside of a `onglobalkeydown` event handler.
    pub fn prevent_navigation(&mut self) {
        self.navigation_mark.write().set_allowed(false);
    }

    /// Get a readable of the currently focused Node Id.
    pub fn focused_id(&self) -> ReadOnlySignal<AccessibilityId> {
        self.focused_id.into()
    }

    /// Get a readable of the currently focused Node.
    pub fn focused_node(&self) -> ReadOnlySignal<AccessibilityNode> {
        self.focused_node.into()
    }
}

/// Create a focus manager for a node.
pub fn use_focus() -> UseFocus {
    let id = use_hook(UseFocus::new_id);

    use_focus_from_id(id)
}

/// Create a focus manager for a node with the provided [AccessibilityId].
pub fn use_focus_from_id(id: AccessibilityId) -> UseFocus {
    let focused_id = use_context::<Signal<AccessibilityId>>();
    let focused_node = use_context::<Signal<AccessibilityNode>>();
    let navigation_mode = use_context::<Signal<NavigationMode>>();
    let navigation_mark = use_context::<Signal<NavigationMark>>();
    let platform = use_platform();

    let is_focused = use_memo(move || id == *focused_id.read());

    let is_focused_with_keyboard =
        use_memo(move || *is_focused.read() && *navigation_mode.read() == NavigationMode::Keyboard);

    use_hook(move || UseFocus {
        id,
        is_focused,
        is_focused_with_keyboard,
        navigation_mode,
        navigation_mark,
        platform,
        focused_id,
        focused_node,
    })
}
