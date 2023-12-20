use std::num::NonZeroU128;

use accesskit::NodeId as AccessibilityId;
use dioxus_core::{AttributeValue, Scope, ScopeState};
use dioxus_hooks::{use_shared_state, use_shared_state_provider, UseSharedState};
use freya_core::navigation_mode::{NavigationMode, NavigatorState};
use freya_elements::events::{keyboard::Code, KeyboardEvent};
use freya_node_state::CustomAttributeValues;
use uuid::Uuid;

pub type FocusId = AccessibilityId;

/// Manage the focus operations of given Node
#[derive(Clone)]
pub struct UseFocus {
    id: AccessibilityId,
    focused_id: UseSharedState<Option<AccessibilityId>>,
    navigation_state: NavigatorState,
}

impl UseFocus {
    /// Focus this node
    pub fn focus(&self) {
        *self.focused_id.write() = Some(self.id)
    }

    /// Get the node focus ID
    pub fn id(&self) -> AccessibilityId {
        self.id
    }

    /// Create a node focus ID attribute
    pub fn attribute<'b, T>(&self, cx: Scope<'b, T>) -> AttributeValue<'b> {
        cx.any_value(CustomAttributeValues::FocusId(self.id))
    }

    /// Check if this node is currently focused
    pub fn is_focused(&self) -> bool {
        Some(self.id) == *self.focused_id.read()
    }

    /// Check if this node is currently selected
    pub fn is_selected(&self) -> bool {
        self.is_focused() && self.navigation_state.get() == NavigationMode::Keyboard
    }

    /// Unfocus the currently focused node.
    pub fn unfocus(&self) {
        *self.focused_id.write() = None;
    }

    /// Validate keydown event
    pub fn validate_keydown(&self, e: KeyboardEvent) -> bool {
        e.data.code == Code::Enter && self.is_selected()
    }
}

/// Create a focus manager for a node.
pub fn use_focus(cx: &ScopeState) -> &UseFocus {
    let focused_id = use_shared_state::<Option<FocusId>>(cx);

    cx.use_hook(move || {
        let focused_id = focused_id.unwrap().clone();
        let id = AccessibilityId(NonZeroU128::new(Uuid::new_v4().as_u128()).unwrap());
        let navigation_state = cx
            .consume_context::<NavigatorState>()
            .expect("This is not expected, and likely a bug. Please, report it.");
        UseFocus {
            id,
            focused_id,
            navigation_state,
        }
    })
}

/// Create a focus provider.
pub fn use_init_focus(cx: &ScopeState) {
    use_shared_state_provider::<Option<FocusId>>(cx, || None);
}

#[cfg(test)]
mod test {
    use crate::use_focus;
    use freya::prelude::*;
    use freya_testing::{launch_test_with_config, FreyaEvent, MouseButton, TestingConfig};

    #[tokio::test]
    pub async fn track_focus() {
        #[allow(non_snake_case)]
        fn OherChild(cx: Scope) -> Element {
            let focus_manager = use_focus(cx);

            render!(
                rect {
                    width: "100%",
                    height: "50%",
                    onclick: move |_| focus_manager.focus(),
                    "{focus_manager.is_focused()}"
                }
            )
        }

        fn use_focus_app(cx: Scope) -> Element {
            render!(
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
            *TestingConfig::default().with_size((100.0, 100.0).into()),
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
