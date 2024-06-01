use std::{cell::RefCell, rc::Rc};

use dioxus_core::{
    prelude::{consume_context, provide_context, spawn},
    use_hook,
};
use dioxus_hooks::{use_context_provider, use_effect};
use dioxus_signals::{Readable, Signal, Writable};
use freya_common::EventMessage;
use freya_core::prelude::NativePlatformReceiver;

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

#[derive(Clone, Copy)]
pub struct UsePlatformEvents {
    pub navigation_mark: Signal<NavigationMark>,
}

/// Keep some native features (focused element, preferred theme, etc) on sync between the platform and the components
pub fn use_init_native_platform() -> UsePlatformEvents {
    // Init the Accessibility Node ID generator
    use_context_provider(|| Rc::new(RefCell::new(0u64)));

    // Init the NavigationMark signal
    let navigation_mark = use_context_provider(|| Signal::new(NavigationMark(true)));

    // Init the signals with platform values
    let focused_id = use_hook(|| {
        let mut platform_receiver = consume_context::<NativePlatformReceiver>();
        let platform_state = platform_receiver.borrow();

        let mut preferred_theme = Signal::new(platform_state.preferred_theme);
        let mut focused_id = Signal::new(platform_state.focused_id);
        let mut navigation_mode = Signal::new(platform_state.navigation_mode);
        let mut information = Signal::new(platform_state.information);

        drop(platform_state);

        // Listen for any changes during the execution of the app
        spawn(async move {
            while platform_receiver.changed().await.is_ok() {
                let state = platform_receiver.borrow();
                if *focused_id.peek() != state.focused_id {
                    *focused_id.write() = state.focused_id;
                }

                if *preferred_theme.peek() != state.preferred_theme {
                    *preferred_theme.write() = state.preferred_theme;
                }

                if *navigation_mode.peek() != state.navigation_mode {
                    *navigation_mode.write() = state.navigation_mode;
                }

                if *information.peek() != state.information {
                    *information.write() = state.information;
                }
            }
        });

        provide_context(preferred_theme);
        provide_context(navigation_mode);
        provide_context(information);
        provide_context(focused_id)
    });

    let platform = use_platform();

    // Tell the renderer the new focused node
    use_effect(move || {
        platform
            .send(EventMessage::FocusAccessibilityNode(*focused_id.read()))
            .unwrap();
    });

    UsePlatformEvents { navigation_mark }
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
