#![allow(dead_code)]

use std::collections::{
    HashMap,
    HashSet,
};

use ragnarok::{
    Area,
    CursorPoint,
    EmmitableEvent,
    EventsExecutor,
    EventsExecutorRunner,
    EventsMeasurer,
    EventsMeasurerRunner,
    NameOfEvent,
    NodesState,
    SourceEvent,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum EventName {
    MouseEnter,
    MouseMove,
    MouseLeave,
    MouseDown,
    MouseUp,

    KeyboardDown,

    CaptureGlobalMouseMove,
}

impl PartialOrd for EventName {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            // Capture events have max priority
            e if e.is_capture() => std::cmp::Ordering::Less,
            // Left events have more priority over non-left
            e if e.is_left() => std::cmp::Ordering::Less,
            e => {
                if e == other {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Greater
                }
            }
        }
    }
}

impl EventName {
    pub fn is_capture(&self) -> bool {
        matches!(self, Self::CaptureGlobalMouseMove)
    }

    pub fn is_left(&self) -> bool {
        matches!(self, Self::MouseLeave)
    }
}

impl NameOfEvent for EventName {
    fn is_moved(&self) -> bool {
        matches!(self, Self::MouseMove | Self::CaptureGlobalMouseMove)
    }

    fn is_enter(&self) -> bool {
        matches!(self, Self::MouseEnter)
    }

    fn is_pressed(&self) -> bool {
        matches!(self, Self::MouseDown)
    }

    fn is_released(&self) -> bool {
        matches!(self, Self::MouseUp)
    }

    fn is_global(&self) -> bool {
        matches!(self, Self::CaptureGlobalMouseMove)
    }

    fn does_bubble(&self) -> bool {
        !matches!(self, Self::MouseLeave)
    }

    fn does_go_through_solid(&self) -> bool {
        true
    }

    fn new_leave() -> Self {
        Self::MouseLeave
    }

    fn get_derived_events(&self) -> Vec<Self> {
        let mut events = vec![*self];
        #[allow(clippy::single_match)]
        match self {
            Self::MouseMove => events.push(Self::MouseEnter),
            _ => {}
        }

        events
    }

    fn get_global_events(&self) -> Vec<Self> {
        match self {
            Self::MouseMove => vec![Self::CaptureGlobalMouseMove],
            _ => Vec::new(),
        }
    }

    fn get_cancellable_events(&self) -> Vec<Self> {
        let mut events = vec![*self];
        #[allow(clippy::single_match)]
        match self {
            Self::CaptureGlobalMouseMove => events.extend([Self::MouseMove, Self::MouseEnter]),
            _ => {}
        }
        events
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, PartialEq, Debug)]
enum TestSourceEvent {
    MouseDown { cursor: CursorPoint },
    MouseMove { cursor: CursorPoint },
    MouseUp { cursor: CursorPoint },
}

impl SourceEvent for TestSourceEvent {
    type Name = EventName;

    fn is_pressed(&self) -> bool {
        matches!(self, Self::MouseDown { .. })
    }

    fn is_moved(&self) -> bool {
        matches!(self, Self::MouseMove { .. })
    }

    fn try_cursor(&self) -> Option<ragnarok::CursorPoint> {
        match self {
            Self::MouseDown { cursor } => Some(*cursor),
            Self::MouseMove { cursor } => Some(*cursor),
            Self::MouseUp { cursor } => Some(*cursor),
        }
    }

