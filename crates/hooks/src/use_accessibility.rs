use std::{cell::RefCell, rc::Rc};

use dioxus_core::{
    prelude::{consume_context, spawn, try_consume_context},
    use_hook,
};
use dioxus_hooks::{use_context_provider, use_effect};
use dioxus_signals::{Readable, Signal, Writable};
use freya_common::EventMessage;
use freya_core::{
    navigation_mode::{NavigationMode, NavigatorState},
    types::FocusReceiver,
};

use freya_core::{accessibility::ACCESSIBILITY_ROOT_ID, types::AccessibilityId};

use crate::use_platform;

pub type AccessibilityIdCounter = Rc<RefCell<u64>>;

#[derive(Clone)]
pub struct NavigationMark(bool);

impl NavigationMark {
    pub fn allowed(&self) -> bool {
        self.0
    }

    pub fn set_allowed(&mut self, allowed: bool) {
        self.0 = allowed;
    }
}

/// Sync both the Focus shared state and the platform accessibility focus
pub fn use_init_accessibility() -> Signal<NavigationMark> {
    let mut focused_id =
        use_context_provider::<Signal<AccessibilityId>>(|| Signal::new(ACCESSIBILITY_ROOT_ID));
    let mut navigation_mode =
        use_context_provider::<Signal<NavigationMode>>(|| Signal::new(NavigationMode::NotKeyboard));
    use_context_provider(|| Rc::new(RefCell::new(0u64)));
    let platform = use_platform();
    let navigation_mark = use_context_provider(|| Signal::new(NavigationMark(true)));

    // Tell the renderer the new focused node
    use_effect(move || {
        platform
            .send(EventMessage::FocusAccessibilityNode(*focused_id.read()))
            .unwrap();
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

    navigation_mark
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_core::{accessibility::ACCESSIBILITY_ROOT_ID, events::EventName};
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn focus_accessibility() {
        #[allow(non_snake_case)]
        fn OtherChild() -> Element {
            let mut focus_manager = use_focus();

            rsx!(rect {
                width: "100%",
                height: "50%",
                onclick: move |_| focus_manager.focus(),
            })
        }

        fn use_focus_app() -> Element {
            rsx!(
                rect {
                    width: "100%",
                    height: "100%",
                    OtherChild {},
                    OtherChild {}
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
        assert_eq!(utils.focus_id(), ACCESSIBILITY_ROOT_ID);

        // Click on the first rect
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (5.0, 5.0).into(),
            button: Some(MouseButton::Left),
        });

        // First rect is now focused
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        let first_focus_id = utils.focus_id();
        assert_ne!(first_focus_id, ACCESSIBILITY_ROOT_ID);

        // Click on the second rect
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (5.0, 75.0).into(),
            button: Some(MouseButton::Left),
        });

        // Second rect is now focused
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        let second_focus_id = utils.focus_id();
        assert_ne!(first_focus_id, second_focus_id);
        assert_ne!(second_focus_id, ACCESSIBILITY_ROOT_ID);
    }
}
