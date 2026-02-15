use std::{
    any::TypeId,
    cell::RefCell,
    cmp::Ordering,
    collections::{
        HashMap,
        HashSet,
        VecDeque,
    },
    fmt::Debug,
    rc::Rc,
    sync::atomic::AtomicU64,
};

use futures_lite::{
    FutureExt,
    StreamExt,
};
use itertools::Itertools;
use pathgraph::PathGraph;
use rustc_hash::{
    FxHashMap,
    FxHashSet,
};

use crate::{
    current_context::CurrentContext,
    diff_key::DiffKey,
    element::{
        Element,
        ElementExt,
        EventHandlerType,
    },
    events::{
        data::{
            Event,
            EventType,
        },
        name::EventName,
    },
    node_id::NodeId,
    path_element::PathElement,
    prelude::{
        Task,
        TaskId,
    },
    reactive_context::ReactiveContext,
    scope::{
        PathNode,
        Scope,
        ScopeStorage,
    },
    scope_id::ScopeId,
    tree::DiffModifies,
};

#[derive(Debug, PartialEq, Eq)]
pub enum MutationRemove {
    /// Because elements always have a different parent we can easily get their position relatively to their parent
    Element { id: NodeId, index: u32 },
    /// In the other hand, roots of Scopes are manually connected to their parent scopes, so getting their index is not worth the effort.
    Scope { id: NodeId },
}

impl PartialOrd for MutationRemove {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MutationRemove {
    fn cmp(&self, other: &Self) -> Ordering {
        use MutationRemove::*;
        match (self, other) {
            // Order Element removals by index descending (so larger indices come first)
            (Element { index: a, .. }, Element { index: b, .. }) => b.cmp(a),
            // Elements come before Scopes
            (Element { .. }, Scope { .. }) => Ordering::Less,
            (Scope { .. }, Element { .. }) => Ordering::Greater,
            // Order Scopes by id descending as well
            (Scope { id: a }, Scope { id: b }) => b.cmp(a),
        }
    }
}

impl MutationRemove {
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Element { id, .. } => *id,
            Self::Scope { id } => *id,
        }
    }
}

#[derive(Default)]
pub struct Mutations {
    pub added: Vec<(NodeId, NodeId, u32, Rc<dyn ElementExt>)>,
    pub modified: Vec<(NodeId, Rc<dyn ElementExt>, DiffModifies)>,
    pub removed: Vec<MutationRemove>,
    pub moved: HashMap<NodeId, Vec<(u32, NodeId)>>,
}

impl Debug for Mutations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Added: {:?} Modified: {:?} Removed: {:?} Moved: {:?}",
            self.added
                .iter()
                .map(|(a, b, c, _)| (*a, *b, *c))
                .collect::<Vec<_>>(),
            self.modified.iter().map(|(a, _, _)| *a).collect::<Vec<_>>(),
            self.removed,
            self.moved
        ))
    }
}

pub enum Message {
    MarkScopeAsDirty(ScopeId),
    PollTask(TaskId),
}

pub struct Runner {
    pub scopes: FxHashMap<ScopeId, Rc<RefCell<Scope>>>,
    pub scopes_storages: Rc<RefCell<FxHashMap<ScopeId, ScopeStorage>>>,

    pub(crate) dirty_scopes: FxHashSet<ScopeId>,
    pub(crate) dirty_tasks: VecDeque<TaskId>,

    pub node_to_scope: FxHashMap<NodeId, ScopeId>,

    pub(crate) node_id_counter: NodeId,
    pub(crate) scope_id_counter: ScopeId,
    pub(crate) task_id_counter: Rc<AtomicU64>,

    pub(crate) tasks: Rc<RefCell<FxHashMap<TaskId, Rc<RefCell<Task>>>>>,

    pub(crate) sender: futures_channel::mpsc::UnboundedSender<Message>,
    pub(crate) receiver: futures_channel::mpsc::UnboundedReceiver<Message>,
}

impl Debug for Runner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Runner")
            .field("dirty_scopes", &self.dirty_scopes.len())
            .field("dirty_tasks", &self.dirty_tasks.len())
            .field("node_to_scope", &self.node_to_scope.len())
            .field("scopes", &self.scopes.len())
            .field("scopes_storages", &self.scopes_storages.borrow().len())
            .field("tasks", &self.tasks.borrow().len())
            .finish()
    }
}

impl Drop for Runner {
    fn drop(&mut self) {
        // Graceful shutdown of scopes based on their height, starting from the deepest
        for (scope_id, _scope) in self
            .scopes
            .drain()
            .sorted_by_key(|s| s.1.borrow().height)
            .rev()
        {
            CurrentContext::run_with_reactive(
                CurrentContext {
                    scope_id,
                    scopes_storages: self.scopes_storages.clone(),
                    tasks: self.tasks.clone(),
                    task_id_counter: self.task_id_counter.clone(),
                    sender: self.sender.clone(),
                },
                || {
                    let _scope = self.scopes_storages.borrow_mut().remove(&scope_id);
                },
            );
        }
    }
}

