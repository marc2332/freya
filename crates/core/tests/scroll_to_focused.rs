use freya::prelude::*;
use freya_testing::prelude::*;

#[tokio::test]
pub async fn scroll_to_focused() {
    fn scroll_to_focused_app() -> Element {
        rsx!(
            ScrollView {
                padding: "50",
                ScrollView {
                    direction: "horizontal",
                    height: "500",
                    spacing: "500",
                    Button {
                        label { "1" }
                    }
                    label {
                        a11y_focusable: "true",
                        max_lines: "1",
                        "This can also be focused!"
                    }
                    Button {
                        label { "2" }
                    }
                }
                ScrollView {
                    direction: "horizontal",
                    height: "500",
                    spacing: "500",
                    Button {
                        label { "3" }
                    }
                    label {
                        a11y_focusable: "true",
                        max_lines: "1",
                        "And this too!"
                    }
                    Button {
                        label { "4" }
                    }
                }
            }
        )
    }

    let mut utils = launch_test(scroll_to_focused_app);
    utils.wait_for_update().await;
    assert_eq!(utils.focus_id(), AccessibilityId(0));
    assert!(utils.focus_node().is_visible());

    // The "repeat(2)" makes sure that only these 6 elements defined above are being focused
    for id in [3, 12, 4, 6, 13, 7].repeat(2) {
        utils.push_event(TestEvent::Keyboard {
            name: KeyboardEventName::KeyDown,
            key: Key::Tab,
            code: Code::Tab,
            modifiers: Modifiers::default(),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        assert_eq!(utils.focus_id(), AccessibilityId(id));
        assert!(utils.focus_node().is_visible());
    }
}
