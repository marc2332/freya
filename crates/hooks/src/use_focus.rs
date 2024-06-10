use dioxus_core::{
    use_hook,
    AttributeValue,
};
use dioxus_hooks::{
    use_context,
    use_memo,
};
use dioxus_signals::{
    Memo,
    Readable,
    Signal,
    Writable,
};
use freya_common::EventMessage;
use freya_core::{
    accessibility::ACCESSIBILITY_ROOT_ID,
    platform_state::NavigationMode,
    types::AccessibilityId,
};
use freya_elements::events::{
    keyboard::Code,
    KeyboardEvent,
};
use freya_node_state::CustomAttributeValues;

use crate::{
    use_platform,
    AccessibilityIdCounter,
    NavigationMark,
    UsePlatform,
};

/// Manage the focus operations of given Node
#[derive(Clone, Copy)]
pub struct UseFocus {
    id: AccessibilityId,
    is_selected: Memo<bool>,
    is_focused: Memo<bool>,
    navigation_mode: Signal<NavigationMode>,
    navigation_mark: Signal<NavigationMark>,
    platform: UsePlatform,
}

impl UseFocus {
    /// Focus this node
    pub fn focus(&mut self) {
        if !*self.is_focused.peek() {
            self.platform
                .send(EventMessage::FocusAccessibilityNode(self.id))
                .ok();
        }
    }

    /// Queue a focus to this node
    pub fn queue_focus(&mut self) {
        if !*self.is_focused.peek() {
            self.platform
                .send(EventMessage::QueueFocusAccessibilityNode(self.id))
                .ok();
        }
    }

    /// Get the node focus ID
    pub fn id(&self) -> AccessibilityId {
        self.id
    }

    /// Create a node focus ID attribute
    pub fn attribute(&self) -> AttributeValue {
        AttributeValue::any_value(CustomAttributeValues::AccessibilityId(self.id))
    }

    /// Check if this node is currently focused
    pub fn is_focused(&self) -> bool {
        *self.is_focused.read()
    }

    /// Check if this node is currently selected
    pub fn is_selected(&self) -> bool {
        *self.is_selected.read() && *self.navigation_mode.read() == NavigationMode::Keyboard
    }

    /// Unfocus the currently focused node.
    pub fn unfocus(&mut self) {
        self.platform
            .send(EventMessage::FocusAccessibilityNode(ACCESSIBILITY_ROOT_ID))
            .ok();
    }

    /// Validate keydown event
    pub fn validate_keydown(&self, e: &KeyboardEvent) -> bool {
        e.data.code == Code::Enter && self.is_selected()
    }

    /// Prevent navigating the accessible nodes with the keyboard.
    /// You must use this this inside of a `onkeydown` event handler.
    pub fn prevent_navigation(&mut self) {
        self.navigation_mark.write().set_allowed(false);
    }
}

/// Create a focus manager for a node.
pub fn use_focus() -> UseFocus {
    let accessibility_id_counter = use_context::<AccessibilityIdCounter>();
    let focused_id = use_context::<Signal<AccessibilityId>>();
    let navigation_mode = use_context::<Signal<NavigationMode>>();
    let navigation_mark = use_context::<Signal<NavigationMark>>();
    let platform = use_platform();

    let id = use_hook(|| {
        let mut counter = accessibility_id_counter.borrow_mut();
        *counter += 1;
        AccessibilityId(*counter)
    });

    let is_focused = use_memo(move || id == *focused_id.read());

    let is_selected =
        use_memo(move || *is_focused.read() && *navigation_mode.read() == NavigationMode::Keyboard);

    use_hook(move || UseFocus {
        id,
        is_focused,
        is_selected,
        navigation_mode,
        navigation_mark,
        platform,
    })
}
