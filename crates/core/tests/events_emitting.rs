use std::{
    path::PathBuf,
    str::FromStr,
};

use freya::prelude::*;
use freya_engine::prelude::Color;
use freya_testing::prelude::*;

#[tokio::test]
pub async fn click_event_overlay_with_transparent_layer() {
    fn app() -> Element {
        let mut state = use_signal(Vec::new);

        rsx!(
            rect {
                height: "100%",
                width: "100%",
                rect {
                    height: "100",
                    width: "200",
                    background: "red",
                    onclick: move |e: MouseEvent| {
                        state.push(1);
                        e.stop_propagation();
                    }
                }
                rect {
                    position: "absolute",
                    height: "200",
                    width: "100",
                }
                label {
                    "{state:?}"
                }
            }
        )
    }

    let mut utils = launch_test(app);

    let root = utils.root().get(0);
    let label = root.get(2);

    assert_eq!(label.get(0).text(), Some("[]"));

    utils.click_cursor((50., 50.)).await;

    assert_eq!(label.get(0).text(), Some("[1]"));
}

#[tokio::test]
pub async fn click_event_overlay_with_solid_layer() {
    fn app() -> Element {
        let mut state = use_signal(Vec::new);

        rsx!(
            rect {
                height: "100%",
                width: "100%",
                rect {
                    height: "100",
                    width: "200",
                    background: "red",
                    onclick: move |e: MouseEvent| {
                        state.push(1);
                        e.stop_propagation();
                    }
                }
                rect {
                    position: "absolute",
                    height: "200",
                    width: "100",
                    background: "blue"
                }
                label {
                    "{state:?}"
                }
            }
        )
    }

    let mut utils = launch_test(app);

    let root = utils.root().get(0);
    let label = root.get(2);

    assert_eq!(label.get(0).text(), Some("[]"));

    utils.click_cursor((50., 50.)).await;

    assert_eq!(label.get(0).text(), Some("[]"));
}

#[tokio::test]
pub async fn mouseenter_event_overlay_with_transparent_layer() {
    fn app() -> Element {
        let mut state = use_signal(Vec::new);

        rsx!(
            rect {
                height: "100%",
                width: "100%",
                rect {
                    height: "100",
                    width: "200",
                    background: "red",
                    onmouseenter: move |e: MouseEvent| {
                        state.push(1);
                        e.stop_propagation();
                    },
                    onmouseleave: move |e: MouseEvent| {
                        state.push(2);
                        e.stop_propagation();
                    }
                }
                rect {
                    position: "absolute",
                    height: "200",
                    width: "100",
                    onmouseenter: move |e: MouseEvent| {
                        state.push(3);
                        e.stop_propagation();
                    },
                    onmouseleave: move |e: MouseEvent| {
                        state.push(4);
                        e.stop_propagation();
                    }
                }
                label {
                    "{state:?}"
                }
            }
        )
    }

    let mut utils = launch_test(app);

    let root = utils.root().get(0);
    let label = root.get(2);

    assert_eq!(label.get(0).text(), Some("[]"));

    utils.move_cursor((50., 50.)).await;

    assert_eq!(label.get(0).text(), Some("[3, 1]"));

    utils.move_cursor((150., 50.)).await;

    assert_eq!(label.get(0).text(), Some("[3, 1, 4]"));

    utils.move_cursor((150., 150.)).await;

    assert_eq!(label.get(0).text(), Some("[3, 1, 4, 2]"));
}

#[tokio::test]
pub async fn global_events() {
    fn app() -> Element {
        let mut state = use_signal(Vec::new);

        rsx!(
            rect {
                height: "100%",
                width: "100%",
                onglobalmousemove: move |_e: MouseEvent| {
                    state.push(1);
                },
                onglobalclick: move |_e: MouseEvent| {
                    state.push(2);
                },
                label {
                    "{state:?}"
                }
            }
        )
    }

    let mut utils = launch_test(app);

    let root = utils.root().get(0);
    let label = root.get(0);

    assert_eq!(label.get(0).text(), Some("[]"));

    utils.move_cursor((50., 50.)).await;

    assert_eq!(label.get(0).text(), Some("[1]"));

    utils.click_cursor((50., 50.)).await;

    assert_eq!(label.get(0).text(), Some("[1, 2]"));
}

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

    utils.click_cursor((50., 50.)).await;

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

    utils.click_cursor((50., 50.)).await;

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

    utils.click_cursor((50., 50.)).await;

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

    utils.click_cursor((50., 50.)).await;

    assert_eq!(label.get(0).text(), Some(""));
}