    fn as_event_name(&self) -> Self::Name {
        match self {
            Self::MouseMove { .. } => EventName::MouseMove,
            Self::MouseDown { .. } => EventName::MouseDown,
            Self::MouseUp { .. } => EventName::MouseUp,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct TestEmmitableEvent {
    key: usize,
    name: EventName,
    source: EventName,
}

impl Eq for TestEmmitableEvent {}

impl PartialOrd for TestEmmitableEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TestEmmitableEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl EmmitableEvent for TestEmmitableEvent {
    type Key = usize;
    type Name = EventName;

    fn name(&self) -> Self::Name {
        self.name
    }

    fn source(&self) -> Self::Name {
        self.source
    }

    fn key(&self) -> Self::Key {
        self.key
    }
}

struct TestExecutor {
    emmited: Vec<TestEmmitableEvent>,
    handler: fn(&TestEmmitableEvent) -> bool,
}

impl TestExecutor {
    pub fn new(handler: fn(&TestEmmitableEvent) -> bool) -> Self {
        Self {
            emmited: Vec::default(),
            handler,
        }
    }

    pub fn without_handler() -> Self {
        Self {
            emmited: Vec::default(),
            handler: |_| true,
        }
    }
}

impl EventsExecutor for TestExecutor {
    type Name = EventName;

    type Key = usize;

    type Emmitable = TestEmmitableEvent;

    type Source = TestSourceEvent;

    fn emit_event(&mut self, event: Self::Emmitable) -> bool {
        let allowed = (self.handler)(&event);
        self.emmited.push(event);
        allowed
    }
}

#[derive(Default)]
struct TestMeasurer {
    layers: HashMap<i16, Vec<usize>>,
    children: HashMap<usize, HashSet<usize>>,
    listeners: HashMap<EventName, Vec<usize>>,
    areas: HashMap<usize, Area>,
}

impl TestMeasurer {
    fn add(&mut self, id: usize, parent: Option<usize>, layer: i16, area: Area) {
        self.layers.entry(layer).or_default().push(id);
        if let Some(parent) = parent {
            self.children.entry(parent).or_default().insert(parent);
        }
        self.areas.insert(id, area);
    }

    fn listen_to(&mut self, id: usize, event: EventName) {
        self.listeners.entry(event).or_default().push(id);
    }
}

impl EventsMeasurer for TestMeasurer {
    type Name = EventName;

    type Key = usize;

    type Emmitable = TestEmmitableEvent;

    type Source = TestSourceEvent;

    fn get_layers(&self) -> impl Iterator<Item = (&i16, impl Iterator<Item = &Self::Key>)> {
        self.layers
            .iter()
            .map(|(layer, nodes)| (layer, nodes.iter()))
    }

    fn get_listeners_of(&self, name: &Self::Name) -> Vec<Self::Key> {
        self.listeners.get(name).cloned().unwrap_or_default()
    }

    fn is_point_inside(&self, key: Self::Key, cursor: ragnarok::CursorPoint) -> bool {
        self.areas.get(&key).unwrap().contains(cursor.to_f32())
    }

    fn is_node_parent_of(&self, key: Self::Key, parent: Self::Key) -> bool {
        self.children.get(&parent).unwrap().contains(&key)
    }

    fn is_listening_to(&self, key: Self::Key, name: &Self::Name) -> bool {
        let Some(listeners) = self.listeners.get(name) else {
            return false;
        };
        listeners.contains(&key)
    }

    fn is_node_transparent(&self, _key: Self::Key) -> bool {
        false
    }

    fn try_area_of(&self, key: Self::Key) -> Option<ragnarok::Area> {
        self.areas.get(&key).cloned()
    }

    fn new_emmitable_event(
        &self,
        key: Self::Key,
        name: Self::Name,
        source: Self::Source,
        _area: Option<ragnarok::Area>,
    ) -> Self::Emmitable {
        TestEmmitableEvent {
            key,
            name,
            source: source.as_event_name(),
        }
    }
}

#[test]
fn enter_leave_events() {
    let mut test_measurer = TestMeasurer::default();
    let mut nodes_state = NodesState::default();

    test_measurer.add(0, None, 0, Area::new((0., 0.).into(), (100., 100.).into()));
    test_measurer.listen_to(0, EventName::MouseEnter);
    test_measurer.listen_to(0, EventName::MouseLeave);

    // Move the mouse over the node a few times
    for i in 0..25 {
        let processed_events = test_measurer.run(
            &mut vec![TestSourceEvent::MouseMove {
                cursor: (5. + i as f64, 5. + i as f64).into(),
            }],
            &mut nodes_state,
            None,
        );
        match i {
            0 => {
                // Assert an enter event is to be emitted
                assert_eq!(
                    processed_events.emmitable_events.first(),
                    Some(&TestEmmitableEvent {
                        key: 0,
                        name: EventName::MouseEnter,
                        source: EventName::MouseMove
                    })
                );
            }
            _ => {
                // No more enter events will be emitted as long as the mouse does not leave and enter again
                assert!(processed_events.emmitable_events.is_empty())
            }
        }
        // Apply the processed events
        TestExecutor::without_handler().run(&mut nodes_state, processed_events);
        // Assert that the node is indeed being hovered
        assert!(nodes_state.is_hovered(0));
    }

    // Put the mouse outside the node
    let processed_events = test_measurer.run(
        &mut vec![TestSourceEvent::MouseMove {
            cursor: (600., 700.).into(),
        }],
        &mut nodes_state,
        None,
    );

    // Assert a leave event is to be emitted
    assert_eq!(
        processed_events.emmitable_events.first(),
        Some(&TestEmmitableEvent {
            key: 0,
            name: EventName::MouseLeave,
            source: EventName::MouseMove
        })
    );
    // Apply the processed events
    TestExecutor::without_handler().run(&mut nodes_state, processed_events);
    // Assert that the node is indeed not being hovered anymore
    assert!(!nodes_state.is_hovered(0));
}

#[test]
fn accurate_down_up_event() {
    let mut test_measurer = TestMeasurer::default();
    let mut nodes_state = NodesState::default();

    test_measurer.add(0, None, 0, Area::new((0., 0.).into(), (100., 100.).into()));
    test_measurer.listen_to(0, EventName::MouseDown);
    test_measurer.listen_to(0, EventName::MouseUp);

    let processed_events = test_measurer.run(
        &mut vec![TestSourceEvent::MouseDown {
            cursor: (25., 25.).into(),
        }],
        &mut nodes_state,
        None,
    );

    // Assert an enter event is to be emitted
    assert_eq!(
        processed_events.emmitable_events.first(),
        Some(&TestEmmitableEvent {
            key: 0,
            name: EventName::MouseDown,
            source: EventName::MouseDown
        })
    );
    // Apply the processed events
    TestExecutor::without_handler().run(&mut nodes_state, processed_events);
    // Assert that the node is indeed being pressed
    assert!(nodes_state.is_pressed(0));

    // Click outside the node
    let processed_events = test_measurer.run(
        &mut vec![TestSourceEvent::MouseUp {
            cursor: (25., 25.).into(),
        }],
        &mut nodes_state,
        None,
    );

    // Assert a leave event is to be emitted
    assert_eq!(
        processed_events.emmitable_events.first(),
        Some(&TestEmmitableEvent {
            key: 0,
            name: EventName::MouseUp,
            source: EventName::MouseUp
        })
    );
    // Apply the processed events
    TestExecutor::without_handler().run(&mut nodes_state, processed_events);
    // Assert that the node is indeed not being pressed anymore
    assert!(nodes_state.is_pressed(0));
}

#[test]
fn missed_up_event() {
    let mut test_measurer = TestMeasurer::default();
    let mut nodes_state = NodesState::default();

    test_measurer.add(0, None, 0, Area::new((0., 0.).into(), (100., 100.).into()));
    test_measurer.listen_to(0, EventName::MouseDown);
    test_measurer.listen_to(0, EventName::MouseUp);

    let processed_events = test_measurer.run(
        &mut vec![TestSourceEvent::MouseDown {
            cursor: (25., 25.).into(),
        }],
        &mut nodes_state,
        None,
    );

    // Assert an enter event is to be emitted
    assert_eq!(
        processed_events.emmitable_events.first(),
        Some(&TestEmmitableEvent {
            key: 0,
            name: EventName::MouseDown,
            source: EventName::MouseDown
        })
    );
    // Apply the processed events
    TestExecutor::without_handler().run(&mut nodes_state, processed_events);
    // Assert that the node is indeed being pressed
    assert!(nodes_state.is_pressed(0));

    // Click outside the node
    let processed_events = test_measurer.run(
        &mut vec![TestSourceEvent::MouseUp {
            cursor: (600., 700.).into(),
        }],
        &mut nodes_state,
        None,
    );
    // Apply the processed events
    TestExecutor::without_handler().run(&mut nodes_state, processed_events);
    // Assert that the node is indeed not being pressed anymore
    assert!(nodes_state.is_pressed(0));
}

#[test]
fn state_updates_without_listeners() {
    let mut test_measurer = TestMeasurer::default();
    let mut nodes_state = NodesState::default();

    test_measurer.add(0, None, 0, Area::new((0., 0.).into(), (100., 100.).into()));

    let processed_events = test_measurer.run(
        &mut vec![TestSourceEvent::MouseDown {
            cursor: (25., 25.).into(),
        }],
        &mut nodes_state,
        None,
    );

    // Assert no event is to be emitted
    assert!(processed_events.emmitable_events.is_empty());
    // Apply the processed events
    TestExecutor::without_handler().run(&mut nodes_state, processed_events);
    // Assert that the node is indeed being pressed
    assert!(nodes_state.is_pressed(0));

    // Click outside the node
    let processed_events = test_measurer.run(
        &mut vec![TestSourceEvent::MouseUp {
            cursor: (600., 700.).into(),
        }],
        &mut nodes_state,
        None,
    );
    assert!(processed_events.emmitable_events.is_empty());
    // Apply the processed events
    TestExecutor::without_handler().run(&mut nodes_state, processed_events);
    // Assert that the node is indeed not being pressed anymore
    assert!(nodes_state.is_pressed(0));
}

#[test]
fn cancel_emmited_events_on_capture() {
    let mut test_measurer = TestMeasurer::default();
    let mut nodes_state = NodesState::default();

    test_measurer.add(0, None, 0, Area::new((0., 0.).into(), (100., 100.).into()));
    test_measurer.listen_to(0, EventName::MouseEnter);

    test_measurer.add(
        1,
        None,
        0,
        Area::new((200., 200.).into(), (100., 100.).into()),
    );
    test_measurer.listen_to(1, EventName::CaptureGlobalMouseMove);

    let processed_events = test_measurer.run(
        &mut vec![TestSourceEvent::MouseMove {
            cursor: (25., 25.).into(),
        }],
        &mut nodes_state,
        None,
    );

    // Assert an enter event is to be emitted
    assert_eq!(
        processed_events.emmitable_events,
        vec![
            TestEmmitableEvent {
                key: 1,
                name: EventName::CaptureGlobalMouseMove,
                source: EventName::MouseMove
            },
            TestEmmitableEvent {
                key: 0,
                name: EventName::MouseEnter,
                source: EventName::MouseMove
            },
        ]
    );
    // Apply the processed events
    TestExecutor::new(|_| false).run(&mut nodes_state, processed_events);
    // Assert that the node is not being hovvered as the event was can cancelled
    assert!(!nodes_state.is_hovered(0));
}
