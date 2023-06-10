use dioxus::prelude::*;
use freya_common::EventMessage;
use freya_core::FocusReceiver;
use freya_hooks::{use_platform, FocusId};

/// Use this to sync both the Focus shared state and the platform accessibility focus
#[allow(non_snake_case)]
pub fn AccessibilityFocusBridge(cx: Scope) -> Element {
    let platform = use_platform(cx);
    let focused_id = use_shared_state::<Option<FocusId>>(cx).unwrap();
    let current_focused_id = *focused_id.read();

    use_effect(cx, &(current_focused_id,), move |(focused_id,)| {
        if let Some(focused_id) = focused_id {
            platform
                .send(EventMessage::FocusAccessibilityNode(focused_id))
                .unwrap();
        }
        async move {}
    });

    use_effect(cx, (), {
        to_owned![focused_id];
        move |_| {
            let focus_id_listener = cx.consume_context::<FocusReceiver>();
            async move {
                let focus_id_listener = focus_id_listener.clone();
                if let Some(mut focus_id_listener) = focus_id_listener {
                    while focus_id_listener.changed().await.is_ok() {
                        *focused_id.write() = *focus_id_listener.borrow();
                    }
                }
            }
        }
    });

    None
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::{launch_test_with_config, FreyaEvent, MouseButton, TestingConfig};

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
            use_init_focus(cx);

            render!(
                AccessibilityFocusBridge { }
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
            TestingConfig::default().with_size((100.0, 100.0).into()),
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
