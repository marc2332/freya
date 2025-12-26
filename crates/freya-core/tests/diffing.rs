use std::{
    collections::HashMap,
    sync::atomic::{
        AtomicI8,
        Ordering,
    },
};

use freya::helpers::*;
use freya_core::{
    integration::*,
    path_element::PathElement,
    prelude::*,
    runner::{
        Diff,
        MutationRemove,
    },
};
use rustc_hash::FxHashMap;

struct RawIdMap(FxHashMap<u64, Vec<u64>>);

impl From<FxHashMap<u64, Vec<u64>>> for RawIdMap {
    fn from(m: FxHashMap<u64, Vec<u64>>) -> Self {
        RawIdMap(m)
    }
}

impl From<RawIdMap> for FxHashMap<NodeId, Vec<NodeId>> {
    fn from(raw: RawIdMap) -> Self {
        raw.0
            .into_iter()
            .map(|(k, v)| (NodeId::from(k), v.into_iter().map(NodeId::from).collect()))
            .collect()
    }
}

fn convert_ids(map: FxHashMap<u64, Vec<u64>>) -> FxHashMap<NodeId, Vec<NodeId>> {
    RawIdMap::from(map).into()
}

#[test]
fn mutations() {
    fn app() -> Element {
        rect()
            .children([
                from_fn_captured(|| counter(5, true)),
                from_fn_captured(|| counter(10, false)),
            ])
            .into()
    }

    fn counter(stuff: u8, inc_or_dec: bool) -> Element {
        let mut value = use_state(|| stuff);

        rect()
            .layer(*value.read() as i16)
            .on_mouse_up(move |_| {
                if inc_or_dec {
                    *value.write() += 1;
                } else {
                    *value.write() -= 1;
                }
            })
            .children(
                [label().text(format!("Value is {}", value.read())).into()]
                    .into_iter()
                    .chain({
                        if *value.read() == stuff + { if inc_or_dec { 1 } else { 0 } } {
                            vec![label().text("Hello World!").into()]
                        } else {
                            vec![]
                        }
                    })
                    .collect::<Vec<Element>>(),
            )
            .into()
    }

    let mut runner = Runner::new(app);
    let mut tree = Tree::default();

    // Addition
    let mutations = runner.sync_and_update();
    assert!(!mutations.added.is_empty());
    assert!(mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());
    tree.apply_mutations(mutations);
    tree.verify_tree_integrity();
    assert_eq!(
        tree.children,
        convert_ids(FxHashMap::from_iter([
            (1, vec![2]),
            (2, vec![5, 7]),
            (5, vec![6]),
            (7, vec![8, 9]),
        ]))
    );

    // Nothing
    let mutations = runner.sync_and_update();
    assert!(mutations.added.is_empty());
    assert!(mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());
    tree.apply_mutations(mutations);
    assert_eq!(
        tree.children,
        convert_ids(FxHashMap::from_iter([
            (1, vec![2]),
            (2, vec![5, 7]),
            (5, vec![6]),
            (7, vec![8, 9]),
        ]))
    );

    // Addition
    runner.handle_event(
        5,
        EventName::MouseUp,
        EventType::Mouse(MouseEventData::default()),
        false,
    );
    let mutations = runner.sync_and_update();
    assert!(!mutations.added.is_empty());
    assert!(!mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());
    tree.apply_mutations(mutations);
    assert_eq!(
        tree.children,
        convert_ids(FxHashMap::from_iter([
            (1, vec![2]),
            (2, vec![5, 7]),
            (5, vec![6, 10]),
            (7, vec![8, 9]),
        ]))
    );

    // Removal
    runner.handle_event(
        7,
        EventName::MouseUp,
        EventType::Mouse(MouseEventData::default()),
        false,
    );
    let mutations = runner.sync_and_update();
    assert!(mutations.added.is_empty());
    assert!(!mutations.modified.is_empty());
    assert!(!mutations.removed.is_empty());
    tree.apply_mutations(mutations);
    assert_eq!(
        tree.children,
        convert_ids(FxHashMap::from_iter([
            (1, vec![2]),
            (2, vec![5, 7]),
            (5, vec![6, 10]),
            (7, vec![8]),
        ]))
    );
}

