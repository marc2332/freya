use freya::prelude::*;
use freya_testing::prelude::*;

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

    assert_eq!(rect.area().unwrap().width(), 250.0);
    assert_eq!(rect.area().unwrap().height(), 430.0);
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

    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (5.0, 5.0).into(),
        button: Some(MouseButton::Left),
    });

    // Render new layout after having it clicked
    utils.wait_for_update().await;

    let text = label.get(0);

    assert_eq!(text.text(), Some("Is enabled? true"));
}

#[tokio::test]
async fn match_by_text() {
    fn app() -> Element {
        rsx!(
            label {
                "Hello, World!"
            }
            rect {
                label {
                    "Hello, Rust!"
                }
            }
        )
    }

    let mut utils = launch_test(app);

    assert_eq!(
        utils.root().get_by_text("Hello, World!").unwrap().text(),
        Some("Hello, World!")
    );

    assert!(utils.root().get_by_text("Blabla").is_none());

    assert_eq!(
        utils.root().get_by_text("Hello, Rust!").unwrap().text(),
        Some("Hello, Rust!")
    );
}
