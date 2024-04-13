use freya::prelude::*;
use freya_testing::prelude::*;
use torin::prelude::CursorPoint;
use winit::event::TouchPhase;

#[tokio::test]
pub async fn pointer_events_from_mouse() {
    fn pointer_events_app() -> Element {
        let mut state = use_signal(|| vec![]);

        let onpointerdown = move |_| state.push("down".to_string());

        let onpointerup = move |_| state.push("up".to_string());

        let onpointerover = move |_| state.push("over".to_string());

        let onpointerenter = move |_| state.push("enter".to_string());

        let onpointerleave = move |_| state.push("leave".to_string());

        let onglobalpointerup = move |_| state.push("globalup".to_string());

        rsx!(
            rect {
                height: "100%",
                width: "100%",
                padding: "10",
                rect {
                    height: "100%",
                    width: "100%",
                    onpointerdown,
                    onpointerup,
                    onpointerover,
                    onpointerenter,
                    onpointerleave,
                    onglobalpointerup,
                    label { "{state:?}" }
                }
            }
        )
    }

    let mut utils = launch_test(pointer_events_app);

    let root = utils.root().get(0);
    let rect = root.get(0);
    let label = rect.get(0);

    assert_eq!(label.get(0).text(), Some("[]"));

    // Moving the mouse for the first time will cause `mouseenter` and `mouseover` events
    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseOver,
        cursor: CursorPoint::new(100.0, 100.0),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "over"]).as_str())
    );

    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseDown,
        cursor: CursorPoint::new(100.0, 100.0),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "over", "down"]).as_str())
    );

    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: CursorPoint::new(100.0, 100.0),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "over", "down", "up"]).as_str())
    );

    utils.push_event(PlatformEvent::Mouse {
        name: EventName::MouseOver,
        cursor: CursorPoint::new(0.0, 0.0),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "over", "down", "up", "leave"]).as_str())
    );

    utils.push_event(PlatformEvent::Mouse {
        name: EventName::PointerUp,
        cursor: CursorPoint::new(0.0, 0.0),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(
            format!(
                "{:?}",
                vec!["enter", "over", "down", "up", "leave", "globalup"]
            )
            .as_str()
        )
    );
}

#[tokio::test]
pub async fn pointer_events_from_touch() {
    fn pointer_events_app() -> Element {
        let mut state = use_signal(|| vec![]);

        let onpointerdown = move |_| state.push("down".to_string());

        let onpointerup = move |_| state.push("up".to_string());

        let onpointerover = move |_| state.push("over".to_string());

        let onpointerenter = move |_| state.push("enter".to_string());

        rsx!(
            rect {
                height: "100%",
                width: "100%",
                padding: "10",
                rect {
                    height: "100%",
                    width: "100%",
                    onpointerdown: onpointerdown,
                    onpointerup: onpointerup,
                    onpointerover: onpointerover,
                    onpointerenter: onpointerenter,
                    label { "{state:?}" }
                }
            }
        )
    }

    let mut utils = launch_test(pointer_events_app);

    let root = utils.root().get(0);
    let rect = root.get(0);
    let label = rect.get(0);

    assert_eq!(label.get(0).text(), Some("[]"));

    utils.push_event(PlatformEvent::Touch {
        name: EventName::TouchMove,
        location: CursorPoint::new(100.0, 100.0),
        finger_id: 1,
        phase: TouchPhase::Moved,
        force: None,
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "over"]).as_str())
    );

    utils.push_event(PlatformEvent::Touch {
        name: EventName::TouchStart,
        location: CursorPoint::new(100.0, 100.0),
        finger_id: 1,
        phase: TouchPhase::Started,
        force: None,
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "over", "down"]).as_str())
    );

    utils.push_event(PlatformEvent::Touch {
        name: EventName::TouchEnd,
        location: CursorPoint::new(100.0, 100.0),
        finger_id: 1,
        phase: TouchPhase::Ended,
        force: None,
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "over", "down", "up"]).as_str())
    );
}
