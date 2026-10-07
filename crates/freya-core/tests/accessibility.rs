use accesskit::TreeId;
use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn moving_an_a11y_id_between_siblings() {
    let (mut test, mut holder) = TestingRunner::new(
        || {
            let holder = use_consume::<State<usize>>();
            let a11y_id = use_a11y();

            rect().children(
                (0..3)
                    .map(|index| {
                        rect()
                            .width(Size::px(100.))
                            .height(Size::px(100.))
                            .maybe(index == *holder.read(), |el| el.a11y_id(a11y_id))
                            .into_element()
                    })
                    .collect::<Vec<_>>(),
            )
        },
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(0usize)),
        1.,
    );
    test.sync_and_update();

    for index in 1..3 {
        holder.set(index);
        let update = test.sync_and_update();

        let mut children = update
            .nodes
            .iter()
            .flat_map(|(_, node)| node.children())
            .collect::<Vec<_>>();
        let total = children.len();
        children.sort_unstable();
        children.dedup();
        assert_eq!(children.len(), total, "duplicated accessibility children");
    }
}

fn action_request(
    action: AccessibilityAction,
    target_node: AccessibilityId,
) -> AccessibilityActionEventData {
    AccessibilityActionEventData {
        action,
        target_tree: TreeId::ROOT,
        target_node,
        data: None,
    }
}

#[test]
fn accessibility_button_click() {
    let (mut test, count) = TestingRunner::new(
        || {
            let mut count = use_consume::<State<usize>>();
            Button::new()
                .on_press(move |_| *count.write() += 1)
                .child("Activate")
        },
        (300., 300.).into(),
        |runner| runner.provide_root_context(|| State::create(0usize)),
        1.,
    );
    let update = test.sync_and_update();
    let (button_id, button) = update
        .nodes
        .iter()
        .find(|(_, node)| node.role() == AccessibilityRole::Button)
        .unwrap();
    assert!(button.supports_action(AccessibilityAction::Click));
    assert!(test.send_accessibility_action(action_request(AccessibilityAction::Click, *button_id)));
    test.sync_and_update();
    assert_eq!(*count.peek(), 1);
}

#[test]
fn accessibility_disabled_nodes_reject_actions() {
    let mut test = launch_test(|| {
        rect()
            .a11y_id(AccessibilityId(100))
            .a11y_enabled(false)
            .on_accessibility_action(|_| panic!("Disabled nodes must not receive actions"))
    });
    test.sync_and_update();
    for action in [AccessibilityAction::Click, AccessibilityAction::Increment] {
        assert!(!test.send_accessibility_action(action_request(action, AccessibilityId(100))));
    }
    test.sync_and_update();
}