impl Runner {
    pub fn new(root: impl Fn() -> Element + 'static) -> Self {
        let (sender, receiver) = futures_channel::mpsc::unbounded::<Message>();
        Self {
            scopes: FxHashMap::from_iter([(
                ScopeId::ROOT,
                Rc::new(RefCell::new(Scope {
                    parent_node_id_in_parent: NodeId::ROOT,
                    path_in_parent: Box::from([]),
                    height: 0,
                    parent_id: None,
                    id: ScopeId::ROOT,
                    key: DiffKey::Root,
                    comp: Rc::new(move |_| root()),
                    props: Rc::new(()),
                    element: None,
                    nodes: {
                        let mut map = PathGraph::new();
                        map.insert(
                            &[],
                            PathNode {
                                node_id: NodeId::ROOT,
                                scope_id: None,
                            },
                        );
                        map
                    },
                })),
            )]),
            scopes_storages: Rc::new(RefCell::new(FxHashMap::from_iter([(
                ScopeId::ROOT,
                ScopeStorage::new(None, |writer| {
                    ReactiveContext::new_for_scope(sender.clone(), ScopeId::ROOT, writer)
                }),
            )]))),

            node_to_scope: FxHashMap::from_iter([(NodeId::ROOT, ScopeId::ROOT)]),

            node_id_counter: NodeId::ROOT,
            scope_id_counter: ScopeId::ROOT,
            task_id_counter: Rc::default(),

            dirty_tasks: VecDeque::default(),
            dirty_scopes: FxHashSet::from_iter([ScopeId::ROOT]),

            tasks: Rc::default(),

            sender,
            receiver,
        }
    }