#[tokio::test]
pub async fn pointer_events_from_mouse() {
    fn pointer_events_app() -> Element {
        let mut state = use_signal(std::vec::Vec::new);

        let onpointerdown = move |_| state.push("down".to_string());

        let onpointerup = move |_| state.push("up".to_string());

        let onpointermove = move |_| state.push("move".to_string());

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
                    onpointermove,
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

    // Moving the mouse for the first time will cause `mouseenter` and `mousemove` events
    utils.move_cursor((100., 100.)).await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "move"]).as_str())
    );

    utils.move_cursor((101., 100.)).await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "move", "move"]).as_str())
    );

    utils.push_event(TestEvent::Mouse {
        name: EventName::MouseDown,
        cursor: CursorPoint::new(100.0, 100.0),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "move", "move", "down"]).as_str())
    );

    utils.push_event(TestEvent::Mouse {
        name: EventName::MouseUp,
        cursor: CursorPoint::new(100.0, 100.0),
        button: Some(MouseButton::Left),
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "move", "move", "down", "up"]).as_str())
    );

    utils.move_cursor((0., 0.)).await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "move", "move", "down", "up", "leave"]).as_str())
    );

    utils.push_event(TestEvent::Mouse {
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
                vec!["enter", "move", "move", "down", "up", "leave", "globalup"]
            )
            .as_str()
        )
    );
}

#[tokio::test]
pub async fn pointer_events_from_touch() {
    fn pointer_events_app() -> Element {
        let mut state = use_signal(std::vec::Vec::new);

        let onpointerdown = move |_| state.push("down".to_string());

        let onpointerup = move |_| state.push("up".to_string());

        let onpointermove = move |_| state.push("move".to_string());

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
                    onpointermove: onpointermove,
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

    utils.push_event(TestEvent::Touch {
        name: EventName::TouchMove,
        location: CursorPoint::new(100.0, 100.0),
        finger_id: 1,
        phase: TouchPhase::Moved,
        force: None,
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "move"]).as_str())
    );

    utils.push_event(TestEvent::Touch {
        name: EventName::TouchStart,
        location: CursorPoint::new(100.0, 100.0),
        finger_id: 1,
        phase: TouchPhase::Started,
        force: None,
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "move", "down"]).as_str())
    );

    utils.push_event(TestEvent::Touch {
        name: EventName::TouchEnd,
        location: CursorPoint::new(100.0, 100.0),
        finger_id: 1,
        phase: TouchPhase::Ended,
        force: None,
    });
    utils.wait_for_update().await;
    assert_eq!(
        label.get(0).text(),
        Some(format!("{:?}", vec!["enter", "move", "down", "up"]).as_str())
    );
}

#[tokio::test]
pub async fn filedrop_events() {
    #[derive(PartialEq)]
    enum Status {
        Idle,
        Hovering,
    }

    fn filedrop_events_app() -> Element {
        let mut path = use_signal::<Option<String>>(|| None);
        let mut status = use_signal(|| Status::Idle);

        let msg = path.read().clone().unwrap_or("Default".to_string());

        let background = if *status.read() == Status::Hovering {
            "red"
        } else {
            "blue"
        };

        rsx!(
            rect {
                height: "100%",
                width: "100%",
                background: "{background}",
                onglobalfilehover: move |_| status.set(Status::Hovering),
                onglobalfilehovercancelled: move |_| status.set(Status::Idle),
                onfiledrop: move |e| {
                    status.set(Status::Idle);
                    path.set(Some(e.file_path.as_ref().unwrap().to_string_lossy().to_string()))
                },
                label {
                    "{msg}"
                }
            }
        )
    }

    let mut utils = launch_test(filedrop_events_app);

    let root = utils.root();

    assert_eq!(root.get(0).get(0).get(0).text(), Some("Default"));
    assert_eq!(root.get(0).style().background, Fill::Color(Color::BLUE));

    utils.push_event(TestEvent::File {
        name: EventName::GlobalFileHover,
        cursor: (5., 5.).into(),
        file_path: None,
    });

    utils.wait_for_update().await;

    assert_eq!(root.get(0).style().background, Fill::Color(Color::RED));

    utils.push_event(TestEvent::File {
        name: EventName::FileDrop,
        cursor: (5., 5.).into(),
        file_path: Some(PathBuf::from_str("/nice/path/right.rs").unwrap()),
    });

    utils.wait_for_update().await;

    assert_eq!(
        root.get(0).get(0).get(0).text(),
        Some("/nice/path/right.rs")
    );
    assert_eq!(root.get(0).style().background, Fill::Color(Color::BLUE));
}
