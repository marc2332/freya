use freya::helpers::*;
use freya_core::{
    integration::*,
    prelude::*,
};
use freya_testing::TestingRunner;
use rustc_hash::FxHashMap;
use torin::size::Size;

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
fn event_propagate() {
    fn app() -> Element {
        rect()
            .children([from_fn_captured(|| {
                aaa(from_fn_captured(|| aaa(rect().into())))
            })])
            .into()
    }

    fn aaa(child: Element) -> Element {
        let mut value = use_state(|| 1);

        rect()
            .on_mouse_up(move |_| {
                *value.write() += 1;
            })
            .children([child, label().text(format!("{}", value.read())).into()])
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
    assert_eq!(
        tree.children,
        convert_ids(FxHashMap::from_iter([
            (1, vec![2]),
            (2, vec![4]),
            (4, vec![7, 6]),
            (7, vec![8, 9])
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
            (2, vec![4]),
            (4, vec![7, 6]),
            (7, vec![8, 9])
        ]))
    );

    // Addition
    runner.handle_event(
        4,
        EventName::MouseUp,
        EventType::Mouse(MouseEventData::default()),
        true,
    );
    let mutations = runner.sync_and_update();
    assert!(mutations.added.is_empty());
    assert!(!mutations.modified.is_empty());
    assert!(mutations.removed.is_empty());
    tree.apply_mutations(mutations);
    assert_eq!(
        tree.children,
        convert_ids(FxHashMap::from_iter([
            (1, vec![2]),
            (2, vec![4]),
            (4, vec![7, 6]),
            (7, vec![8, 9])
        ]))
    );
}

#[test]
fn touch_events() {
    fn app() -> Element {
        let mut state = use_consume::<State<i32>>();
        rect()
            .expanded()
            .background((255, 255, 255))
            .on_touch_start(move |_| *state.write() += 1)
            .on_touch_move(move |_| *state.write() += 2)
            .on_touch_end(move |_| *state.write() += 3)
            .on_touch_cancel(move |_| *state.write() += 4)
            .into()
    }

    let (mut test, state) = TestingRunner::new(
        app,
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(0)),
        1.,
    );
    test.sync_and_update();

    assert_eq!(*state.peek(), 0);

    test.send_event(PlatformEvent::Touch {
        name: TouchEventName::TouchStart,
        location: (15., 15.).into(),
        finger_id: 0,
        phase: TouchPhase::Started,
        force: None,
    });
    test.sync_and_update();

    assert_eq!(*state.peek(), 1);

    test.send_event(PlatformEvent::Touch {
        name: TouchEventName::TouchMove,
        location: (15., 15.).into(),
        finger_id: 0,
        phase: TouchPhase::Started,
        force: None,
    });
    test.sync_and_update();

    assert_eq!(*state.peek(), 3);

    test.send_event(PlatformEvent::Touch {
        name: TouchEventName::TouchEnd,
        location: (15., 15.).into(),
        finger_id: 0,
        phase: TouchPhase::Started,
        force: None,
    });
    test.sync_and_update();

    assert_eq!(*state.peek(), 6);

    test.send_event(PlatformEvent::Touch {
        name: TouchEventName::TouchCancel,
        location: (15., 15.).into(),
        finger_id: 0,
        phase: TouchPhase::Started,
        force: None,
    });
    test.sync_and_update();

    assert_eq!(*state.peek(), 10);
}

#[test]
fn pointer_events() {
    fn app() -> Element {
        let mut state = use_consume::<State<i32>>();
        rect()
            .expanded()
            .background((255, 255, 255))
            .on_pointer_down(move |_| *state.write() += 1)
            .on_pointer_press(move |_| *state.write() += 2)
            .into()
    }

    let (mut test, state) = TestingRunner::new(
        app,
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(0)),
        1.,
    );
    test.sync_and_update();

    assert_eq!(*state.peek(), 0);

    test.send_event(PlatformEvent::Touch {
        name: TouchEventName::TouchStart,
        location: (15., 15.).into(),
        finger_id: 0,
        phase: TouchPhase::Started,
        force: None,
    });
    test.sync_and_update();

    assert_eq!(*state.peek(), 1);

    test.send_event(PlatformEvent::Mouse {
        name: MouseEventName::MouseDown,
        cursor: (15., 15.).into(),
        button: None,
    });
    test.sync_and_update();

    assert_eq!(*state.peek(), 2);

    test.send_event(PlatformEvent::Touch {
        name: TouchEventName::TouchEnd,
        location: (15., 15.).into(),
        finger_id: 0,
        phase: TouchPhase::Started,
        force: None,
    });
    test.sync_and_update();

    assert_eq!(*state.peek(), 4);

    test.send_event(PlatformEvent::Mouse {
        name: MouseEventName::MouseUp,
        cursor: (15., 15.).into(),
        button: None,
    });
    test.sync_and_update();

    assert_eq!(*state.peek(), 6);
}