#[test]
fn components() {
    fn app() -> Element {
        let mut value = use_state(|| 1);
        let curr = *value.read();
        rect()
            .on_mouse_up(move |_| {
                *value.write() += 1;
            })
            .children([
                from_fn_captured(move || counter(&curr)),
                from_fn_standalone_borrowed(*value.read(), counter),
            ])
            .into()
    }

    fn counter(value: &u8) -> Element {
        rect()
            .children([label().text(format!("Value is {value}")).into()])
            .into()
    }

    let mut runner = Runner::new(app);
    let mut tree = Tree::default();

    let mutations = runner.sync_and_update();
    assert_eq!(mutations.added.len(), 5);
    assert!(mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());
    tree.apply_mutations(mutations);

    runner.handle_event(
        2,
        EventName::MouseUp,
        EventType::Mouse(MouseEventData::default()),
        false,
    );
    let mutations = runner.sync_and_update();
    assert!(mutations.added.is_empty());
    assert_eq!(mutations.modified.len(), 2);
    assert!(mutations.removed.is_empty());
    tree.apply_mutations(mutations);
}

#[test]
fn state_reconcillation() {
    fn first() -> Element {
        rect().into()
    }

    fn second() -> Element {
        rect().into()
    }

    let first_render: Element = rect().children([from_fn_standalone(second)]).into();
    let second_render: Element = rect()
        .children([from_fn_standalone(first), from_fn_standalone(second)])
        .into();
    let first_render = PathElement::from_element(vec![0], first_render);
    let second_render = PathElement::from_element(vec![0], second_render);
    let mut diff = Diff::default();
    second_render.diff(Some(&first_render), &mut diff);
    assert_eq!(diff.added, vec![vec![0, 0].into_boxed_slice()]);
    assert!(diff.modified.is_empty());
    assert!(diff.removed.is_empty());

    let first_render: Element = rect()
        .children([from_fn_standalone(first), from_fn_standalone(second)])
        .into();
    let second_render: Element = rect().children([from_fn_standalone(second)]).into();
    let first_render = PathElement::from_element(vec![0], first_render);
    let second_render = PathElement::from_element(vec![0], second_render);
    let mut diff = Diff::default();
    second_render.diff(Some(&first_render), &mut diff);
    assert!(diff.added.is_empty());
    assert!(diff.modified.is_empty());
    assert_eq!(diff.removed, vec![vec![0, 0].into_boxed_slice()]);

    let first_render: Element = rect()
        .children([from_fn_standalone(first), from_fn_standalone(second)])
        .into();
    let first_render = PathElement::from_element(vec![0], first_render);
    let mut diff = Diff::default();
    first_render.diff(None, &mut diff);
    assert!(!diff.added.is_empty());
    assert!(diff.modified.is_empty());
    assert!(diff.removed.is_empty());
}

