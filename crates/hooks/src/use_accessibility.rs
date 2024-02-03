use std::rc::Rc;

use dioxus_core::ScopeState;
use freya_common::EventMessage;
use freya_core::{navigation_mode::NavigatorState, types::FocusReceiver};

use dioxus_hooks::{
    to_owned, use_context_provider, use_memo, use_shared_state, use_shared_state_provider, RefCell,
};
use freya_core::{accessibility::ACCESSIBILITY_ROOT_ID, types::AccessibilityId};

use crate::use_platform;

pub type AccessibilityIdCounter = Rc<RefCell<u64>>;

/// Sync both the Focus shared state and the platform accessibility focus
pub fn use_init_accessibility(cx: &ScopeState) {
    let platform = use_platform(cx);
    use_context_provider(cx, || Rc::new(RefCell::new(0u64)));
    use_shared_state_provider::<AccessibilityId>(cx, || ACCESSIBILITY_ROOT_ID);
    let focused_id = use_shared_state::<AccessibilityId>(cx).unwrap();

    let current_focused_id = *focused_id.read();

    // Notify the platform that a new Node has been focused manually
    let _ = use_memo(cx, &(current_focused_id,), move |(focused_id,)| {
        platform
            .send(EventMessage::FocusAccessibilityNode(focused_id))
            .unwrap();
    });

    cx.use_hook(|| {
        to_owned![focused_id];

        let focus_id_listener = cx.consume_context::<FocusReceiver>();
        let navigation_state = cx.consume_context::<NavigatorState>().unwrap();

        // Listen for focus changes
        cx.spawn({
            to_owned![focused_id];
            async move {
                let focus_id_listener = focus_id_listener.clone();
                if let Some(mut focus_id_listener) = focus_id_listener {
                    while focus_id_listener.changed().await.is_ok() {
                        *focused_id.write() = *focus_id_listener.borrow();
                    }
                }
            }
        });

        // Listen for navigation mode changes
        cx.spawn(async move {
            let mut getter = navigation_state.getter();
            while getter.changed().await.is_ok() {
                focused_id.notify_consumers();
            }
        });
    });
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_core::accessibility::ACCESSIBILITY_ROOT_ID;
    use freya_testing::{
        events::pointer::MouseButton, launch_test_with_config, FreyaEvent, TestingConfig,
    };

    #[tokio::test]
    pub async fn focus_accessibility() {
        #[allow(non_snake_case)]
        fn OherChild(cx: Scope) -> Element {
            let focus_manager = use_focus(cx);

            render!(rect {
                width: "100%",
                height: "50%",
                onclick: move |_| focus_manager.focus(),
            })
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
        assert_eq!(utils.focus_id(), ACCESSIBILITY_ROOT_ID);

        // Click on the first rect
        utils.push_event(FreyaEvent::Mouse {
            name: "click".to_string(),
            cursor: (5.0, 5.0).into(),
            button: Some(MouseButton::Left),
        });

        // First rect is now focused
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        let first_focus_id = utils.focus_id();
        assert_ne!(first_focus_id, ACCESSIBILITY_ROOT_ID);

        // Click on the second rect
        utils.push_event(FreyaEvent::Mouse {
            name: "click".to_string(),
            cursor: (5.0, 75.0).into(),
            button: Some(MouseButton::Left),
        });

        // Second rect is now focused
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        let second_focus_id = utils.focus_id();
        assert_ne!(first_focus_id, second_focus_id);
        assert_ne!(second_focus_id, ACCESSIBILITY_ROOT_ID);
    }
}
