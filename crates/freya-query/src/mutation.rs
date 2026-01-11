use core::fmt;
use std::{
    cell::{
        Ref,
        RefCell,
    },
    collections::HashMap,
    future::Future,
    hash::Hash,
    mem,
    rc::Rc,
    time::{
        Duration,
        Instant,
    },
};

use async_io::Timer;
use freya_core::{
    integration::FxHashSet,
    lifecycle::context::{
        consume_context,
        provide_context_for_scope_id,
        try_consume_context,
    },
    prelude::*,
    scope_id::ScopeId,
};

pub trait MutationCapability
where
    Self: 'static + Clone + PartialEq + Hash + Eq,
{
    type Ok;
    type Err;
    type Keys: Hash + PartialEq + Clone;

    /// Mutation logic.
    fn run(&self, keys: &Self::Keys) -> impl Future<Output = Result<Self::Ok, Self::Err>>;

    /// Implement a custom logic to check if this mutation should be invalidated or not given a [MutationCapability::Keys].
    fn matches(&self, _keys: &Self::Keys) -> bool {
        true
    }

    /// Runs after [MutationCapability::run].
    /// You may use this method to invalidate [crate::query::Query]s.
    fn on_settled(
        &self,
        _keys: &Self::Keys,
        _result: &Result<Self::Ok, Self::Err>,
    ) -> impl Future<Output = ()> {
        async {}
    }
}

pub enum MutationStateData<Q: MutationCapability> {
    /// Has not loaded yet.
    Pending,
    /// Is loading and may not have a previous settled value.
    Loading { res: Option<Result<Q::Ok, Q::Err>> },
    /// Is not loading and has a settled value.
    Settled {
        res: Result<Q::Ok, Q::Err>,
        settlement_instant: Instant,
    },
}

impl<Q> fmt::Debug for MutationStateData<Q>
where
    Q: MutationCapability,
    Q::Ok: fmt::Debug,
    Q::Err: fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => f.write_str("Pending"),
            Self::Loading { res } => write!(f, "Loading {{ {res:?} }}"),
            Self::Settled { res, .. } => write!(f, "Settled {{ {res:?} }}"),
        }
    }
}

impl<Q: MutationCapability> MutationStateData<Q> {
    /// Check if the state is [MutationStateData::Settled] and [Result::Ok].
    pub fn is_ok(&self) -> bool {
        matches!(self, MutationStateData::Settled { res: Ok(_), .. })
    }

    /// Check if the state is [MutationStateData::Settled] and [Result::Err].
    pub fn is_err(&self) -> bool {
        matches!(self, MutationStateData::Settled { res: Err(_), .. })
    }

    /// Check if the state is [MutationStateData::Loading].
    pub fn is_loading(&self) -> bool {
        matches!(self, MutationStateData::Loading { .. })
    }

    /// Check if the state is [MutationStateData::Pending].
    pub fn is_pending(&self) -> bool {
        matches!(self, MutationStateData::Pending)
    }

    /// Get the value as an [Option].
    pub fn ok(&self) -> Option<&Q::Ok> {
        match self {
            Self::Settled { res: Ok(res), .. } => Some(res),
            Self::Loading { res: Some(Ok(res)) } => Some(res),
            _ => None,
        }
    }

    /// Get the value as an [Result] if possible, otherwise it will panic.
    pub fn unwrap(&self) -> &Result<Q::Ok, Q::Err> {
        match self {
            Self::Loading { res: Some(v) } => v,
            Self::Settled { res, .. } => res,
            _ => unreachable!(),
        }
    }

    fn into_loading(self) -> MutationStateData<Q> {
        match self {
            MutationStateData::Pending => MutationStateData::Loading { res: None },
            MutationStateData::Loading { res } => MutationStateData::Loading { res },
            MutationStateData::Settled { res, .. } => MutationStateData::Loading { res: Some(res) },
        }
    }
}
pub struct MutationsStorage<Q: MutationCapability> {
    storage: State<HashMap<Mutation<Q>, MutationData<Q>>>,
}