#[test]
fn state_reconcillation2() {
    fn app() -> Element {
        let mut value = use_state(|| 5);
        rect()
            .children(
                vec![
                    rect()
                        .on_mouse_up(move |_| {
                            *value.write() += 1;
                        })
                        .into(),
                    rect()
                        .on_mouse_up(move |_| {
                            *value.write() -= 1;
                        })
                        .into(),
                ]
                .with(
                    (0..*value.read())
                        .map(|_| from_fn_captured(|| counter(5)))
                        .collect::<Vec<_>>(),
                ),
            )
            .into()
    }

    fn counter(stuff: u8) -> Element {
        rect()
            .children([label().text(format!("Value is {stuff}")).into()])
            .into()
    }

    let mut runner = Runner::new(app);
    let mut tree = Tree::default();

    let mutations = runner.sync_and_update();
    assert!(!mutations.added.is_empty());
    assert!(mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());
    tree.apply_mutations(mutations);
    assert_eq!(
        tree.children,
        convert_ids(FxHashMap::from_iter([
            (1, vec![2]),
            (2, vec![3, 4, 10, 12, 14, 16, 18]),
            (10, vec![11]),
            (12, vec![13]),
            (14, vec![15]),
            (16, vec![17]),
            (18, vec![19]),
        ]))
    );
    assert_eq!(
        runner
            .scopes
            .get(&ScopeId::ROOT)
            .unwrap()
            .borrow()
            .nodes
            .size(),
        9
    );

    runner.handle_event(
        3,
        EventName::MouseUp,
        EventType::Mouse(MouseEventData::default()),
        false,
    );
    let mutations = runner.sync_and_update();
    assert!(!mutations.added.is_empty());
    assert!(!mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());
    tree.apply_mutations(mutations);
    assert_eq!(
        tree.children,
        convert_ids(FxHashMap::from_iter([
            (1, vec![2]),
            (2, vec![3, 4, 10, 12, 14, 16, 18, 21]),
            (10, vec![11]),
            (12, vec![13]),
            (14, vec![15]),
            (16, vec![17]),
            (18, vec![19]),
            (21, vec![22]),
        ]))
    );
    assert_eq!(
        runner
            .scopes
            .get(&ScopeId::ROOT)
            .unwrap()
            .borrow()
            .nodes
            .size(),
        10
    );

    runner.handle_event(
        4,
        EventName::MouseUp,
        EventType::Mouse(MouseEventData::default()),
        false,
    );
    let mutations = runner.sync_and_update();
    assert!(mutations.added.is_empty());
    assert!(!mutations.modified.is_empty());
    assert!(!mutations.removed.is_empty());
    tree.apply_mutations(mutations);
    runner.handle_event(
        4,
        EventName::MouseUp,
        EventType::Mouse(MouseEventData::default()),
        false,
    );
    let mutations = runner.sync_and_update();
    assert!(mutations.added.is_empty());
    assert!(!mutations.modified.is_empty());
    assert!(!mutations.removed.is_empty());
    tree.apply_mutations(mutations);
    assert_eq!(
        tree.children,
        convert_ids(FxHashMap::from_iter([
            (1, vec![2]),
            (2, vec![3, 4, 10, 12, 14, 16]),
            (10, vec![11]),
            (12, vec![13]),
            (14, vec![15]),
            (16, vec![17]),
        ]))
    );
    assert_eq!(
        runner
            .scopes
            .get(&ScopeId::ROOT)
            .unwrap()
            .borrow()
            .nodes
            .size(),
        8
    );
}

#[test]
fn scopes_smart_rerun() {
    static COUNTER: AtomicI8 = AtomicI8::new(0);

    fn app() -> Element {
        let mut value = use_state(|| 5);
        rect()
            .children(vec![
                rect()
                    .on_mouse_up(move |_| {
                        *value.write() += 1;
                    })
                    .into(),
                label().text(format!("Value is {}", value.read())).into(),
                from_fn_standalone_borrowed(*value.read(), counter),
            ])
            .into()
    }

    fn counter(stuff: &usize) -> Element {
        COUNTER.fetch_add(1, Ordering::Relaxed);
        let mut value = use_state(|| 5);
        rect()
            .on_mouse_up(move |_| {
                *value.write() += 1;
            })
            .children([label()
                .text(format!("Value is {stuff} {}", value.read()))
                .into()])
            .into()
    }

    let mut runner = Runner::new(app);
    let mut tree = Tree::default();

    let mutations = runner.sync_and_update();
    tree.apply_mutations(mutations);
    assert_eq!(COUNTER.load(Ordering::Relaxed), 1);

    runner.handle_event(
        3,
        EventName::MouseUp,
        EventType::Mouse(MouseEventData::default()),
        false,
    );
    runner.handle_event(
        6,
        EventName::MouseUp,
        EventType::Mouse(MouseEventData::default()),
        false,
    );
    let mutations = runner.sync_and_update();
    assert!(mutations.added.is_empty());
    assert!(!mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());
    tree.apply_mutations(mutations);
    assert_eq!(COUNTER.load(Ordering::Relaxed), 2);
}

