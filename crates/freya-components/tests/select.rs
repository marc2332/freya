use std::time::Duration;

use freya::prelude::*;
use freya_testing::prelude::*;

fn two_selects_app() -> impl IntoElement {
    rect()
        .horizontal()
        .spacing(6.)
        .child(
            Select::new()
                .selected_item("Select A")
                .child(MenuItem::new().child("A Item 1"))
                .child(MenuItem::new().child("A Item 2")),
        )
        .child(
            Select::new()
                .selected_item("Select B")
                .child(MenuItem::new().child("B Item 1"))
                .child(MenuItem::new().child("B Item 2")),
        )
}

fn label_center(test: &TestingRunner, text: &str) -> (f64, f64) {
    let node = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|label| label.text == text)
                .map(|_| node)
        })
        .unwrap_or_else(|| panic!("label `{text}` not found"));
    let area = node.layout().area;
    (
        area.min_x() as f64 + area.size.width as f64 / 2.0,
        area.min_y() as f64 + area.size.height as f64 / 2.0,
    )
}

fn label_exists(test: &TestingRunner, text: &str) -> bool {
    test.find(|node, element| {
        Label::try_downcast(element)
            .filter(|label| label.text == text)
            .map(|_| node)
    })
    .is_some()
}

fn click_without_settling(test: &mut TestingRunner, point: (f64, f64)) {
    test.send_event(PlatformEvent::Mouse {
        name: MouseEventName::MouseDown,
        cursor: point.into(),
        button: Some(MouseButton::Left),
    });
    test.process_events_and_layout();
    test.send_event(PlatformEvent::Mouse {
        name: MouseEventName::MouseUp,
        cursor: point.into(),
        button: Some(MouseButton::Left),
    });
    test.process_events_and_layout();
}

#[test]
pub fn clicking_another_select_keeps_it_open() {
    let mut test = launch_test(two_selects_app);
    test.sync_and_update();

    test.click_cursor(label_center(&test, "Select A"));
    test.poll_n(Duration::from_millis(5), 100);
    test.sync_and_update();

    assert!(label_exists(&test, "A Item 1"), "Select A should be open");

    test.click_cursor(label_center(&test, "Select B"));
    test.poll_n(Duration::from_millis(5), 100);
    test.sync_and_update();

    assert!(
        !label_exists(&test, "A Item 1"),
        "Select A should close once Select B is clicked"
    );
    assert!(
        label_exists(&test, "B Item 1"),
        "Select B should stay open after being clicked"
    );
}

#[test]
pub fn opening_second_select_does_not_close_itself_while_its_focus_is_still_landing() {
    let mut test = launch_test(two_selects_app);
    test.sync_and_update();

    let a_point = label_center(&test, "Select A");
    click_without_settling(&mut test, a_point);
    test.apply_pending_focus_strategy();
    test.commit_accessibility();
    test.process_events_and_layout();
    test.process_events_and_layout();
    assert!(label_exists(&test, "A Item 1"), "Select A should open");

    let b_point = label_center(&test, "Select B");
    click_without_settling(&mut test, b_point);

    assert!(label_exists(&test, "A Item 1"), "Select A stays open until it loses focus");
    assert!(!label_exists(&test, "B Item 1"), "Select B waits for its own focus to land");

    test.apply_pending_focus_strategy();
    test.commit_accessibility();
    test.process_events_and_layout();
    test.poll_n(Duration::from_millis(5), 100);
    test.sync_and_update();

    assert!(
        !label_exists(&test, "A Item 1"),
        "Select A should close once focus moves to Select B"
    );
    assert!(
        label_exists(&test, "B Item 1"),
        "Select B should open once its own focus request lands"
    );
}