impl<Q: MutationCapability> Copy for MutationsStorage<Q> {}

impl<Q: MutationCapability> Clone for MutationsStorage<Q> {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct MutationData<Q: MutationCapability> {
    state: Rc<RefCell<MutationStateData<Q>>>,
    reactive_contexts: Rc<RefCell<FxHashSet<ReactiveContext>>>,

    clean_task: Rc<RefCell<Option<TaskHandle>>>,
}

impl<Q: MutationCapability> Clone for MutationData<Q> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            reactive_contexts: self.reactive_contexts.clone(),
            clean_task: self.clean_task.clone(),
        }
    }
}

impl<Q: MutationCapability> MutationsStorage<Q> {
    fn new_in_root() -> Self {
        Self {
            storage: State::create_global(HashMap::default()),
        }
    }

    fn insert_or_get_mutation(&mut self, mutation: Mutation<Q>) -> MutationData<Q> {
        let mut storage = self.storage.write_unchecked();

        let mutation_data = storage.entry(mutation).or_insert_with(|| MutationData {
            state: Rc::new(RefCell::new(MutationStateData::Pending)),
            reactive_contexts: Rc::new(RefCell::new(FxHashSet::default())),
            clean_task: Rc::default(),
        });

        // Cancel clean task
        if let Some(clean_task) = mutation_data.clean_task.take() {
            clean_task.cancel();
        }

        mutation_data.clone()
    }

    fn update_tasks(&mut self, mutation: Mutation<Q>) {
        let storage_clone = self.storage;
        let mut storage = self.storage.write_unchecked();

        let mutation_data = storage.get_mut(&mutation).unwrap();

        // Spawn clean up task if there no more reactive contexts
        if mutation_data.reactive_contexts.borrow().is_empty() {
            *mutation_data.clean_task.borrow_mut() = Some(spawn_forever(async move {
                // Wait as long as the stale time is configured
                Timer::after(mutation.clean_time).await;

                // Finally clear the mutation
                let mut storage = storage_clone.write_unchecked();
                storage.remove(&mutation);
            }));
        }
    }

    async fn run(mutation: &Mutation<Q>, data: &MutationData<Q>, keys: Q::Keys) {
        // Set to Loading
        let res =
            mem::replace(&mut *data.state.borrow_mut(), MutationStateData::Pending).into_loading();
        *data.state.borrow_mut() = res;
        for reactive_context in data.reactive_contexts.borrow().iter() {
            reactive_context.notify();
        }

        // Run
        let res = mutation.mutation.run(&keys).await;

        // Set to Settled
        mutation.mutation.on_settled(&keys, &res).await;
        *data.state.borrow_mut() = MutationStateData::Settled {
            res,
            settlement_instant: Instant::now(),
        };
        for reactive_context in data.reactive_contexts.borrow().iter() {
            reactive_context.notify();
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Mutation<Q: MutationCapability> {
    mutation: Q,

    clean_time: Duration,
}

impl<Q: MutationCapability> Eq for Mutation<Q> {}
impl<Q: MutationCapability> Hash for Mutation<Q> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.mutation.hash(state);
    }
}

impl<Q: MutationCapability> Mutation<Q> {
    pub fn new(mutation: Q) -> Self {
        Self {
            mutation,
            clean_time: Duration::ZERO,
        }
    }

