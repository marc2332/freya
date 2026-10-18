use accesskit::TreeId;
use freya::prelude::*;
use freya_core::integration::{
    AccessibilityTree,
    NodesState,
    Runner,
    Tree,
};
use freya_engine::prelude::{
    FontCollection,
    FontMgr,
};
use freya_testing::prelude::*;

#[test]
fn initialization_preserves_pending_focus() {
    let input_id = AccessibilityId(100);
    let mut runner = Runner::new(move || {
        rect()
            .expanded()
            .child(
                rect()
                    .a11y_id(input_id)
                    .a11y_focusable(true)
                    .a11y_role(AccessibilityRole::TextInput),
            )
            .into_element()
    });
    let mut tree = Tree::default();
    tree.apply_mutations(runner.sync_and_update(), 1.0);
    let (events_sender, _events_receiver) = futures_channel::mpsc::unbounded();
    tree.measure_layout(
        (300.0, 300.0).into(),
        &mut FontCollection::new(),
        &FontMgr::default(),
        &events_sender,
        &mut NodesState::default(),
        1.0,
        &[],
    );
    tree.accessibility_diff
        .request_focus(AccessibilityFocusStrategy::Node(input_id));
    let mut accessibility = AccessibilityTree::default();

    let update = accessibility.init(&mut tree, "");

    assert_eq!(update.focus, input_id);
    assert_eq!(accessibility.focused_id, input_id);
    assert!(tree.accessibility_diff.requested_focus.is_none());
}

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
    let button = test
        .find(|_, element| {
            let accessibility = element.accessibility();
            (accessibility.builder.role() == AccessibilityRole::Button)
                .then(|| accessibility.into_owned())
        })
        .unwrap();
    assert!(button.builder.supports_action(AccessibilityAction::Click));
    assert!(test.send_accessibility_action(action_request(
        AccessibilityAction::Click,
        button.a11y_id.unwrap(),
    )));
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