#[test]
fn element_diffing() {
    let first_render: Element = rect()
        .children([rect().into(), rect().into(), rect().into()])
        .into();
    let second_render: Element = rect().children([rect().into(), rect().into()]).into();
    let first_render = PathElement::from_element(vec![0], first_render);
    let second_render = PathElement::from_element(vec![0], second_render);
    let mut diff = Diff::default();
    second_render.diff(Some(&first_render), &mut diff);
    assert!(diff.added.is_empty());
    assert!(diff.modified.is_empty());
    assert_eq!(diff.removed, vec![vec![0, 2].into_boxed_slice()]);

    // Compare keys from one render to the other one and diff those, then dif normally the others, and finally remove thus not unmarked

    let first_render: Element = rect()
        .children([
            rect().key(1).into(),
            rect().key(2).into(),
            rect().key(3).into(),
        ])
        .into();
    let second_render: Element = rect()
        .children([rect().key(1).into(), rect().key(3).into()])
        .into();
    let first_render = PathElement::from_element(vec![0], first_render);
    let second_render = PathElement::from_element(vec![0], second_render);
    let mut diff = Diff::default();
    second_render.diff(Some(&first_render), &mut diff);
    assert!(diff.added.is_empty());
    assert!(diff.modified.is_empty());
    assert_eq!(diff.removed, vec![vec![0, 1].into_boxed_slice()]);
}

#[test]
fn element_diffing3() {
    let first_render: Element = rect().key(1).into();
    let second_render: Element = rect().key(2).into();
    let first_render = PathElement::from_element(vec![], first_render);
    let second_render = PathElement::from_element(vec![], second_render);
    let mut diff = Diff::default();
    second_render.diff(Some(&first_render), &mut diff);
    assert_eq!(diff.added, vec![vec![].into_boxed_slice()]);
    assert!(diff.modified.is_empty());
    assert_eq!(diff.removed, vec![vec![].into_boxed_slice()]);

    fn first() -> Element {
        rect().into()
    }

    fn second() -> Element {
        rect().into()
    }

    let first_render: Element = from_fn_standalone(first);
    let second_render: Element = from_fn_standalone(second);
    let first_render = PathElement::from_element(vec![], first_render);
    let second_render = PathElement::from_element(vec![], second_render);
    let mut diff = Diff::default();
    second_render.diff(Some(&first_render), &mut diff);
    assert_eq!(diff.added, vec![vec![].into_boxed_slice()]);
    assert!(diff.modified.is_empty());
    assert_eq!(diff.removed, vec![vec![].into_boxed_slice()]);
}

#[test]
fn element_diffing4() {
    fn container(children: &Element) -> Element {
        children.clone()
    }

    let mut runner = Runner::new(|| {
        let state = use_consume::<State<bool>>();
        if state() {
            rect()
                .child(from_fn_standalone_borrowed_keyed(
                    1,
                    from_fn_standalone_borrowed("1".into(), container),
                    container,
                ))
                .child(from_fn_standalone_borrowed_keyed(
                    2,
                    from_fn_standalone_borrowed("2".into(), container),
                    container,
                ))
                .into()
        } else {
            rect()
                .child(from_fn_standalone_borrowed_keyed(
                    2,
                    from_fn_standalone_borrowed("2".into(), container),
                    container,
                ))
                .child(from_fn_standalone_borrowed_keyed(
                    1,
                    from_fn_standalone_borrowed("1".into(), container),
                    container,
                ))
                .into()
        }
    });
    let mut state = runner.provide_root_context(|| State::create(true));
    let mutations = runner.sync_and_update();
    assert_eq!(mutations.added.len(), 3);
    assert_eq!(mutations.added[0].2, 0);
    assert_eq!(mutations.added[1].2, 0);
    assert_eq!(mutations.added[2].2, 1);
    assert!(mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());

    let scope_a_before = runner
        .scopes
        .get(&ScopeId::from(2))
        .unwrap()
        .borrow()
        .path_in_parent
        .clone();
    let scope_b_before = runner
        .scopes
        .get(&ScopeId::from(3))
        .unwrap()
        .borrow()
        .path_in_parent
        .clone();

    assert_eq!(scope_a_before.as_ref(), [0, 0]);
    assert_eq!(scope_b_before.as_ref(), [0, 1]);

    state.set(false);
    let mutations = runner.sync_and_update();
    assert!(mutations.added.is_empty());
    assert!(mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());
    assert!(!mutations.moved.is_empty());

    let scope_a_after = runner
        .scopes
        .get(&ScopeId::from(2))
        .unwrap()
        .borrow()
        .path_in_parent
        .clone();
    let scope_b_after = runner
        .scopes
        .get(&ScopeId::from(3))
        .unwrap()
        .borrow()
        .path_in_parent
        .clone();

    assert_eq!(scope_a_after.as_ref(), [0, 1]);
    assert_eq!(scope_b_after.as_ref(), [0, 0]);
}

