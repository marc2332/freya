use std::time::Duration;

use freya::prelude::*;
use freya_animation::prelude::Function;
use freya_testing::prelude::*;

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
                vec![
                    Portal::new("a")
                        .key("a")
                        .width(Size::px(100.))
                        .height(Size::px(100.))
                        .duration(Duration::from_millis(50))
                        .function(Function::Linear)
                        .child(label().expanded().text("A"))
                        .into(),
                    Portal::new("b")
                        .key("b")
                        .width(Size::px(100.))
                        .height(Size::px(100.))
                        .duration(Duration::from_millis(50))
                        .function(Function::Linear)
                        .child(label().expanded().text("B"))
                        .into(),
                ]
            } else {
                vec![
                    Portal::new("b")
                        .key("b")
                        .width(Size::px(100.))
                        .height(Size::px(100.))
                        .duration(Duration::from_millis(50))
                        .function(Function::Linear)
                        .child(label().expanded().text("B"))
                        .into(),
                    Portal::new("a")
                        .key("a")
                        .width(Size::px(100.))
                        .height(Size::px(100.))
                        .duration(Duration::from_millis(50))
                        .function(Function::Linear)
                        .child(label().expanded().text("A"))
                        .into(),
                ]
            }))
    }

    let mut test = launch_test(portal_app);
    test.poll(Duration::from_millis(1), Duration::from_millis(10));

    // Find labels and get initial positions
    let labels = test.find_many(|node, element| Label::try_downcast(element).map(|_| node));

    let label_a = labels
        .iter()
        .find(|l| Label::try_downcast(&*l.element()).unwrap().text == "A")
        .unwrap();
    let label_b = labels
        .iter()
        .find(|l| Label::try_downcast(&*l.element()).unwrap().text == "B")
        .unwrap();

    let initial_a_x = label_a.layout().area.min_x();
    let initial_b_x = label_b.layout().area.min_x();

    // A should be to the left of B initially
    assert!(initial_a_x < initial_b_x);

    // Click the swap button
    test.click_cursor((15.0, 15.0));

    // Poll partway through the animation (25ms out of 50ms)
    test.poll(Duration::from_millis(2), Duration::from_millis(40));

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(|_| node));

    let label_a = labels
        .iter()
        .find(|l| Label::try_downcast(&*l.element()).unwrap().text == "A")
        .unwrap();
    let label_b = labels
        .iter()
        .find(|l| Label::try_downcast(&*l.element()).unwrap().text == "B")
        .unwrap();

    let mid_a_x = label_a.layout().area.min_x();
    let mid_b_x = label_b.layout().area.min_x();

    // Mid-animation: A should have moved right from initial position
    assert!(mid_a_x > initial_a_x, "A should be moving right");
    // Mid-animation: B should have moved left from initial position
    assert!(mid_b_x < initial_b_x, "B should be moving left");

    // Wait for animation to complete
    test.poll(Duration::from_millis(1), Duration::from_millis(75));

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(|_| node));

    let label_a = labels
        .iter()
        .find(|l| Label::try_downcast(&*l.element()).unwrap().text == "A")
        .unwrap();
    let label_b = labels
        .iter()
        .find(|l| Label::try_downcast(&*l.element()).unwrap().text == "B")
        .unwrap();

    let final_a_x = label_a.layout().area.min_x();
    let final_b_x = label_b.layout().area.min_x();

    // After swap, B should be to the left of A
    assert!(final_b_x < final_a_x);

    // Final positions should match the swapped initial positions
    assert_eq!(final_a_x, initial_b_x, "A should now be where B started");
    assert_eq!(final_b_x, initial_a_x, "B should now be where A started");
}
