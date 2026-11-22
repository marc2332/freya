use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn input_context_menu_edits_text() {
    fn app() -> impl IntoElement {
        Input::new(use_consume::<State<String>>())
    }

    let (mut test, value) = TestingRunner::new(
        app,
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create("Hello world".to_string())),
        1.,
    );

    let cursor = (15., 15.);
    for item in ["Select all", "Copy", "Delete", "Paste"] {
        test.move_cursor(cursor);
        for name in [MouseEventName::MouseDown, MouseEventName::MouseUp] {
            test.send_event(PlatformEvent::Mouse {
                name,
                cursor: cursor.into(),
                button: Some(MouseButton::Right),
            });
            test.sync_and_update();
        }
        let location = test
            .find(|node, element| {
                Label::try_downcast(element)
                    .filter(|label| label.text.as_ref() == item)
                    .map(|_| node.layout().area.center().to_f64())
            })
            .unwrap();
        test.click_cursor(location);
        test.handle_events_immediately();
        test.sync_and_update();

        if item == "Delete" {
            assert!(value.peek().is_empty());
        }
    }

    assert_eq!(value.peek().as_str(), "Hello world");
}
