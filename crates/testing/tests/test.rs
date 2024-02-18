use dioxus::prelude::*;
use freya_core::events::FreyaEvent;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::mouse::MouseButton;
use freya_testing::launch_test;

#[tokio::test]
async fn no_state() {
    fn no_state_app() -> Element {
        rsx!(
            label {
                "Hello"
            }
        )
    }

    let mut utils = launch_test(no_state_app);

    assert_eq!(utils.root().get(0).get(0).text(), Some("Hello"));
}

#[tokio::test]
async fn with_state() {
    fn stateful_app() -> Element {
        let mut state = use_signal(|| false);

        use_effect(move || {
            state.set(true);
        });

        rsx!(
            label {
                "Is enabled? {state}"
            }
        )
    }

    let mut utils = launch_test(stateful_app);

    let label = utils.root().get(0);

    assert_eq!(label.get(0).text(), Some("Is enabled? false"));

    utils.wait_for_update().await;

    assert_eq!(label.get(0).text(), Some("Is enabled? true"));
}

#[tokio::test]
async fn check_size() {
    fn stateful_app() -> Element {
        rsx!(rect {
            width: "50%",
            height: "calc(100% - 70)"
        })
    }

    let mut utils = launch_test(stateful_app);

    utils.wait_for_update().await;

    let rect = utils.root().get(0);

    assert_eq!(rect.layout().unwrap().width(), 250.0);
    assert_eq!(rect.layout().unwrap().height(), 430.0);
}

#[tokio::test]
async fn simulate_events() {
    fn stateful_app() -> Element {
        let mut enabled = use_signal(|| false);
        rsx!(
            rect {
                overflow: "clip",
                width: "100%",
                height: "100%",
                background: "red",
                onclick: move |_| {
                    enabled.set(true);
                },
                label {
                    "Is enabled? {enabled}"
                }
            }
        )
    }

    let mut utils = launch_test(stateful_app);

    let rect = utils.root().get(0);
    let label = rect.get(0);

    // Render initial layout
    utils.wait_for_update().await;

    let text = label.get(0);

    assert_eq!(text.text(), Some("Is enabled? false"));

    utils.push_event(FreyaEvent::Mouse {
        name: "click".to_string(),
        cursor: (5.0, 5.0).into(),
        button: Some(MouseButton::Left),
    });

    // Render new layout after having it clicked
    utils.wait_for_update().await;

    let text = label.get(0);

    assert_eq!(text.text(), Some("Is enabled? true"));
}
