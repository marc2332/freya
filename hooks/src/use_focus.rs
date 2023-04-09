use dioxus_core::ScopeState;
use dioxus_hooks::{use_shared_state, use_shared_state_provider, UseSharedState};
use uuid::Uuid;

/// Subscribe and change the current focus.
pub fn use_focus(cx: &ScopeState) -> (bool, impl Fn() + '_) {
    let my_id = cx.use_hook(Uuid::new_v4);
    let focused_id = use_shared_state::<Uuid>(cx);

    let is_focused = Some(*my_id) == focused_id.map(|v| *v.read());

    let focus = move || {
        if let Some(focused_id) = focused_id {
            *focused_id.write() = *my_id;
        }
    };

    (is_focused, focus)
}

/// Subscribe and change the current focus but return the raw value and the caller focus ID.
pub fn use_raw_focus(cx: &ScopeState) -> (bool, Uuid, Option<UseSharedState<Uuid>>) {
    let my_id = cx.use_hook(Uuid::new_v4);
    let focused_id = use_shared_state::<Uuid>(cx);
    let focused = Some(*my_id) == focused_id.map(|v| *v.read());
    (focused, *my_id, focused_id)
}

/// Create a focus provider.
pub fn use_init_focus(cx: &ScopeState) {
    use_shared_state_provider(cx, Uuid::new_v4);
}

#[cfg(test)]
mod test {
    use crate::{use_focus, use_init_focus};
    use freya::prelude::*;
    use freya_testing::{launch_test_with_config, FreyaEvent, MouseButton, TestingConfig};

    #[tokio::test]
    pub async fn track_focus() {
        #[allow(non_snake_case)]
        fn OherChild(cx: Scope) -> Element {
            let (focused, focus) = use_focus(cx);

            render!(
                rect {
                    width: "100%",
                    height: "50%",
                    onclick: move |_| focus(),
                    "{focused}"
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

        let mut utils = launch_test_with_config(
            use_focus_app,
            TestingConfig::default().with_size((100.0, 100.0).into()),
        );

        // Initial state
        utils.wait_for_update().await;
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
            cursor: (5.0, 5.0).into(),
            button: Some(MouseButton::Left),
        });

        // First rect is now focused
        utils.wait_for_update().await;
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
            cursor: (5.0, 75.0).into(),
            button: Some(MouseButton::Left),
        });

        // Second rect is now focused
        utils.wait_for_update().await;
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
