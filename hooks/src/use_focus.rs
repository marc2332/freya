use std::num::NonZeroU128;

use dioxus_core::{AttributeValue, Element, Scope, ScopeState};
use dioxus_hooks::{
    to_owned, use_effect, use_shared_state, use_shared_state_provider, use_state, UseSharedState,
};
use freya_common::EventMessage;
use freya_node_state::CustomAttributeValues;
use glutin::event_loop::EventLoopProxy;
use tokio::sync::watch;
use uuid::Uuid;

use accesskit::NodeId as NodeIdKit;

pub type FocusId = NodeIdKit;

#[derive(Clone, Copy)]
pub struct FocusManager<'a> {
    id: NodeIdKit,
    focused_id: Option<UseSharedState<'a, Option<NodeIdKit>>>,
}

impl FocusManager<'_> {
    pub fn focus(&self) {
        if let Some(focused_id) = self.focused_id {
            *focused_id.write() = Some(self.id)
        }
    }

    pub fn id(&self) -> NodeIdKit {
        self.id
    }

    pub fn attribute<'b>(&self, cx: Scope<'b>) -> AttributeValue<'b> {
        cx.any_value(CustomAttributeValues::FocusId(self.id))
    }

    pub fn is_focused(&self) -> bool {
        Some(Some(self.id)) == self.focused_id.map(|f| *f.read())
    }
}

pub fn use_focus(cx: &ScopeState) -> FocusManager {
    let id = *cx.use_hook(|| NodeIdKit(NonZeroU128::new(Uuid::new_v4().as_u128()).unwrap()));
    let focused_id = use_shared_state::<Option<FocusId>>(cx);
    FocusManager { id, focused_id }
}

/// Create a focus provider.
pub fn use_init_focus(cx: &ScopeState) {
    use_shared_state_provider::<Option<FocusId>>(cx, || None);
}

/// Propagate changes from the focus context to the renderer and viceversa
#[allow(non_snake_case)]
pub fn AccessibilityFocusProvider(cx: Scope) -> Element {
    let focused_id = use_shared_state::<Option<FocusId>>(cx).unwrap();
    let current_focused_id = *focused_id.read();
    let focus = use_state::<Option<FocusId>>(cx, || None);

    use_effect(cx, &(current_focused_id,), move |(focused_id,)| {
        if let Some(focused_id) = focused_id {
            let proxy = cx.consume_context::<EventLoopProxy<EventMessage>>();
            if let Some(proxy) = &proxy {
                proxy
                    .send_event(EventMessage::FocusAccessibilityNode(focused_id))
                    .unwrap();
            }
        }
        async move {}
    });

    use_effect(cx, (), {
        to_owned![focus];
        move |_| {
            let focus_id_listener = cx.consume_context::<watch::Receiver<Option<FocusId>>>();
            async move {
                let focus_id_listener = focus_id_listener.clone();
                if let Some(mut focus_id_listener) = focus_id_listener {
                    while focus_id_listener.changed().await.is_ok() {
                        focus.set(*focus_id_listener.borrow())
                    }
                }
            }
        }
    });

    use_effect(cx, (&focus.get().clone(),), move |(focus,)| {
        *focused_id.write() = focus;
        async move {}
    });

    None
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
        utils.wait_for_work((100.0, 100.0)).await;
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
        utils.send_event(FreyaEvent::Mouse {
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
        utils.send_event(FreyaEvent::Mouse {
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
