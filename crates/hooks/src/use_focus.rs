use accesskit::NodeId as AccessibilityId;
use dioxus_core::{use_hook, AttributeValue};
use dioxus_hooks::{use_context, use_memo};
use dioxus_signals::{ReadOnlySignal, Readable, Signal, Writable};
use freya_core::{accessibility::ACCESSIBILITY_ROOT_ID, navigation_mode::NavigationMode};
use freya_elements::events::{keyboard::Code, KeyboardEvent};
use freya_node_state::CustomAttributeValues;

use crate::AccessibilityIdCounter;

/// Manage the focus operations of given Node
#[derive(Clone, Copy)]
pub struct UseFocus {
    id: AccessibilityId,
    is_selected: ReadOnlySignal<bool>,
    is_focused: ReadOnlySignal<bool>,
    focused_id: Signal<AccessibilityId>,
    navigation_mode: Signal<NavigationMode>,
}

impl UseFocus {
    /// Focus this node
    pub fn focus(&mut self) {
        if !*self.is_focused.peek() {
            *self.focused_id.write() = self.id
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
        *self.focused_id.write() = ACCESSIBILITY_ROOT_ID;
    }

    /// Validate keydown event
    pub fn validate_keydown(&self, e: KeyboardEvent) -> bool {
        e.data.code == Code::Enter && self.is_selected()
    }
}

/// Create a focus manager for a node.
pub fn use_focus() -> UseFocus {
    let accessibility_id_counter = use_context::<AccessibilityIdCounter>();
    let focused_id = use_context::<Signal<AccessibilityId>>();
    let navigation_mode = use_context::<Signal<NavigationMode>>();

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
        focused_id,
        is_focused,
        is_selected,
        navigation_mode,
    })
}
#[cfg(test)]
mod test {
    use crate::use_focus;
    use freya::prelude::*;
    use freya_testing::{
        events::pointer::MouseButton, launch_test_with_config, FreyaEvent, TestingConfig,
    };

    #[tokio::test]
    pub async fn track_focus() {
        #[allow(non_snake_case)]
        fn OherChild() -> Element {
            let mut focus_manager = use_focus();

            rsx!(
                rect {
                    width: "100%",
                    height: "50%",
                    onclick: move |_| focus_manager.focus(),
                    "{focus_manager.is_focused()}"
                }
            )
        }

        fn use_focus_app() -> Element {
            rsx!(
                rect {
                    width: "100%",
                    height: "100%",
                    OherChild {},
                    OherChild {}
                }
            )
        }

        let mut utils = launch_test_with_config(
            use_focus_app,
            TestingConfig {
                size: (100.0, 100.0).into(),
                ..TestingConfig::default()
            },
        );

        // Initial state
        utils.wait_for_update().await;
        let root = utils.root().get(0);
        assert_eq!(root.get(0).get(0).text(), Some("false"));
        assert_eq!(root.get(1).get(0).text(), Some("false"));

        // Click on the first rect
        utils.push_event(FreyaEvent::Mouse {
            name: "click".to_string(),
            cursor: (5.0, 5.0).into(),
            button: Some(MouseButton::Left),
        });

        // First rect is now focused
        utils.wait_for_update().await;
        assert_eq!(root.get(0).get(0).text(), Some("true"));
        assert_eq!(root.get(1).get(0).text(), Some("false"));

        // Click on the second rect
        utils.push_event(FreyaEvent::Mouse {
            name: "click".to_string(),
            cursor: (5.0, 75.0).into(),
            button: Some(MouseButton::Left),
        });

        // Second rect is now focused
        utils.wait_for_update().await;
        assert_eq!(root.get(0).get(0).text(), Some("false"));
        assert_eq!(root.get(1).get(0).text(), Some("true"));
    }
}