#[test]
fn element_diffing5() {
    fn container(_: &()) -> Element {
        rect().into()
    }

    let first_render: Element = rect()
        .child(from_fn_standalone_borrowed_keyed(1, (), container))
        .child(from_fn_standalone_borrowed_keyed(2, (), container))
        .into();
    let second_render: Element = rect()
        .child(from_fn_standalone_borrowed_keyed(2, (), container))
        .child(from_fn_standalone_borrowed_keyed(1, (), container))
        .into();
    let first_render = PathElement::from_element(vec![], first_render);
    let second_render = PathElement::from_element(vec![], second_render);
    let mut diff = Diff::default();
    second_render.diff(Some(&first_render), &mut diff);
    assert!(diff.added.is_empty());
    assert!(diff.modified.is_empty());
    assert!(diff.removed.is_empty());
    assert_eq!(
        diff.moved,
        HashMap::from_iter([(Box::from([]), vec![(1, 0), (0, 1)])])
    );

    let first_render: Element = rect().child(rect().key(1)).child(rect().key(0)).into();
    let second_render: Element = rect().child(rect().key(0)).child(rect().key(1)).into();
    let first_render = PathElement::from_element(vec![], first_render);
    let second_render = PathElement::from_element(vec![], second_render);
    let mut diff = Diff::default();
    second_render.diff(Some(&first_render), &mut diff);
    assert!(diff.added.is_empty());
    assert!(diff.modified.is_empty());
    assert!(diff.removed.is_empty());
    assert_eq!(
        diff.moved,
        HashMap::from_iter([(Box::from([]), vec![(1, 0), (0, 1)])])
    );
}

#[test]
fn element_diffing6() {
    fn container(_: &()) -> Element {
        rect().into()
    }

    let first_render: Element = rect()
        .child(from_fn_standalone_borrowed_keyed(1, (), container))
        .child(from_fn_standalone_borrowed_keyed(2, (), container))
        .into();
    let second_render: Element = rect()
        .child(rect().child(from_fn_standalone_borrowed_keyed(2, (), container)))
        .into();
    let first_render = PathElement::from_element(vec![], first_render);
    let second_render = PathElement::from_element(vec![], second_render);
    let mut diff = Diff::default();
    second_render.diff(Some(&first_render), &mut diff);
    assert_eq!(
        diff.added,
        vec![vec![0].into_boxed_slice(), vec![0, 0].into_boxed_slice()]
    );
    assert!(diff.modified.is_empty());
    assert_eq!(
        diff.removed,
        vec![vec![0].into_boxed_slice(), vec![1].into_boxed_slice()]
    );
    assert!(diff.moved.is_empty());
}

