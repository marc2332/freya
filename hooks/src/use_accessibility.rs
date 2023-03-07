use std::num::NonZeroU128;

use accesskit::NodeId as NodeIdKit;
use dioxus_core::{AttributeValue, ScopeState};
use freya_common::EventMessage;
use freya_node_state::CustomAttributeValues;
use glutin::event_loop::EventLoopProxy;
use uuid::Uuid;

pub fn use_accessibility(cx: &ScopeState) -> (NodeIdKit, AttributeValue) {
    let id = *cx.use_hook(|| NodeIdKit(NonZeroU128::new(Uuid::new_v4().as_u128()).unwrap()));
    let attr = cx.any_value(CustomAttributeValues::AccessibilityId(id));

    (id, attr)
}

pub fn use_focus_accessibility(cx: &ScopeState) -> impl Fn(NodeIdKit) + '_ {
    |id: NodeIdKit| {
        let proxy = cx.consume_context::<EventLoopProxy<EventMessage>>();
        if let Some(proxy) = &proxy {
            proxy
                .send_event(EventMessage::FocusAccessibilityNode(id))
                .unwrap();
        }
    }
}