#[test]
fn pointer_enter_leave_at_large_coordinates() {
    fn app() -> Element {
        let mut state = use_consume::<State<i32>>();
        rect()
            .expanded()
            .background((255, 255, 255))
            .children([
                rect()
                    .width(Size::percent(100.))
                    .height(Size::px(3000.))
                    .into(),
                rect()
                    .width(Size::px(100.))
                    .height(Size::px(100.))
                    .background((0, 0, 0))
                    .on_pointer_enter(move |_| *state.write() += 1)
                    .on_pointer_leave(move |_| *state.write() += 10)
                    .into(),
                rect()
                    .width(Size::percent(100.))
                    .height(Size::px(100.))
                    .background((20, 20, 20))
                    .on_pointer_enter(move |_| *state.write() += 100)
                    .into(),
            ])
            .into()
    }

    let (mut test, state) = TestingRunner::new(
        app,
        (10_000., 4000.).into(),
        |runner| runner.provide_root_context(|| State::create(0)),
        1.,
    );
    test.sync_and_update();

    test.move_cursor((10., 10.));
    test.sync_and_update();
    assert_eq!(*state.peek(), 0);

    test.move_cursor((50., 3050.));
    test.sync_and_update();
    assert_eq!(*state.peek(), 1);

    test.move_cursor((60., 3060.));
    test.sync_and_update();
    assert_eq!(*state.peek(), 1);

    test.move_cursor((10., 10.));
    test.sync_and_update();
    assert_eq!(*state.peek(), 11);

    test.move_cursor((5_000., 3150.));
    test.sync_and_update();
    assert_eq!(*state.peek(), 111);
}

#[test]
fn large_coordinate_hover_does_not_trigger_other_elements() {
    #[derive(Clone, Copy)]
    struct Counters(State<(i32, i32)>);

    fn app() -> Element {
        let mut counters = use_consume::<Counters>().0;
        rect()
            .expanded()
            .background((255, 255, 255))
            .children([
                rect()
                    .width(Size::px(100.))
                    .height(Size::px(100.))
                    .background((0, 0, 0))
                    .on_pointer_enter(move |_| counters.write().0 += 1)
                    .on_pointer_leave(move |_| counters.write().0 += 10)
                    .into(),
                rect()
                    .width(Size::percent(100.))
                    .height(Size::px(2900.))
                    .into(),
                rect()
                    .width(Size::px(100.))
                    .height(Size::px(100.))
                    .background((40, 40, 40))
                    .on_pointer_enter(move |_| counters.write().1 += 1)
                    .on_pointer_leave(move |_| counters.write().1 += 10)
                    .into(),
            ])
            .into()
    }

    let (mut test, counters) = TestingRunner::new(
        app,
        (4000., 4000.).into(),
        |runner| runner.provide_root_context(|| Counters(State::create((0, 0)))),
        1.,
    );
    test.sync_and_update();

    test.move_cursor((200., 200.));
    test.sync_and_update();
    assert_eq!(*counters.0.peek(), (0, 0));

    test.move_cursor((50., 50.));
    test.sync_and_update();
    assert_eq!(*counters.0.peek(), (1, 0));

    test.move_cursor((200., 200.));
    test.sync_and_update();
    assert_eq!(*counters.0.peek(), (11, 0));

    test.move_cursor((50., 3050.));
    test.sync_and_update();
    assert_eq!(*counters.0.peek(), (11, 1));

    test.move_cursor((60., 3060.));
    test.sync_and_update();
    assert_eq!(*counters.0.peek(), (11, 1));

    test.move_cursor((200., 200.));
    test.sync_and_update();
    assert_eq!(*counters.0.peek(), (11, 11));
}