#[test]
fn element_diffing7() {
    fn app() -> Element {
        let state = use_consume::<State<bool>>();

        if state() {
            rect()
                .child(rect().key(4))
                .child(rect().key(1))
                .child(rect().key(5))
                .child(rect().key(0))
                .into()
        } else {
            rect()
                .child(rect().key(3))
                .child(rect().key(0))
                .child(rect().key(1))
                .into()
        }
    }
    let mut runner = Runner::new(app);
    let mut tree = Tree::default();
    let mut state = runner.provide_root_context(|| State::create(true));
    let mutations = runner.sync_and_update();
    tree.apply_mutations(mutations);
    state.set(false);
    let mutations = runner.sync_and_update();
    assert_eq!(mutations.added.len(), 1);
    assert!(mutations.modified.is_empty());
    assert_eq!(mutations.removed.len(), 2);
    assert_eq!(mutations.moved.len(), 1);
    assert_eq!(mutations.moved.iter().next().unwrap().1.len(), 1);
    tree.apply_mutations(mutations);
}

#[test]
fn element_diffing8() {
    fn container(_: &()) -> Element {
        rect().into()
    }

    fn app() -> Element {
        let state = use_consume::<State<bool>>();

        if state() {
            rect()
                .child(rect().key(4))
                .child(from_fn_standalone_borrowed_keyed(1, (), container))
                .child(rect().key(5))
                .child(rect().key(0))
                .into()
        } else {
            rect()
                .child(rect().key(3))
                .child(rect().key(0))
                .child(from_fn_standalone_borrowed_keyed(1, (), container))
                .into()
        }
    }
    let mut runner = Runner::new(app);
    let mut tree = Tree::default();
    let mut state = runner.provide_root_context(|| State::create(true));
    let mutations = runner.sync_and_update();
    tree.apply_mutations(mutations);
    state.set(false);
    let mutations = runner.sync_and_update();
    assert_eq!(mutations.added.len(), 1);
    assert!(mutations.modified.is_empty());
    assert_eq!(mutations.removed.len(), 2);
    assert_eq!(mutations.moved.len(), 1);
    assert_eq!(mutations.moved.iter().next().unwrap().1.len(), 1);
    tree.apply_mutations(mutations);

    let mut runner = Runner::new(app);
    let mut tree = Tree::default();
    let mut state = runner.provide_root_context(|| State::create(false));
    let mutations = runner.sync_and_update();
    tree.apply_mutations(mutations);
    state.set(true);
    let mutations = runner.sync_and_update();
    assert_eq!(mutations.added.len(), 2);
    assert!(mutations.modified.is_empty());
    assert_eq!(mutations.removed.len(), 1);
    assert_eq!(mutations.moved.len(), 1);
    assert_eq!(mutations.moved.iter().next().unwrap().1.len(), 2);
    tree.apply_mutations(mutations);
}

#[test]
fn element_diffing9() {
    fn app() -> Element {
        let state = use_consume::<State<bool>>();

        rect()
            .maybe_child(state().then(rect))
            .child(rect().key(1))
            .child(rect().key(2))
            .into()
    }
    let mut runner = Runner::new(app);
    let mut tree = Tree::default();
    let mut state = runner.provide_root_context(|| State::create(false));
    let mutations = runner.sync_and_update();
    tree.apply_mutations(mutations);

    state.set(true);
    let mutations = runner.sync_and_update();
    assert_eq!(mutations.added.len(), 1);
    assert!(mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());
    assert!(mutations.moved.is_empty());
    tree.apply_mutations(mutations);
}

#[test]
fn element_diffing10() {
    fn container() -> Element {
        rect().into()
    }

    fn app() -> Element {
        let state = use_consume::<State<bool>>();

        if state() {
            rect()
                .child(rect().key(1).child(from_fn_standalone(container)))
                .child(rect().key(2).child(from_fn_standalone(container)))
                .into()
        } else {
            rect()
                .child(rect().key(2).child(from_fn_standalone(container)))
                .child(rect().key(1).child(from_fn_standalone(container)))
                .into()
        }
    }
    let mut runner = Runner::new(app);
    let mut tree = Tree::default();
    let mut state = runner.provide_root_context(|| State::create(false));
    let mutations = runner.sync_and_update();
    tree.apply_mutations(mutations);

    state.set(true);
    let mutations = runner.sync_and_update();

    assert!(mutations.added.is_empty());
    assert!(mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());
    assert_eq!(mutations.moved.len(), 1);
    assert_eq!(mutations.moved.iter().next().unwrap().1.len(), 1);
    assert_eq!(mutations.moved.iter().next().unwrap().1[0].0, 0);
    tree.apply_mutations(mutations);
}

