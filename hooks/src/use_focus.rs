use std::num::NonZeroU128;

use dioxus_core::{AttributeValue, Scope, ScopeState};
use dioxus_hooks::{use_shared_state, use_shared_state_provider, UseSharedState};
use freya_common::EventMessage;
use freya_node_state::CustomAttributeValues;
use uuid::Uuid;
use winit::event_loop::EventLoopProxy;

use accesskit::NodeId as AccessibilityId;

pub type FocusId = AccessibilityId;

/// Manage the focus operations of given Node
#[derive(Clone, Copy)]
pub struct FocusManager<'a> {
    id: AccessibilityId,
    focused_id: Option<UseSharedState<'a, Option<AccessibilityId>>>,
}

impl FocusManager<'_> {
    /// Focus this node
    pub fn focus(&self) {
        if let Some(focused_id) = self.focused_id {
            *focused_id.write() = Some(self.id)
        }
    }

    /// Get the node focus ID
    pub fn id(&self) -> AccessibilityId {
        self.id
    }

    /// Create a node focus ID attribute
    pub fn attribute<'b>(&self, cx: Scope<'b>) -> AttributeValue<'b> {
        cx.any_value(CustomAttributeValues::FocusId(self.id))
    }

    /// Check if this node is currently focused
    pub fn is_focused(&self) -> bool {
        Some(Some(self.id)) == self.focused_id.map(|f| *f.read())
    }
}

/// Create a [`FocusManager`] for a component.
pub fn use_focus(cx: &ScopeState) -> FocusManager {
    let id = *cx.use_hook(|| AccessibilityId(NonZeroU128::new(Uuid::new_v4().as_u128()).unwrap()));
    let focused_id = use_shared_state::<Option<FocusId>>(cx);
    FocusManager { id, focused_id }
}

/// Create a focus provider.
pub fn use_init_focus(cx: &ScopeState) {
    use_shared_state_provider::<Option<FocusId>>(cx, || None);
}

/// Easily focus the given node focus ID
pub fn focus_node_id(cx: Scope, id: FocusId) {
    let proxy = cx.consume_context::<EventLoopProxy<EventMessage>>();
    if let Some(proxy) = &proxy {
        proxy
            .send_event(EventMessage::FocusAccessibilityNode(id))
            .unwrap();
    }
}

#[cfg(test)]
mod test {
    use crate::{use_focus, use_init_focus};
    use freya::prelude::*;
    use freya_testing::{launch_test, FreyaEvent, MouseButton};

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
            use_init_focus(cx);

            render!(
                rect {
                    width: "100%",
                    height: "100%",
                    OherChild {},
                    OherChild {}
                }
            )
        }

        let mut utils = launch_test(use_focus_app);

        // Initial state
        utils.wait_for_work((100.0, 100.0));
        let root = utils.root().child(0).unwrap();
        assert_eq!(
            root.child(0).unwrap().child(0).unwrap().text(),
            Some("false")
        );
        assert_eq!(
            root.child(1).unwrap().child(0).unwrap().text(),
            Some("false")
        );

        // Click on the first rect
        utils.push_event(FreyaEvent::Mouse {
            name: "click",
            cursor: (5.0, 5.0),
            button: Some(MouseButton::Left),
        });

        // First rect is now focused
        utils.wait_for_update((100.0, 100.0)).await;
        assert_eq!(
            root.child(0).unwrap().child(0).unwrap().text(),
            Some("true")
        );
        assert_eq!(
            root.child(1).unwrap().child(0).unwrap().text(),
            Some("false")
        );

        // Click on the second rect
        utils.push_event(FreyaEvent::Mouse {
            name: "click",
            cursor: (5.0, 75.0),
            button: Some(MouseButton::Left),
        });

        // Second rect is now focused
        utils.wait_for_update((100.0, 100.0)).await;
        assert_eq!(
            root.child(0).unwrap().child(0).unwrap().text(),
            Some("false")
        );
        assert_eq!(
            root.child(1).unwrap().child(0).unwrap().text(),
            Some("true")
        );
    }
}
