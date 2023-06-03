use dioxus::prelude::*;
use freya_core::events::FreyaEvent;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::PointerEvent;
use freya_testing::{launch_test, MouseButton};
use torin::prelude::CursorPoint;

#[tokio::test]
pub async fn pointer_events() {
    fn pointer_events_app(cx: Scope) -> Element {
        let state = use_state(cx, || vec![]);

        let onpointerdown = move |ev: PointerEvent| state.with_mut(|v| v.push("down".to_string()));

        let onpointerup = move |ev: PointerEvent| state.with_mut(|v| v.push("up".to_string()));

        let onpointerover = move |ev: PointerEvent| state.with_mut(|v| v.push("over".to_string()));

        let onpointerenter =
            move |ev: PointerEvent| state.with_mut(|v| v.push("enter".to_string()));

        let onpointerleave =
            move |ev: PointerEvent| state.with_mut(|v| v.push("leave".to_string()));

        render!(
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
                    onpointerleave: onpointerleave,
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
    utils.push_event(FreyaEvent::Mouse {
        name: "mouseover".to_string(),
        cursor: CursorPoint::new(100.0, 100.0),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "over"]).as_str())
    );

    utils.push_event(FreyaEvent::Mouse {
        name: "mousedown".to_string(),
        cursor: CursorPoint::new(100.0, 100.0),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "over", "down"]).as_str())
    );

    utils.push_event(FreyaEvent::Mouse {
        name: "click".to_string(),
        cursor: CursorPoint::new(100.0, 100.0),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "over", "down", "up"]).as_str())
    );

    utils.push_event(FreyaEvent::Mouse {
        name: "mouseover".to_string(),
        cursor: CursorPoint::new(0.0, 0.0),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "over", "down", "up", "leave"]).as_str())
    );
}