#[test]
fn element_diffing11() {
    fn container(_: &()) -> Element {
        rect().into()
    }

    fn app() -> Element {
        let state = use_consume::<State<bool>>();

        if state() {
            rect()
                .child(from_fn_standalone_borrowed_keyed(1, (), container))
                .child(from_fn_standalone_borrowed_keyed(2, (), container))
                .into()
        } else {
            rect()
                .child(from_fn_standalone_borrowed_keyed(2, (), container))
                .into()
        }
    }
    let mut runner = Runner::new(app);
    let mut tree = Tree::default();
    let mut state = runner.provide_root_context(|| State::create(true));
    let mutations = runner.sync_and_update();

    assert_eq!(mutations.added.len(), 3);
    assert!(mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());
    assert!(mutations.moved.is_empty());
    tree.apply_mutations(mutations);

    state.set(false);
    let mutations = runner.sync_and_update();

    assert!(mutations.added.is_empty());
    assert!(mutations.modified.is_empty());
    assert_eq!(
        mutations.removed,
        vec![MutationRemove::Scope { id: 5.into() }]
    );
    assert!(mutations.moved.is_empty());
    tree.apply_mutations(mutations);
}

#[test]
fn element_diffing12() {
    fn container(_: &()) -> Element {
        rect().into()
    }

    fn app() -> Element {
        let state = use_consume::<State<bool>>();

        if state() {
            rect()
                .key(3)
                .child(from_fn_standalone_borrowed_keyed(2, (), container))
                .into()
        } else {
            rect()
                .key(4)
                .child(from_fn_standalone_borrowed_keyed(1, (), container))
                .into()
        }
    }
    let mut runner = Runner::new(app);
    let mut tree = Tree::default();
    let mut state = runner.provide_root_context(|| State::create(true));
    let mutations = runner.sync_and_update();

    assert_eq!(mutations.added.len(), 2);
    assert!(mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());
    assert!(mutations.moved.is_empty());
    tree.apply_mutations(mutations);

    state.set(false);
    let mutations = runner.sync_and_update();

    assert_eq!(mutations.added.len(), 2);
    assert_eq!(mutations.removed.len(), 2);
    assert!(mutations.modified.is_empty());
    assert!(mutations.moved.is_empty());
    tree.apply_mutations(mutations);
}

