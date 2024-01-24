use dioxus_core::{
    prelude::{consume_context, spawn, try_consume_context},
    use_hook,
};
use dioxus_hooks::use_context;
use dioxus_signals::{use_memo, Readable, Signal, Writable};
use freya_common::EventMessage;
use freya_core::{
    navigation_mode::{NavigationMode, NavigatorState},
    types::FocusReceiver,
};

use crate::{use_platform, FocusId};

/// Sync both the Focus shared state and the platform accessibility focus
pub fn use_init_accessibility() {
    let platform = use_platform();
    let focused_id = use_context::<Signal<Option<FocusId>>>();
    let navigation_mode = use_context::<Signal<NavigationMode>>();

    // Tell the renderer the new focused node
    let _ = use_memo(move || {
        if let Some(focused_id) = *focused_id.read() {
            platform
                .send(EventMessage::FocusAccessibilityNode(focused_id))
                .unwrap();
        }
    });

    use_hook(|| {
        let focus_id_listener = try_consume_context::<FocusReceiver>();
        let navigation_state = consume_context::<NavigatorState>();

        // Listen for focus changes
        spawn(async move {
            let focus_id_listener = focus_id_listener.clone();
            if let Some(mut focus_id_listener) = focus_id_listener {
                while focus_id_listener.changed().await.is_ok() {
                    *focused_id.write() = *focus_id_listener.borrow();
                }
            }
        });

        // Listen for navigation mode changes
        spawn(async move {
            let mut getter = navigation_state.getter();
            while getter.changed().await.is_ok() {
                *navigation_mode.write() = *getter.borrow();
            }
        });
    });
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::{
        events::pointer::MouseButton, launch_test_with_config, FreyaEvent, TestingConfig,
    };

    #[tokio::test]
    pub async fn focus_accessibility() {
        #[allow(non_snake_case)]
        fn OherChild(cx: Scope) -> Element {
            let focus_manager = use_focus(cx);

            rsx!(rect {
                width: "100%",
                height: "50%",
                onclick: move |_| focus_manager.focus(),
            })
        }

        fn use_focus_app(cx: Scope) -> Element {
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
            *TestingConfig::default().with_size((100.0, 100.0).into()),
        );

        // Initial state
        utils.wait_for_update().await;
        assert!(utils.focus_id().is_none());

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
        assert!(first_focus_id.is_some());

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
        assert!(second_focus_id.is_some());
    }
}
