use dioxus::prelude::*;
use freya_core::events::FreyaEvent;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::mouse::MouseButton;
use freya_testing::launch_test;

#[tokio::test]
async fn no_state() {
    fn no_state_app(cx: Scope) -> Element {
        render!(
            label {
                "Hello"
            }
        )
    }

    let mut utils = launch_test(no_state_app);

    assert_eq!(
        utils.root().child(0).unwrap().child(0).unwrap().text(),
        Some("Hello".to_string())
    );
}

#[tokio::test]
async fn with_state() {
    fn stateful_app(cx: Scope) -> Element {
        let state = use_state(cx, || false);
        let state_setter = state.setter();

        use_effect(cx, (), move |_| async move {
            state_setter(true);
        });

        render!(
            label {
                "Is enabled? {state}"
            }
        )
    }

    let mut utils = launch_test(stateful_app);

    let label = utils.root().child(0).unwrap();

    assert_eq!(
        label.child(0).unwrap().text(),
        Some("Is enabled? false".to_string())
    );

    utils.wait_for_update().await;

    assert_eq!(
        label.child(0).unwrap().text(),
        Some("Is enabled? true".to_string())
    );
}

#[tokio::test]
async fn check_size() {
    fn stateful_app(cx: Scope) -> Element {
        render!(rect {
            width: "50%",
            height: "calc(100% - 70)"
        })
    }

    let mut utils = launch_test(stateful_app);

    utils.wait_for_update().await;

    let rect = utils.root().child(0).unwrap();

    assert_eq!(rect.layout().unwrap().width(), 250.0);
    assert_eq!(rect.layout().unwrap().height(), 430.0);
}

#[tokio::test]
async fn simulate_events() {
    fn stateful_app(cx: Scope) -> Element {
        let enabled = use_state(cx, || false);
        render!(
            container {
                width: "100%",
                height: "100%",
                background: "red",
                direction: "both",
                onclick: |_| {
                    enabled.set(true);
                },
                label {
                    "Is enabled? {enabled}"
                }
            }
        )
    }

    let mut utils = launch_test(stateful_app);

    let rect = utils.root().child(0).unwrap();
    let label = rect.child(0).unwrap();

    // Render initial layout
    utils.wait_for_update().await;

    let text = label.child(0).unwrap();

    assert_eq!(text.text(), Some("Is enabled? false".to_string()));

    utils.push_event(FreyaEvent::Mouse {
        name: "click",
        cursor: (5.0, 5.0).into(),
        button: Some(MouseButton::Left),
    });

    // Render new layout after having it clicked
    utils.wait_for_update().await;

    let text = label.child(0).unwrap();

    assert_eq!(text.text(), Some("Is enabled? true".to_string()));
}
