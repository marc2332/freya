use accesskit::TreeId;
use freya::prelude::*;
use freya_core::integration::{
    ACCESSIBILITY_ROOT_ID,
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
fn disabled_nodes_reject_focus_and_release_existing_focus() {
    let input_id = AccessibilityId(100);
    let (mut test, mut enabled) = TestingRunner::new(
        move || {
            let enabled = use_consume::<State<bool>>();
            rect()
                .a11y_id(input_id)
                .a11y_focusable(*enabled.read())
        },
        (300., 300.).into(),
        |runner| runner.provide_root_context(|| State::create(false)),
        1.,
    );
    test.sync_and_update();

    test.run_in(|| input_id.request_focus());
    assert_eq!(test.sync_and_update().focus, ACCESSIBILITY_ROOT_ID);

    enabled.set(true);
    test.sync_and_update();
    test.run_in(|| input_id.request_focus());
    assert_eq!(test.sync_and_update().focus, input_id);

    enabled.set(false);
    assert_eq!(test.sync_and_update().focus, ACCESSIBILITY_ROOT_ID);
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

#[test]
fn keyed_sibling_reorders_update_accessibility_children() {
    let (mut test, mut keys) = TestingRunner::new(
        || {
            let keys = use_consume::<State<[u64; 3]>>();
            rect()
                .a11y_id(AccessibilityId(1000))
                .a11y_role(AccessibilityRole::Group)
                .children(keys().into_iter().map(|key| {
                    rect()
                        .key(key)
                        .width(Size::px(100.))
                        .height(Size::px(20.))
                        .a11y_id(AccessibilityId(100 + key))
                        .a11y_role(AccessibilityRole::Button)
                }))
        },
        (300., 300.).into(),
        |runner| runner.provide_root_context(|| State::create([0u64, 1, 2])),
        1.,
    );
    test.sync_and_update();

    for next in [[2, 1, 0], [1, 0, 2], [0, 1, 2]] {
        keys.set(next);
        let update = test.sync_and_update();
        let (_, parent) = update
            .nodes
            .iter()
            .find(|(id, _)| *id == AccessibilityId(1000))
            .expect("reordered parent must receive an accessibility update");
        assert_eq!(
            parent.children(),
            next.map(|key| AccessibilityId(100 + key))
        );
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
