use crate::use_focus;
use freya::prelude::*;
use freya_testing::prelude::*;

#[tokio::test]
pub async fn track_focus() {
    #[allow(non_snake_case)]
    fn OherChild() -> Element {
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
                OherChild {},
                OherChild {}
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