#[test]
fn element_diffing13() {
    let first_render: Element = rect()
        .child(
            rect()
                .child(rect().child(rect()))
                .child(rect().child(rect()))
                .child(rect()),
        )
        .child(rect())
        .into();
    let second_render: Element = rect()
        .key(2)
        .child(rect().child(rect()))
        .child(rect().child(rect()))
        .child(rect())
        .into();
    let first_render = PathElement::from_element(vec![], first_render);
    let second_render = PathElement::from_element(vec![], second_render);
    let mut diff = Diff::default();
    second_render.diff(Some(&first_render), &mut diff);

    assert_eq!(diff.added.len(), 6);
    assert!(diff.modified.is_empty());
    assert_eq!(diff.removed.len(), 7);
    assert!(diff.moved.is_empty());

    fn app() -> Element {
        let state = use_consume::<State<bool>>();

        if state() {
            rect()
                .child(
                    rect()
                        .child(rect().child(rect()))
                        .child(rect().child(rect()))
                        .child(rect()),
                )
                .child(rect())
                .into()
        } else {
            rect()
                .key(2)
                .child(rect().child(rect()))
                .child(rect().child(rect()))
                .child(rect())
                .into()
        }
    }
    let mut runner = Runner::new(app);
    let mut tree = Tree::default();
    let mut state = runner.provide_root_context(|| State::create(true));
    let mutations = runner.sync_and_update();

    tree.apply_mutations(mutations);

    state.set(false);
    let mutations = runner.sync_and_update();
    tree.apply_mutations(mutations);

    let first_render: Element = rect()
        .child(rect().key(2))
        .child(rect().child(rect().child(rect())))
        .into();
    let second_render: Element = rect()
        .child(rect().child(rect().key(3)).child(rect()))
        .into();
    let first_render = PathElement::from_element(vec![], first_render);
    let second_render = PathElement::from_element(vec![], second_render);
    let mut diff = Diff::default();
    second_render.diff(Some(&first_render), &mut diff);

    fn app3() -> Element {
        let state = use_consume::<State<bool>>();

        if state() {
            rect()
                .child(rect().key(2))
                .child(
                    rect()
                        .child(rect().child(rect()))
                        .child(rect().child(rect()))
                        .child(rect().child(rect())),
                )
                .into()
        } else {
            rect()
                .child(rect().child(rect().key(3)).child(rect()))
                .into()
        }
    }
    let mut runner = Runner::new(app3);
    let mut tree = Tree::default();
    let mut state = runner.provide_root_context(|| State::create(true));
    let mutations = runner.sync_and_update();

    tree.apply_mutations(mutations);

    state.set(false);
    let mutations = runner.sync_and_update();
    tree.apply_mutations(mutations);

    fn app4() -> Element {
        let state = use_consume::<State<i32>>();

        if state() == 1 {
            rect()
                .child(rect().key(0))
                .child(rect().key(1))
                .child(rect().key(2))
                .into()
        } else if state() == 2 {
            rect()
                .child(rect().key(3))
                .child(rect().key(4))
                .child(rect().key(5))
                .into()
        } else {
            rect()
                .child(rect().key(4))
                .child(rect().key(7))
                .child(rect().key(8))
                .into()
        }
    }
    let mut runner = Runner::new(app4);
    let mut tree = Tree::default();
    let mut state = runner.provide_root_context(|| State::create(0));
    let mutations = runner.sync_and_update();

    tree.apply_mutations(mutations);

    state.set(1);
    let mutations = runner.sync_and_update();
    tree.apply_mutations(mutations);

    state.set(2);
    let mutations = runner.sync_and_update();
    tree.apply_mutations(mutations);
}

#[test]
fn tree_unordered_mutations() {
    fn app() -> Element {
        let mut show = use_state(|| false);
        rect()
            .on_mouse_up(move |_| show.toggle())
            .maybe_child(show().then(|| from_fn_captured(|| counter())))
            .child(rect().key(1))
            .child(rect().key(2))
            .maybe_child(show().then(|| rect()))
            .into()
    }

    fn counter() -> Element {
        rect().into()
    }

    let mut runner = Runner::new(app);
    let mut tree = Tree::default();

    let mutations = runner.sync_and_update();
    assert!(!mutations.added.is_empty());
    assert!(mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());
    tree.apply_mutations(mutations);
    assert_eq!(
        tree.children,
        convert_ids(FxHashMap::from_iter([(1, vec![2]), (2, vec![3, 4]),]))
    );
    assert_eq!(
        runner
            .scopes
            .get(&ScopeId::ROOT)
            .unwrap()
            .borrow()
            .nodes
            .size(),
        4
    );

    runner.handle_event(
        2,
        EventName::MouseUp,
        EventType::Mouse(MouseEventData::default()),
        false,
    );
    let mutations = runner.sync_and_update();
    assert!(!mutations.added.is_empty());
    assert!(!mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());
    tree.apply_mutations(mutations);
    assert_eq!(
        tree.children,
        convert_ids(FxHashMap::from_iter([(1, vec![2]), (2, vec![7, 3, 4, 6]),]))
    );
    assert_eq!(
        runner
            .scopes
            .get(&ScopeId::ROOT)
            .unwrap()
            .borrow()
            .nodes
            .size(),
        6
    );
}
