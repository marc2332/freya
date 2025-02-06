use freya::prelude::*;
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
