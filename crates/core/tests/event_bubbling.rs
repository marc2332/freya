use freya::prelude::*;
use freya_testing::prelude::*;

#[tokio::test]
pub async fn captured_event() {
    fn app() -> Element {
        let mut state = use_signal(String::new);

        rsx!(
            rect {
                height: "100%",
                width: "100%",
                onclick: move |_: MouseEvent| {
                    state.set("1".to_string());
                },
                rect {
                    height: "100",
                    width: "100",
                    background: "red",
                    onclick: move |e: MouseEvent| {
                        state.set("2".to_string());
                        e.stop_propagation();
                    }
                }
                label {
                    "{state}"
                }
            }
        )
    }

    let mut utils = launch_test(app);

    let root = utils.root().get(0);
    let label = root.get(1);

    assert_eq!(label.get(0).text(), Some(""));

    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: CursorPoint::new(50.0, 50.0),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;

    assert_eq!(label.get(0).text(), Some("2"));
}

#[tokio::test]
pub async fn not_captured_event() {
    fn app() -> Element {
        let mut state = use_signal(String::new);

        rsx!(
            rect {
                height: "100%",
                width: "100%",
                onclick: move |_: MouseEvent| {
                    state.set("1".to_string());
                },
                rect {
                    height: "100",
                    width: "100",
                    background: "red",
                    onclick: move |_: MouseEvent| {
                        state.set("2".to_string());
                    }
                }
                label {
                    "{state}"
                }
            }
        )
    }

    let mut utils = launch_test(app);

    let root = utils.root().get(0);
    let label = root.get(1);

    assert_eq!(label.get(0).text(), Some(""));

    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: CursorPoint::new(50.0, 50.0),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;

    assert_eq!(label.get(0).text(), Some("1"));
}

#[tokio::test]
pub async fn event_gets_captured_at_wall() {
    fn app() -> Element {
        let mut state = use_signal(String::new);

        rsx!(
            rect {
                height: "200",
                width: "200",
                background: "blue",
                onclick: move |_: MouseEvent| {
                    state.set("1".to_string());
                },
                label {
                    "{state}"
                }
            }
            rect {
                position: "absolute",
                height: "100",
                width: "100",
                background: "red",
                layer: "-99",
                onclick: move |_: MouseEvent| {
                    state.set("2".to_string());
                }
            }
        )
    }

    let mut utils = launch_test(app);

    let root = utils.root().get(0);
    let label = root.get(0);

    assert_eq!(label.get(0).text(), Some(""));

    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: CursorPoint::new(50.0, 50.0),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;

    assert_eq!(label.get(0).text(), Some("2"));
}

#[tokio::test]
pub async fn event_cant_pass_through_wall() {
    fn app() -> Element {
        let mut state = use_signal(String::new);

        rsx!(
            rect {
                height: "200",
                width: "200",
                background: "blue",
                onclick: move |_: MouseEvent| {
                    state.set("1".to_string());
                },
                label {
                    "{state}"
                }
            }
            rect {
                position: "absolute",
                height: "100",
                width: "100",
                background: "red",
                layer: "-99",
            }
        )
    }

    let mut utils = launch_test(app);

    let root = utils.root().get(0);
    let label = root.get(0);

    assert_eq!(label.get(0).text(), Some(""));

    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: CursorPoint::new(50.0, 50.0),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;

    assert_eq!(label.get(0).text(), Some(""));
}