    #[cfg(all(debug_assertions, feature = "debug-integrity"))]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn verify_scopes_integrity(&self) {
        let mut visited = FxHashSet::default();
        let size = self.scopes.len();
        let mut buffer = vec![ScopeId::ROOT];
        while let Some(scope_id) = buffer.pop() {
            if visited.contains(&scope_id) {
                continue;
            }
            visited.insert(scope_id);
            let scope = self.scopes.get(&scope_id).unwrap();
            let scope = scope.borrow();
            if let Some(parent) = scope.parent_id {
                buffer.push(parent);
            }
            scope.nodes.traverse(&[], |_, &PathNode { scope_id, .. }| {
                if let Some(scope_id) = scope_id {
                    buffer.push(scope_id);
                }
            });
        }
        assert_eq!(size, visited.len())
    }

    pub fn provide_root_context<T: 'static + Clone>(&mut self, context: impl FnOnce() -> T) -> T {
        CurrentContext::run(
            CurrentContext {
                scope_id: ScopeId::ROOT,
                scopes_storages: self.scopes_storages.clone(),
                tasks: self.tasks.clone(),
                task_id_counter: self.task_id_counter.clone(),
                sender: self.sender.clone(),
            },
            move || {
                let context = context();
                let mut scopes_storages = self.scopes_storages.borrow_mut();
                let root_scope_storage = scopes_storages.get_mut(&ScopeId::ROOT).unwrap();
                root_scope_storage
                    .contexts
                    .insert(TypeId::of::<T>(), Rc::new(context.clone()));

                context
            },
        )
    }

    pub fn handle_event(
        &mut self,
        node_id: impl Into<NodeId>,
        event_name: EventName,
        event_type: EventType,
        bubbles: bool,
    ) -> bool {
        let node_id = node_id.into();
        #[cfg(debug_assertions)]
        tracing::info!("Handling event {event_name:?} for {node_id:?}");
        let propagate = Rc::new(RefCell::new(bubbles));
        let default = Rc::new(RefCell::new(true));

        let Some(scope_id) = self.node_to_scope.get(&node_id) else {
            return false;
        };
        let Some(path) = self
            .scopes
            .get(scope_id)
            .unwrap()
            .borrow()
            .nodes
            .find_path(|value| {
                value
                    == Some(&PathNode {
                        node_id,
                        scope_id: None,
                    })
            })
        else {
            return false;
        };

        let mut current_target = Some((path, *scope_id));
        while let Some((path, scope_id)) = current_target.take() {
            let scope = self.scopes.get(&scope_id).cloned().unwrap();
            scope.borrow().with_element(&path, |element| {
                match element {
                    PathElement::Component { .. } => {
                        unreachable!()
                    }
                    PathElement::Element { element, .. } => {
                        CurrentContext::run(
                            CurrentContext {
                                scope_id,
                                scopes_storages: self.scopes_storages.clone(),
                                tasks: self.tasks.clone(),
                                task_id_counter: self.task_id_counter.clone(),
                                sender: self.sender.clone(),
                            },
                            || {
                                match &event_type {
                                    EventType::Mouse(data) => {
                                        let event_handlers = element.events_handlers();
                                        if let Some(event_handlers) = event_handlers {
                                            match event_handlers.get(&event_name) {
                                                Some(EventHandlerType::Mouse(handler)) => {
                                                    handler.call(Event {
                                                        data: data.clone(),
                                                        propagate: propagate.clone(),
                                                        default: default.clone(),
                                                    });
                                                }
                                                Some(_) => unreachable!(),
                                                _ => {}
                                            }
                                        }
                                    }
                                    EventType::Keyboard(data) => {
                                        let event_handlers = element.events_handlers();
                                        if let Some(event_handlers) = event_handlers {
                                            match event_handlers.get(&event_name) {
                                                Some(EventHandlerType::Keyboard(handler)) => {
                                                    handler.call(Event {
                                                        data: data.clone(),
                                                        propagate: propagate.clone(),
                                                        default: default.clone(),
                                                    });
                                                }
                                                Some(_) => unreachable!(),
                                                _ => {}
                                            }
                                        }
                                    }
                                    EventType::Sized(data) => {
                                        let event_handlers = element.events_handlers();
                                        if let Some(event_handlers) = event_handlers {
                                            match event_handlers.get(&event_name) {
                                                Some(EventHandlerType::Sized(handler)) => {
                                                    handler.call(Event {
                                                        data: data.clone(),
                                                        propagate: propagate.clone(),
                                                        default: default.clone(),
                                                    });
                                                }
                                                Some(_) => unreachable!(),
                                                _ => {}
                                            }
                                        }
                                    }
                                    EventType::Wheel(data) => {
                                        let event_handlers = element.events_handlers();
                                        if let Some(event_handlers) = event_handlers {
                                            match event_handlers.get(&event_name) {
                                                Some(EventHandlerType::Wheel(handler)) => {
                                                    handler.call(Event {
                                                        data: data.clone(),
                                                        propagate: propagate.clone(),
                                                        default: default.clone(),
                                                    });
                                                }
                                                Some(_) => unreachable!(),
                                                _ => {}
                                            }
                                        }
                                    }
                                    EventType::Touch(data) => {
                                        let event_handlers = element.events_handlers();
                                        if let Some(event_handlers) = event_handlers {
                                            match event_handlers.get(&event_name) {
                                                Some(EventHandlerType::Touch(handler)) => {
                                                    handler.call(Event {
                                                        data: data.clone(),
                                                        propagate: propagate.clone(),
                                                        default: default.clone(),
                                                    });
                                                }
                                                Some(_) => unreachable!(),
                                                _ => {}
                                            }
                                        }
                                    }
                                    EventType::Pointer(data) => {
                                        let event_handlers = element.events_handlers();
                                        if let Some(event_handlers) = event_handlers {
                                            match event_handlers.get(&event_name) {
                                                Some(EventHandlerType::Pointer(handler)) => {
                                                    handler.call(Event {
                                                        data: data.clone(),
                                                        propagate: propagate.clone(),
                                                        default: default.clone(),
                                                    });
                                                }
                                                Some(_) => unreachable!(),
                                                _ => {}
                                            }
                                        }
                                    }
                                    EventType::File(data) => {
                                        let event_handlers = element.events_handlers();
                                        if let Some(event_handlers) = event_handlers {
                                            match event_handlers.get(&event_name) {
                                                Some(EventHandlerType::File(handler)) => {
                                                    handler.call(Event {
                                                        data: data.clone(),
                                                        propagate: propagate.clone(),
                                                        default: default.clone(),
                                                    });
                                                }
                                                Some(_) => unreachable!(),
                                                _ => {}
                                            }
                                        }
                                    }
                                    EventType::ImePreedit(data) => {
                                        let event_handlers = element.events_handlers();
                                        if let Some(event_handlers) = event_handlers {
                                            match event_handlers.get(&event_name) {
                                                Some(EventHandlerType::ImePreedit(handler)) => {
                                                    handler.call(Event {
                                                        data: data.clone(),
                                                        propagate: propagate.clone(),
                                                        default: default.clone(),
                                                    });
                                                }
                                                Some(_) => unreachable!(),
                                                _ => {}
                                            }
                                        }
                                    }
                                }

                                // Bubble up if desired
                                if *propagate.borrow() {
                                    if path.len() > 1 {
                                        // Change the target to this element parent (still in the same Scope)
                                        current_target
                                            .replace((path[..path.len() - 1].to_vec(), scope_id));
                                    } else {
                                        let mut parent_scope_id = scope.borrow().parent_id;
                                        // Otherwise change the target to this element parent in the parent Scope
                                        loop {
                                            if let Some(parent_id) = parent_scope_id.take() {
                                                let parent_scope =
                                                    self.scopes.get(&parent_id).unwrap();
                                                let path = parent_scope.borrow().nodes.find_path(
                                                    |value| {
                                                        value
                                                            == Some(&PathNode {
                                                                node_id: scope
                                                                    .borrow()
                                                                    .parent_node_id_in_parent,
                                                                scope_id: None,
                                                            })
                                                    },
                                                );
                                                if let Some(path) = path {
                                                    current_target.replace((path, parent_id));
                                                    break;
                                                } else {
                                                    parent_scope_id =
                                                        parent_scope.borrow().parent_id;
                                                }
                                            } else {
                                                return;
                                            }
                                        }
                                    }
                                }
                            },
                        )
                    }
                }
            });
        }
        *default.borrow()
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub async fn handle_events(&mut self) {
        loop {
            while let Ok(Some(msg)) = self.receiver.try_recv() {
                match msg {
                    Message::MarkScopeAsDirty(scope_id) => {
                        self.dirty_scopes.insert(scope_id);
                    }
                    Message::PollTask(task_id) => {
                        self.dirty_tasks.push_back(task_id);
                    }
                }
            }

            if !self.dirty_scopes.is_empty() {
                return;
            }

            while let Some(task_id) = self.dirty_tasks.pop_front() {
                let Some(task) = self.tasks.borrow().get(&task_id).cloned() else {
                    continue;
                };
                let mut task = task.borrow_mut();
                let waker = task.waker.clone();

                let mut cx = std::task::Context::from_waker(&waker);

                CurrentContext::run(
                    {
                        let Some(scope) = self.scopes.get(&task.scope_id) else {
                            continue;
                        };
                        CurrentContext {
                            scope_id: scope.borrow().id,
                            scopes_storages: self.scopes_storages.clone(),
                            tasks: self.tasks.clone(),
                            task_id_counter: self.task_id_counter.clone(),
                            sender: self.sender.clone(),
                        }
                    },
                    || {
                        let poll_result = task.future.poll(&mut cx);
                        if poll_result.is_ready() {
                            self.tasks.borrow_mut().remove(&task_id);
                        }
                    },
                );
            }

            if !self.dirty_scopes.is_empty() {
                return;
            }

            while let Some(msg) = self.receiver.next().await {
                match msg {
                    Message::MarkScopeAsDirty(scope_id) => {
                        self.dirty_scopes.insert(scope_id);
                    }
                    Message::PollTask(task_id) => {
                        self.dirty_tasks.push_back(task_id);
                    }
                }
            }
        }
    }

    /// Useful for freya-testing
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn handle_events_immediately(&mut self) {
        while let Ok(Some(msg)) = self.receiver.try_recv() {
            match msg {
                Message::MarkScopeAsDirty(scope_id) => {
                    self.dirty_scopes.insert(scope_id);
                }
                Message::PollTask(task_id) => {
                    self.dirty_tasks.push_back(task_id);
                }
            }
        }

        if !self.dirty_scopes.is_empty() {
            return;
        }

        // Poll here
        while let Some(task_id) = self.dirty_tasks.pop_front() {
            let Some(task) = self.tasks.borrow().get(&task_id).cloned() else {
                continue;
            };
            let mut task = task.borrow_mut();
            let waker = task.waker.clone();

            let mut cx = std::task::Context::from_waker(&waker);

            CurrentContext::run(
                {
                    let Some(scope) = self.scopes.get(&task.scope_id) else {
                        continue;
                    };
                    CurrentContext {
                        scope_id: scope.borrow().id,
                        scopes_storages: self.scopes_storages.clone(),
                        tasks: self.tasks.clone(),
                        task_id_counter: self.task_id_counter.clone(),
                        sender: self.sender.clone(),
                    }
                },
                || {
                    let poll_result = task.future.poll(&mut cx);
                    if poll_result.is_ready() {
                        self.tasks.borrow_mut().remove(&task_id);
                    }
                },
            );
        }
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn sync_and_update(&mut self) -> Mutations {
        self.handle_events_immediately();
        use itertools::Itertools;

        #[cfg(all(debug_assertions, feature = "debug-integrity"))]
        self.verify_scopes_integrity();

        let mut mutations = Mutations::default();

        let dirty_scopes = self
            .dirty_scopes
            .drain()
            .filter_map(|id| self.scopes.get(&id).cloned())
            .sorted_by_key(|s| s.borrow().height)
            .map(|s| s.borrow().id)
            .collect::<Box<[_]>>();

        let mut visited_scopes = FxHashSet::default();

        for scope_id in dirty_scopes {
            // No need to run scopes more than once
            if visited_scopes.contains(&scope_id) {
                continue;
            }

            let Some(scope_rc) = self.scopes.get(&scope_id).cloned() else {
                continue;
            };

            let scope_id = scope_rc.borrow().id;

            let element = CurrentContext::run_with_reactive(
                CurrentContext {
                    scope_id,
                    scopes_storages: self.scopes_storages.clone(),
                    tasks: self.tasks.clone(),
                    task_id_counter: self.task_id_counter.clone(),
                    sender: self.sender.clone(),
                },
                || {
                    let scope = scope_rc.borrow();
                    (scope.comp)(scope.props.clone())
                },
            );

            let path_element = PathElement::from_element(vec![0], element);
            let mut diff = Diff::default();
            path_element.diff(scope_rc.borrow().element.as_ref(), &mut diff);

            self.apply_diff(&scope_rc, diff, &mut mutations, &path_element);

            self.run_scope(
                &scope_rc,
                &path_element,
                &mut mutations,
                &mut visited_scopes,
            );

            let mut scopes_storages = self.scopes_storages.borrow_mut();
            let scope_storage = scopes_storages.get_mut(&scope_rc.borrow().id).unwrap();
            scope_storage.current_value = 0;
            scope_storage.current_run += 1;

            scope_rc.borrow_mut().element = Some(path_element);
        }

        mutations
    }

    pub fn run_in<T>(&self, run: impl FnOnce() -> T) -> T {
        CurrentContext::run(
            CurrentContext {
                scope_id: ScopeId::ROOT,
                scopes_storages: self.scopes_storages.clone(),
                tasks: self.tasks.clone(),
                task_id_counter: self.task_id_counter.clone(),
                sender: self.sender.clone(),
            },
            run,
        )
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn run_scope(
        &mut self,
        scope: &Rc<RefCell<Scope>>,
        element: &PathElement,
        mutations: &mut Mutations,
        visited_scopes: &mut FxHashSet<ScopeId>,
    ) {
        visited_scopes.insert(scope.borrow().id);
        match element {
            PathElement::Component {
                comp,
                props,
                key,
                path,
            } => {
                // Safe to unwrap because this is a component
                let assigned_scope_id = scope
                    .borrow()
                    .nodes
                    .get(path)
                    .and_then(|path_node| path_node.scope_id)
                    .unwrap();

                let parent_node_id = if path.as_ref() == [0] {
                    scope.borrow().parent_node_id_in_parent
                } else {
                    scope
                        .borrow()
                        .nodes
                        .get(&path[..path.len() - 1])
                        .unwrap()
                        .node_id
                };

                if let Some(Ok(mut existing_scope)) = self
                    .scopes
                    .get(&assigned_scope_id)
                    .map(|s| s.try_borrow_mut())
                {
                    let key_changed = existing_scope.key != *key;
                    if key_changed || existing_scope.props.changed(props.as_ref()) {
                        self.dirty_scopes.insert(assigned_scope_id);
                        existing_scope.props = props.clone();

                        if key_changed {
                            self.scopes_storages
                                .borrow_mut()
                                .get_mut(&assigned_scope_id)
                                .unwrap()
                                .reset();
                        }
                    }
                } else {
                    self.scopes.insert(
                        assigned_scope_id,
                        Rc::new(RefCell::new(Scope {
                            parent_node_id_in_parent: parent_node_id,
                            path_in_parent: path.clone(),
                            height: scope.borrow().height + 1,
                            parent_id: Some(scope.borrow().id),
                            id: assigned_scope_id,
                            key: key.clone(),
                            comp: comp.clone(),
                            props: props.clone(),
                            element: None,
                            nodes: PathGraph::default(),
                        })),
                    );
                    self.scopes_storages.borrow_mut().insert(
                        assigned_scope_id,
                        ScopeStorage::new(Some(scope.borrow().id), |writer| {
                            ReactiveContext::new_for_scope(
                                self.sender.clone(),
                                assigned_scope_id,
                                writer,
                            )
                        }),
                    );
                    self.dirty_scopes.insert(assigned_scope_id);
                }

                let was_dirty = self.dirty_scopes.remove(&assigned_scope_id);

                if !was_dirty {
                    // No need to rerun scope if it is not dirty
                    return;
                }

                let scope_rc = self.scopes.get(&assigned_scope_id).cloned().unwrap();

                let element = hotpath::measure_block!("Scope Rendering", {
                    CurrentContext::run_with_reactive(
                        CurrentContext {
                            scope_id: assigned_scope_id,
                            scopes_storages: self.scopes_storages.clone(),
                            tasks: self.tasks.clone(),
                            task_id_counter: self.task_id_counter.clone(),
                            sender: self.sender.clone(),
                        },
                        || {
                            let scope = scope_rc.borrow();
                            (scope.comp)(scope.props.clone())
                        },
                    )
                });

                let path_element = PathElement::from_element(vec![0], element);
                let mut diff = Diff::default();
                path_element.diff(scope_rc.borrow().element.as_ref(), &mut diff);

                self.apply_diff(&scope_rc, diff, mutations, &path_element);

                self.run_scope(&scope_rc, &path_element, mutations, visited_scopes);
                let mut scopes_storages = self.scopes_storages.borrow_mut();
                let scope_storage = scopes_storages.get_mut(&assigned_scope_id).unwrap();
                scope_storage.current_value = 0;
                scope_storage.current_run += 1;

                scope_rc.borrow_mut().element = Some(path_element);
            }
            PathElement::Element { elements, .. } => {
                for element in elements.iter() {
                    self.run_scope(scope, element, mutations, visited_scopes);
                }
            }
        }
    }

    /// Recursively traverse up in the scopes tree until a suitable (non-root) slot is found to put an element.
    /// Returns a parent node id and a slot index pointing to one of its children.
    fn find_scope_root_parent_info(
        &self,
        parent_id: Option<ScopeId>,
        parent_node_id: NodeId,
        scope_id: ScopeId,
    ) -> (NodeId, u32) {
        let mut index_inside_parent = 0;

        if let Some(parent_id) = parent_id {
            let mut buffer = Some((parent_id, parent_node_id, scope_id));
            while let Some((parent_id, parent_node_id, scope_id)) = buffer.take() {
                let parent_scope = self.scopes.get(&parent_id).unwrap();
                let parent_scope = parent_scope.borrow();

                let scope = self.scopes.get(&scope_id).unwrap();
                let scope = scope.borrow();

                let path_node_parent = parent_scope.nodes.find(|v| {
                    if let Some(v) = v {
                        v.node_id == parent_node_id
                    } else {
                        false
                    }
                });

                if let Some(path_node_parent) = path_node_parent {
                    if let Some(scope_id) = path_node_parent.scope_id {
                        if let Some(parent_id) = parent_scope.parent_id {
                            // The found element turns out to be a component so go to it to continue looking
                            buffer.replace((
                                parent_id,
                                parent_scope.parent_node_id_in_parent,
                                scope_id,
                            ));
                        } else {
                            assert_eq!(scope_id, ScopeId::ROOT);
                        }
                    } else {
                        // Found an Element parent so we get the index from the path
                        index_inside_parent = *scope.path_in_parent.last().unwrap();
                        return (parent_node_id, index_inside_parent);
                    }
                } else if let Some(new_parent_id) = parent_scope.parent_id {
                    // If no element was found we go to the parent scope
                    buffer.replace((
                        new_parent_id,
                        parent_scope.parent_node_id_in_parent,
                        parent_id,
                    ));
                }
            }
        } else {
            assert_eq!(scope_id, ScopeId::ROOT);
        }

        (parent_node_id, index_inside_parent)
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn apply_diff(
        &mut self,
        scope: &Rc<RefCell<Scope>>,
        diff: Diff,
        mutations: &mut Mutations,
        path_element: &PathElement,
    ) {
        let mut moved_nodes =
            FxHashMap::<Box<[u32]>, (NodeId, FxHashMap<u32, PathNode>)>::default();
        let mut parents_to_resync_scopes = FxHashSet::default();

        // Store the moved nodes so that they can
        // later be rarranged once the removals and additions have been done
        for (parent, movements) in &diff.moved {
            parents_to_resync_scopes.insert(parent.clone());
            let paths = moved_nodes.entry(parent.clone()).or_insert_with(|| {
                let parent_node_id = scope.borrow().nodes.get(parent).unwrap().node_id;
                (parent_node_id, FxHashMap::default())
            });

            for (from, _to) in movements.iter() {
                let mut path = parent.to_vec();
                path.push(*from);

                let path_node = scope.borrow().nodes.get(&path).cloned().unwrap();

                paths.1.insert(*from, path_node);
            }
        }

        // Collect a set of branches to remove in cascade
        let mut selected_roots: HashMap<&[u32], HashSet<&[u32]>> = HashMap::default();
        let mut scope_removal_buffer = vec![];

        // Given some removals like:
        // [
        //     [0,2],
        //     [0,1,0,1],
        //     [0,1,0,2],
        //     [0,3],
        //     [0,1,5,8],
        // ]
        //
        // We want them ordered like:
        // [
        //     [0,3],
        //     [0,2],
        //     [0,1,5,8],
        //     [0,1,0,2],
        //     [0,1,0,1],
        // ]
        //
        // This way any removal does not move the next removals
        'remove: for removed in diff.removed.iter().sorted_by(|a, b| {
            for (x, y) in a.iter().zip(b.iter()) {
                match x.cmp(y) {
                    Ordering::Equal => continue,
                    non_eq => return non_eq.reverse(),
                }
            }
            b.len().cmp(&a.len())
        }) {
            parents_to_resync_scopes.insert(Box::from(&removed[..removed.len() - 1]));

            let path_node = scope.borrow().nodes.get(removed).cloned();
            if let Some(PathNode { node_id, scope_id }) = path_node {
                if scope_id.is_some() {
                    selected_roots
                        .entry(&removed[..removed.len() - 1])
                        .or_default()
                        .insert(removed);
                } else {
                    let index_inside_parent = if removed.as_ref() == [0] {
                        let parent_id = scope.borrow().parent_id;
                        let scope_id = scope.borrow().id;
                        let parent_node_id = scope.borrow().parent_node_id_in_parent;
                        self.find_scope_root_parent_info(parent_id, parent_node_id, scope_id)
                            .1
                    } else {
                        // Only do it for non-scope-roots because the root is is always in the same position therefore it doesnt make sense to resync from its parent
                        removed[removed.len() - 1]
                    };

                    // plain element removal
                    mutations.removed.push(MutationRemove::Element {
                        id: node_id,
                        index: index_inside_parent,
                    });

                    // Skip if this removed path is already covered by a previously selected root
                    for (root, inner) in &mut selected_roots {
                        if is_descendant(removed, root) {
                            inner.insert(removed);
                            continue 'remove;
                        }
                    }

                    // Remove any previously selected roots that are descendants of this new (higher) removed path
                    selected_roots.retain(|root, _| !is_descendant(root, removed));

                    selected_roots
                        .entry(&removed[..removed.len() - 1])
                        .or_default()
                        .insert(removed);
                }
            } else {
                unreachable!()
            }
        }

        // Traverse each chosen branch root and queue nested scopes
        for (root, removed) in selected_roots {
            scope.borrow_mut().nodes.retain(
                root,
                |p, _| !removed.contains(p),
                |_, &PathNode { scope_id, node_id }| {
                    if let Some(scope_id) = scope_id {
                        // Queue scope to be removed
                        scope_removal_buffer.push(self.scopes.get(&scope_id).cloned().unwrap());
                    } else {
                        self.node_to_scope.remove(&node_id).unwrap();
                    }
                },
            );
        }

        let mut scope_removal_queue = VecDeque::new();

        while let Some(scope_rc) = scope_removal_buffer.pop() {
            // Push the scopes to a queue that will remove
            // them starting from the deepest to the highest ones
            scope_removal_queue.push_front(scope_rc.clone());

            let scope = scope_rc.borrow_mut();

            let mut scope_root_node_id = None;

            // Queue nested scopes to be removed
            scope
                .nodes
                .traverse(&[], |path, &PathNode { scope_id, node_id }| {
                    if let Some(scope_id) = scope_id {
                        scope_removal_buffer.push(self.scopes.get(&scope_id).cloned().unwrap());
                    } else {
                        self.node_to_scope.remove(&node_id).unwrap();
                    }
                    if path == [0] {
                        scope_root_node_id = Some(node_id);
                    }
                });

            // Nodes that have a scope id are components, so no need to mark those as removed in the tree
            // Instead we get their root node id and remove it
            mutations.removed.push(MutationRemove::Scope {
                id: scope_root_node_id.unwrap(),
            });
        }

        // Finally drops the scopes and their storage
        for scope_rc in scope_removal_queue {
            let scope = scope_rc.borrow_mut();

            self.scopes.remove(&scope.id);

            // Dropped hooks might e.g spawn forever tasks, so they need access to the context
            CurrentContext::run_with_reactive(
                CurrentContext {
                    scope_id: scope.id,
                    scopes_storages: self.scopes_storages.clone(),
                    tasks: self.tasks.clone(),
                    task_id_counter: self.task_id_counter.clone(),
                    sender: self.sender.clone(),
                },
                || {
                    // This is very important, the scope storage must be dropped after the borrow in `scopes_storages` has been released
                    let _scope = self.scopes_storages.borrow_mut().remove(&scope.id);
                },
            );

            // TODO: Scopes could also maintain its own registry of assigned tasks
            self.tasks
                .borrow_mut()
                .retain(|_task_id, task| task.borrow().scope_id != scope.id);
        }

        // Given some additions like:
        // [
        //     [0,2],
        //     [0,1,0,1],
        //     [0,1,0,2],
        //     [0,3],
        //     [0,1,5,8],
        // ]
        //
        // We want them ordered like:
        // [
        //     [0,1,0,1],
        //     [0,1,0,2],
        //     [0,1,5,8],
        //     [0,2],
        //     [0,3],
        // ]
        //
        // This way, no addition offsets the next additions in line.
        for added in diff
            .added
            .iter()
            .sorted_by(|a, b| {
                for (x, y) in a.iter().zip(b.iter()) {
                    match x.cmp(y) {
                        Ordering::Equal => continue,
                        non_eq => return non_eq.reverse(),
                    }
                }
                b.len().cmp(&a.len())
            })
            .rev()
        {
            let (parent_node_id, index_inside_parent) = if added.as_ref() == [0] {
                let parent_id = scope.borrow().parent_id;
                let scope_id = scope.borrow().id;
                let parent_node_id = scope.borrow().parent_node_id_in_parent;
                self.find_scope_root_parent_info(parent_id, parent_node_id, scope_id)
            } else {
                // Only do it for non-scope-roots because the root is is always in the same position therefore it doesnt make sense to resync from its parent
                parents_to_resync_scopes.insert(Box::from(&added[..added.len() - 1]));
                (
                    scope
                        .borrow()
                        .nodes
                        .get(&added[..added.len() - 1])
                        .unwrap()
                        .node_id,
                    added[added.len() - 1],
                )
            };

            self.node_id_counter += 1;

            path_element.with_element(added, |element| match element {
                PathElement::Component { .. } => {
                    self.scope_id_counter += 1;
                    let scope_id = self.scope_id_counter;

                    scope.borrow_mut().nodes.insert(
                        added,
                        PathNode {
                            node_id: self.node_id_counter,
                            scope_id: Some(scope_id),
                        },
                    );
                }
                PathElement::Element { element, .. } => {
                    mutations.added.push((
                        self.node_id_counter,
                        parent_node_id,
                        index_inside_parent,
                        element.clone(),
                    ));

                    self.node_to_scope
                        .insert(self.node_id_counter, scope.borrow().id);
                    scope.borrow_mut().nodes.insert(
                        added,
                        PathNode {
                            node_id: self.node_id_counter,
                            scope_id: None,
                        },
                    );
                }
            });
        }

        for (parent, movements) in diff.moved.into_iter().sorted_by(|(a, _), (b, _)| {
            for (x, y) in a.iter().zip(b.iter()) {
                match x.cmp(y) {
                    Ordering::Equal => continue,
                    non_eq => return non_eq.reverse(),
                }
            }
            b.len().cmp(&a.len())
        }) {
            parents_to_resync_scopes.insert(parent.clone());

            let (parent_node_id, paths) = moved_nodes.get_mut(&parent).unwrap();

            for (from, to) in movements.into_iter().sorted_by_key(|e| e.0).rev() {
                let path_node = paths.remove(&from).unwrap();

                let PathNode { node_id, scope_id } = path_node;

                // Search for this moved node current position
                let from_path = scope
                    .borrow()
                    .nodes
                    .find_child_path(&parent, |v| v == Some(&path_node))
                    .unwrap();

                let mut to_path = parent.to_vec();
                to_path.push(to);

                if from_path == to_path {
                    continue;
                }

                // Remove the node from the old position and add it to the new one
                let path_entry = scope.borrow_mut().nodes.remove(&from_path).unwrap();
                scope.borrow_mut().nodes.insert_entry(&to_path, path_entry);

                if let Some(scope_id) = scope_id {
                    let scope_rc = self.scopes.get(&scope_id).cloned().unwrap();

                    let scope = scope_rc.borrow();

                    let scope_root_node_id = scope.nodes.get(&[0]).map(|node| node.node_id);

                    // Mark the scope root node id as moved
                    mutations
                        .moved
                        .entry(scope.parent_node_id_in_parent)
                        .or_default()
                        .push((to, scope_root_node_id.unwrap()));
                } else {
                    // Mark the element as moved
                    mutations
                        .moved
                        .entry(*parent_node_id)
                        .or_default()
                        .push((to, node_id));
                }
            }
        }

        for (modified, flags) in diff.modified {
            path_element.with_element(&modified, |element| match element {
                PathElement::Component { .. } => {
                    // Components never change when being diffed
                    unreachable!()
                }
                PathElement::Element { element, .. } => {
                    let node_id = scope
                        .borrow()
                        .nodes
                        .get(&modified)
                        .map(|path_node| path_node.node_id)
                        .unwrap();
                    mutations.modified.push((node_id, element.clone(), flags));
                }
            });
        }

        // When a parent gets a new child, or a child is removed or moved we
        // resync its 1 level children scopes with their new path
        for parent in parents_to_resync_scopes {
            // But only if the parent already existed before otherwise its pointless
            // as Scopes will be created with the latest path already
            if diff.added.contains(&parent) {
                // TODO: Maybe do this check before inserting
                continue;
            }

            // Update all the nested scopes in this Scope with their up to date paths
            scope
                .borrow_mut()
                .nodes
                .traverse_1_level(&parent, |p, path_node| {
                    if let Some(scope_id) = path_node.scope_id
                        && let Some(scope_rc) = self.scopes.get(&scope_id)
                    {
                        let mut scope = scope_rc.borrow_mut();
                        scope.path_in_parent = Box::from(p);
                    }
                });
        }
    }
}

#[derive(Default, Debug)]
pub struct Diff {
    pub added: Vec<Box<[u32]>>,

    pub modified: Vec<(Box<[u32]>, DiffModifies)>,

    pub removed: Vec<Box<[u32]>>,

    pub moved: HashMap<Box<[u32]>, Vec<(u32, u32)>>,
}

fn is_descendant(candidate: &[u32], ancestor: &[u32]) -> bool {
    if ancestor.len() > candidate.len() {
        return false;
    }
    candidate[..ancestor.len()] == *ancestor
}