    /// For how long the data is kept cached after there are no more mutation subscribers.
    ///
    /// Defaults to [Duration::ZERO], meaning it clears automatically.
    pub fn clean_time(self, clean_time: Duration) -> Self {
        Self { clean_time, ..self }
    }
}

pub struct MutationReader<Q: MutationCapability> {
    state: Rc<RefCell<MutationStateData<Q>>>,
}

impl<Q: MutationCapability> MutationReader<Q> {
    pub fn state(&'_ self) -> Ref<'_, MutationStateData<Q>> {
        self.state.borrow()
    }
}

pub struct UseMutation<Q: MutationCapability> {
    mutation: State<Mutation<Q>>,
}

impl<Q: MutationCapability> Clone for UseMutation<Q> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Q: MutationCapability> Copy for UseMutation<Q> {}

impl<Q: MutationCapability> UseMutation<Q> {
    /// Read the [Mutation].
    ///
    /// This **will** automatically subscribe.
    /// If you want a **subscribing** method have a look at [UseMutation::peek].
    pub fn read(&self) -> MutationReader<Q> {
        let storage = consume_context::<MutationsStorage<Q>>();
        let map = storage.storage.peek();
        let mutation_data = map.get(&self.mutation.peek()).cloned().unwrap();

        // Subscribe if possible
        if let Some(mut reactive_context) = ReactiveContext::try_current() {
            reactive_context.subscribe(&mutation_data.reactive_contexts);
        }

        MutationReader {
            state: mutation_data.state,
        }
    }

    /// Read the [Mutation].
    ///
    /// This **will not** automatically subscribe.
    /// If you want a **subscribing** method have a look at [UseMutation::read].
    pub fn peek(&self) -> MutationReader<Q> {
        let storage = consume_context::<MutationsStorage<Q>>();
        let map = storage.storage.peek();
        let mutation_data = map.get(&self.mutation.peek()).cloned().unwrap();

        MutationReader {
            state: mutation_data.state,
        }
    }

    /// Run this mutation await its result.
    ///
    /// For a `sync` version use [UseMutation::mutate].
    pub async fn mutate_async(&self, keys: Q::Keys) -> MutationReader<Q> {
        let storage = consume_context::<MutationsStorage<Q>>();

        let mutation = self.mutation.peek().clone();
        let map = storage.storage.peek();
        let mutation_data = map.get(&mutation).cloned().unwrap();

        // Run the mutation
        MutationsStorage::run(&mutation, &mutation_data, keys).await;

        MutationReader {
            state: mutation_data.state,
        }
    }

    // Run this mutation and await its result.
    ///
    /// For an `async` version use [UseMutation::mutate_async].
    pub fn mutate(&self, keys: Q::Keys) {
        let storage = consume_context::<MutationsStorage<Q>>();

        let mutation = self.mutation.peek().clone();
        let map = storage.storage.peek();
        let mutation_data = map.get(&mutation).cloned().unwrap();

        // Run the mutation
        spawn(async move { MutationsStorage::run(&mutation, &mutation_data, keys).await });
    }
}

/// Mutations are used to update data asynchronously of an e.g external resources such as HTTP APIs.
///
/// ### Clean time
/// This is how long will the mutation result be kept cached after there are no more subscribers of that mutation.
///
/// See [Mutation::clean_time].
pub fn use_mutation<Q: MutationCapability>(mutation: Mutation<Q>) -> UseMutation<Q> {
    let mut storage = match try_consume_context::<MutationsStorage<Q>>() {
        Some(storage) => storage,
        None => {
            provide_context_for_scope_id(MutationsStorage::<Q>::new_in_root(), Some(ScopeId::ROOT));
            try_consume_context::<MutationsStorage<Q>>().unwrap()
        }
    };

    let mut make_mutation = |mutation: &Mutation<Q>, mut prev_mutation: Option<Mutation<Q>>| {
        let _data = storage.insert_or_get_mutation(mutation.clone());

        // Update the mutation tasks if there has been a change in the mutation
        if let Some(prev_mutation) = prev_mutation.take() {
            storage.update_tasks(prev_mutation);
        }
    };

    let mut current_mutation = use_hook(|| {
        make_mutation(&mutation, None);
        State::create(mutation.clone())
    });

    if *current_mutation.read() != mutation {
        let prev = mem::replace(&mut *current_mutation.write(), mutation.clone());
        make_mutation(&mutation, Some(prev));
    }

    // Update the mutation tasks when the scope is dropped
    use_drop({
        move || {
            storage.update_tasks(current_mutation.peek().clone());
        }
    });

    UseMutation {
        mutation: current_mutation,
    }
}
