use crate::use_focus;
use freya::prelude::*;
use freya_testing::prelude::*;

#[tokio::test]
pub async fn track_focus() {
    #[allow(non_snake_case)]
    fn OtherChild() -> Element {
        let mut focus_manager = use_focus();

        rsx!(
            rect {
                width: "100%",
                height: "50%",
                onclick: move |_| focus_manager.focus(),
                label {
                    "{focus_manager.is_focused()}"
                }
            }
        )
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
    let root = utils.root().get(0);
    assert_eq!(root.get(0).get(0).get(0).text(), Some("false"));
    assert_eq!(root.get(1).get(0).get(0).text(), Some("false"));

    // Click on the first rect
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (5.0, 5.0).into(),
        button: Some(MouseButton::Left),
    });

    // First rect is now focused
    utils.wait_for_update().await;
    assert_eq!(root.get(0).get(0).get(0).text(), Some("true"));
    assert_eq!(root.get(1).get(0).get(0).text(), Some("false"));

    // Click on the second rect
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (5.0, 75.0).into(),
        button: Some(MouseButton::Left),
    });

    // Second rect is now focused
    utils.wait_for_update().await;
    utils.wait_for_update().await;
    assert_eq!(root.get(0).get(0).get(0).text(), Some("false"));
    assert_eq!(root.get(1).get(0).get(0).text(), Some("true"));
}

#[tokio::test]
pub async fn block_focus() {
    #[allow(non_snake_case)]
    fn Child() -> Element {
        let mut focus_manager = use_focus();

        rsx!(
            rect {
                focus_id: focus_manager.attribute(),
                width: "100%",
                height: "50%",
                onclick: move |_| focus_manager.focus(),
                label {
                    "{focus_manager.is_focused()}"
                }
            }
        )
    }

    #[allow(non_snake_case)]
    fn BlockingChild() -> Element {
        let mut focus_manager = use_focus();

        rsx!(
            rect {
                focus_id: focus_manager.attribute(),
                width: "100%",
                height: "50%",
                onkeydown: move |_| {
                    if focus_manager.is_focused() {
                        focus_manager.prevent_navigation();
                    }
                },
                onclick: move |_| focus_manager.focus(),
                label {
                    "{focus_manager.is_focused()}"
                }
            }
        )
    }

    fn use_focus_app() -> Element {
        rsx!(
            rect {
                width: "100%",
                height: "100%",
                Child {},
                BlockingChild {}
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
    let root = utils.root().get(0);
    assert_eq!(root.get(0).get(0).get(0).text(), Some("false"));
    assert_eq!(root.get(1).get(0).get(0).text(), Some("false"));

    // Click on the first rect
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (5.0, 5.0).into(),
        button: Some(MouseButton::Left),
    });

    // First rect is now focused
    utils.wait_for_update().await;
    assert_eq!(root.get(0).get(0).get(0).text(), Some("true"));
    assert_eq!(root.get(1).get(0).get(0).text(), Some("false"));

    // Navigate to the second rect
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        key: Key::Tab,
        code: Code::Tab,
        modifiers: Modifiers::default(),
    });

    // Second rect is now focused
    utils.wait_for_update().await;
    utils.wait_for_update().await;
    assert_eq!(root.get(0).get(0).get(0).text(), Some("false"));
    assert_eq!(root.get(1).get(0).get(0).text(), Some("true"));

    // Try to navigate to the first rect again
    utils.push_event(PlatformEvent::Keyboard {
        name: EventName::KeyDown,
        key: Key::Tab,
        code: Code::Tab,
        modifiers: Modifiers::default(),
    });

    // Second rect is still focused
    utils.wait_for_update().await;
    utils.wait_for_update().await;
    assert_eq!(root.get(0).get(0).get(0).text(), Some("false"));
    assert_eq!(root.get(1).get(0).get(0).text(), Some("true"));
}
