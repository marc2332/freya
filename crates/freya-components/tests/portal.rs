use std::time::Duration;

use freya::prelude::*;
use freya_animation::prelude::Function;
use freya_testing::prelude::*;

fn test_portal(id: &'static str) -> Portal<&'static str> {
    Portal::new(id)
        .key(id)
        .width(Size::px(100.))
        .height(Size::px(100.))
        .duration(Duration::from_secs(10))
        .function(Function::Linear)
        .child(label().expanded().text(id))
}

fn label_x(test: &TestingRunner, text: &str) -> f32 {
    test.find(|node, element| {
        Label::try_downcast(element)
            .filter(|label| label.text == text)
            .map(|_| node)
    })
    .unwrap()
    .layout()
    .area
    .min_x()
}

#[test]
pub fn portal_animates_position_change() {
    fn portal_app() -> impl IntoElement {
        let mut swap = use_state(|| false);

        rect()
            .child(
                Button::new()
                    .on_press(move |_| swap.set(!swap()))
                    .child("Swap"),
            )
            .child(rect().horizontal().children(if !swap() {
                vec![test_portal("A"), test_portal("B")]
            } else {
                vec![test_portal("B"), test_portal("A")]
            }))
    }

    let mut test = launch_test(portal_app);
    test.animation_clock().disable();
    test.poll_n(Duration::from_millis(1), 15);

    let initial_a_x = label_x(&test, "A");
    let initial_b_x = label_x(&test, "B");
    assert!(initial_a_x < initial_b_x);

    test.click_cursor((15.0, 15.0));
    test.poll_n(Duration::from_millis(1), 15);

    assert_eq!(label_x(&test, "A"), initial_b_x);
    assert_eq!(label_x(&test, "B"), initial_a_x);
}

fn spaced_portal_app(dependency: fn(bool) -> u8) -> impl IntoElement {
    let mut spaced = use_state(|| false);

    rect()
        .child(
            Button::new()
                .on_press(move |_| spaced.set(!spaced()))
                .child("Toggle"),
        )
        .child(
            rect()
                .horizontal()
                .child(
                    rect()
                        .width(Size::px(if spaced() { 100. } else { 0. }))
                        .height(Size::px(100.)),
                )
                .child(test_portal("A").animation_dependency(dependency(spaced()))),
        )
}

#[test]
pub fn portal_with_dependency_snaps_on_unrelated_layout_change() {
    let mut test = launch_test(|| spaced_portal_app(|_| 0));
    test.poll_n(Duration::from_millis(1), 15);
    let initial_x = label_x(&test, "A");

    test.click_cursor((15.0, 15.0));
    test.poll_n(Duration::from_millis(1), 15);

    assert_eq!(label_x(&test, "A"), initial_x + 100.);
}

#[test]
pub fn portal_with_dependency_animates_on_dependency_change() {
    let mut test = launch_test(|| spaced_portal_app(|spaced| spaced as u8));
    test.poll_n(Duration::from_millis(1), 15);
    let initial_x = label_x(&test, "A");

    test.click_cursor((15.0, 15.0));
    test.poll_n(Duration::from_millis(1), 15);

    assert!(label_x(&test, "A") < initial_x + 50.);

    test.animation_clock().disable();
    test.poll_n(Duration::from_millis(1), 15);

    assert_eq!(label_x(&test, "A"), initial_x + 100.);
}
