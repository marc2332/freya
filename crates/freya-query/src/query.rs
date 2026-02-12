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
use futures_util::stream::{
    FuturesUnordered,
    StreamExt,
};

pub trait QueryCapability
where
    Self: 'static + Clone + PartialEq + Hash + Eq,
{
    type Ok;
    type Err;
    type Keys: Hash + PartialEq + Clone;

    /// Query logic.
    fn run(&self, keys: &Self::Keys) -> impl Future<Output = Result<Self::Ok, Self::Err>>;

    /// Implement a custom logic to check if this query should be invalidated or not given a [QueryCapability::Keys].
    fn matches(&self, _keys: &Self::Keys) -> bool {
        true
    }
}

pub enum QueryStateData<Q: QueryCapability> {
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

impl<Q: QueryCapability> TryFrom<QueryStateData<Q>> for Result<Q::Ok, Q::Err> {
    type Error = ();

    fn try_from(value: QueryStateData<Q>) -> Result<Self, Self::Error> {
        match value {
            QueryStateData::Loading { res: Some(res) } => Ok(res),
            QueryStateData::Settled { res, .. } => Ok(res),
            _ => Err(()),
        }
    }
}

impl<Q> fmt::Debug for QueryStateData<Q>
where
    Q: QueryCapability,
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

impl<Q: QueryCapability> QueryStateData<Q> {
    /// Check if the state is [QueryStateData::Settled] and [Result::Ok].
    pub fn is_ok(&self) -> bool {
        matches!(self, QueryStateData::Settled { res: Ok(_), .. })
    }

    /// Check if the state is [QueryStateData::Settled] and [Result::Err].
    pub fn is_err(&self) -> bool {
        matches!(self, QueryStateData::Settled { res: Err(_), .. })
    }

    /// Check if the state is [QueryStateData::Loading].
    pub fn is_loading(&self) -> bool {
        matches!(self, QueryStateData::Loading { .. })
    }

    /// Check if the state is [QueryStateData::Pending].
    pub fn is_pending(&self) -> bool {
        matches!(self, QueryStateData::Pending)
    }

    /// Check if the state is stale or not, where stale means outdated.
    pub fn is_stale(&self, query: &Query<Q>) -> bool {
        match self {
            QueryStateData::Pending => true,
            QueryStateData::Loading { .. } => true,
            QueryStateData::Settled {
                settlement_instant, ..
            } => Instant::now().duration_since(*settlement_instant) >= query.stale_time,
        }
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

    fn into_loading(self) -> QueryStateData<Q> {
        match self {
            QueryStateData::Pending => QueryStateData::Loading { res: None },
            QueryStateData::Loading { res } => QueryStateData::Loading { res },
            QueryStateData::Settled { res, .. } => QueryStateData::Loading { res: Some(res) },
        }
    }
}

pub struct QueriesStorage<Q: QueryCapability> {
    storage: State<HashMap<Query<Q>, QueryData<Q>>>,
}

impl<Q: QueryCapability> Copy for QueriesStorage<Q> {}

impl<Q: QueryCapability> Clone for QueriesStorage<Q> {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct QueryData<Q: QueryCapability> {
    state: Rc<RefCell<QueryStateData<Q>>>,
    reactive_contexts: Rc<RefCell<FxHashSet<ReactiveContext>>>,

    interval_task: Rc<RefCell<Option<(Duration, TaskHandle)>>>,
    clean_task: Rc<RefCell<Option<TaskHandle>>>,
}

impl<Q: QueryCapability> Clone for QueryData<Q> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            reactive_contexts: self.reactive_contexts.clone(),

            interval_task: self.interval_task.clone(),
            clean_task: self.clean_task.clone(),
        }
    }
}

impl<Q: QueryCapability> QueriesStorage<Q> {
    fn new_in_root() -> Self {
        Self {
            storage: State::create_global(HashMap::default()),
        }
    }

    fn insert_or_get_query(&mut self, query: Query<Q>) -> QueryData<Q> {
        let query_clone = query.clone();
        let mut storage = self.storage.write_unchecked();

        let query_data = storage.entry(query).or_insert_with(|| QueryData {
            state: Rc::new(RefCell::new(QueryStateData::Pending)),
            reactive_contexts: Rc::new(RefCell::new(FxHashSet::default())),
            interval_task: Rc::default(),
            clean_task: Rc::default(),
        });
        let query_data_clone = query_data.clone();

        // Cancel clean task
        if let Some(clean_task) = query_data.clean_task.take() {
            clean_task.cancel();
        }

        // Start an interval task if necessary
        // If multiple queries subscribers use different intervals the interval task
        // will run using the shortest interval
        let interval = query_clone.interval_time;
        let interval_enabled = query_clone.interval_time != Duration::MAX;
        let interval_task = &mut *query_data.interval_task.borrow_mut();

        let create_interval_task = match interval_task {
            None if interval_enabled => true,
            Some((current_interval, current_interval_task)) if interval_enabled => {
                let new_interval_is_shorter = *current_interval > interval;
                if new_interval_is_shorter {
                    current_interval_task.cancel();
                    *interval_task = None;
                }
                new_interval_is_shorter
            }
            _ => false,
        };
        if create_interval_task {
            let task = spawn_forever(async move {
                loop {
                    // Wait as long as the stale time is configured
                    Timer::after(interval).await;

                    // Run the query
                    QueriesStorage::<Q>::run_queries(&[(&query_clone, &query_data_clone)]).await;
                }
            });
            *interval_task = Some((interval, task));
        }

        query_data.clone()
    }

    fn update_tasks(&mut self, query: Query<Q>) {
        let storage_clone = self.storage;
        let mut storage = self.storage.write_unchecked();

        let query_data = storage.get_mut(&query).unwrap();

        // Cancel interval task
        if let Some((_, interval_task)) = query_data.interval_task.take() {
            interval_task.cancel();
        }

        // Spawn clean up task if there no more reactive contexts
        if query_data.reactive_contexts.borrow().len() == 1 {
            *query_data.clean_task.borrow_mut() = Some(spawn_forever(async move {
                // Wait as long as the stale time is configured
                Timer::after(query.clean_time).await;

                // Finally clear the query
                let mut storage = storage_clone.write_unchecked();
                storage.remove(&query);
            }));
        }
    }

    pub async fn get(get_query: GetQuery<Q>) -> QueryReader<Q> {
        let query: Query<Q> = get_query.into();

        let mut storage = match try_consume_context::<QueriesStorage<Q>>() {
            Some(storage) => storage,
            None => {
                provide_context_for_scope_id(
                    QueriesStorage::<Q>::new_in_root(),
                    Some(ScopeId::ROOT),
                );
                try_consume_context::<QueriesStorage<Q>>().unwrap()
            }
        };

        let mut map = storage.storage.write();
        let query_data = map
            .entry(query.clone())
            .or_insert_with(|| QueryData {
                state: Rc::new(RefCell::new(QueryStateData::Pending)),
                reactive_contexts: Rc::new(RefCell::new(FxHashSet::default())),
                interval_task: Rc::default(),
                clean_task: Rc::default(),
            })
            .clone();

        // Run the query if the value is stale
        if query_data.state.borrow().is_stale(&query) {
            // Set to Loading
            let res = mem::replace(&mut *query_data.state.borrow_mut(), QueryStateData::Pending)
                .into_loading();
            *query_data.state.borrow_mut() = res;
            for reactive_context in query_data.reactive_contexts.borrow().iter() {
                reactive_context.notify();
            }

            // Run
            let res = query.query.run(&query.keys).await;

            // Set to Settled
            *query_data.state.borrow_mut() = QueryStateData::Settled {
                res,
                settlement_instant: Instant::now(),
            };
            for reactive_context in query_data.reactive_contexts.borrow().iter() {
                reactive_context.notify();
            }
        }

        // Spawn clean up task if there no more reactive contexts
        if query_data.reactive_contexts.borrow().is_empty() {
            *query_data.clean_task.borrow_mut() = Some(spawn_forever(async move {
                // Wait as long as the stale time is configured
                Timer::after(query.clean_time).await;

                // Finally clear the query
                let mut storage = storage.storage.write_unchecked();
                storage.remove(&query);
            }));
        }

        QueryReader {
            state: query_data.state,
        }
    }

    pub async fn invalidate_all() {
        let storage = consume_context::<QueriesStorage<Q>>();

        // Get all the queries
        let matching_queries = storage
            .storage
            .read()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        let matching_queries = matching_queries
            .iter()
            .map(|(q, d)| (q, d))
            .collect::<Vec<_>>();

        // Invalidate the queries
        Self::run_queries(&matching_queries).await
    }

    pub async fn invalidate_matching(matching_keys: Q::Keys) {
        let storage = consume_context::<QueriesStorage<Q>>();

        // Get those queries that match
        let mut matching_queries = Vec::new();
        for (query, data) in storage.storage.read().iter() {
            if query.query.matches(&matching_keys) {
                matching_queries.push((query.clone(), data.clone()));
            }
        }
        let matching_queries = matching_queries
            .iter()
            .map(|(q, d)| (q, d))
            .collect::<Vec<_>>();

        // Invalidate the queries
        Self::run_queries(&matching_queries).await
    }

    async fn run_queries(queries: &[(&Query<Q>, &QueryData<Q>)]) {
        let tasks = FuturesUnordered::new();

        for (query, query_data) in queries {
            // Set to Loading
            let res = mem::replace(&mut *query_data.state.borrow_mut(), QueryStateData::Pending)
                .into_loading();
            *query_data.state.borrow_mut() = res;
            for reactive_context in query_data.reactive_contexts.borrow().iter() {
                reactive_context.notify();
            }

            tasks.push(Box::pin(async move {
                // Run
                let res = query.query.run(&query.keys).await;

                // Set to settled
                *query_data.state.borrow_mut() = QueryStateData::Settled {
                    res,
                    settlement_instant: Instant::now(),
                };
                for reactive_context in query_data.reactive_contexts.borrow().iter() {
                    reactive_context.notify();
                }
            }));
        }

        tasks.count().await;
    }
}

pub struct GetQuery<Q: QueryCapability> {
    query: Q,
    keys: Q::Keys,

    stale_time: Duration,
    clean_time: Duration,
}

impl<Q: QueryCapability> GetQuery<Q> {
    pub fn new(keys: Q::Keys, query: Q) -> Self {
        Self {
            query,
            keys,
            stale_time: Duration::ZERO,
            clean_time: Duration::ZERO,
        }
    }
    /// For how long is the data considered stale. If a query subscriber is mounted and the data is stale, it will re run the query.
    ///
    /// Defaults to [Duration::ZERO], meaning it is marked stale immediately.
    pub fn stale_time(self, stale_time: Duration) -> Self {
        Self { stale_time, ..self }
    }

    /// For how long the data is kept cached after there are no more query subscribers.
    ///
    /// Defaults to [Duration::ZERO], meaning it clears automatically.
    pub fn clean_time(self, clean_time: Duration) -> Self {
        Self { clean_time, ..self }
    }
}

impl<Q: QueryCapability> From<GetQuery<Q>> for Query<Q> {
    fn from(value: GetQuery<Q>) -> Self {
        Query {
            query: value.query,
            keys: value.keys,

            enabled: true,

            stale_time: value.stale_time,
            clean_time: value.clean_time,
            interval_time: Duration::MAX,
        }
    }
}
#[derive(PartialEq, Clone)]
pub struct Query<Q: QueryCapability> {
    query: Q,
    keys: Q::Keys,

    enabled: bool,

    stale_time: Duration,
    clean_time: Duration,
    interval_time: Duration,
}

impl<Q: QueryCapability> Eq for Query<Q> {}
impl<Q: QueryCapability> Hash for Query<Q> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.query.hash(state);
        self.keys.hash(state);

        self.enabled.hash(state);

        self.stale_time.hash(state);
        self.clean_time.hash(state);

        // Intentionally left out as intervals can vary from one query subscriber to another
        // self.interval_time.hash(state);
    }
}

impl<Q: QueryCapability> Query<Q> {
    pub fn new(keys: Q::Keys, query: Q) -> Self {
        Self {
            query,
            keys,
            enabled: true,
            stale_time: Duration::ZERO,
            clean_time: Duration::from_secs(5 * 60),
            interval_time: Duration::MAX,
        }
    }

    /// Enable or disable this query so that it doesnt automatically run.
    ///
    /// Defaults to `true`.
    pub fn enable(self, enabled: bool) -> Self {
        Self { enabled, ..self }
    }

    /// For how long is the data considered stale. If a query subscriber is mounted and the data is stale, it will re run the query
    /// otherwise it return the cached data.
    ///
    /// Defaults to [Duration::ZERO], meaning it is marked stale immediately after it has been used.
    pub fn stale_time(self, stale_time: Duration) -> Self {
        Self { stale_time, ..self }
    }

    /// For how long the data is kept cached after there are no more query subscribers.
    ///
    /// Defaults to `5min`, meaning it clears automatically after 5 minutes of no subscribers to it.
    pub fn clean_time(self, clean_time: Duration) -> Self {
        Self { clean_time, ..self }
    }

    /// Every how often the query reruns.
    ///
    /// Defaults to [Duration::MAX], meaning it never re runs automatically.
    ///
    /// **Note**: If multiple subscribers of the same query use different intervals, only the shortest one will be used.
    pub fn interval_time(self, interval_time: Duration) -> Self {
        Self {
            interval_time,
            ..self
        }
    }
}

pub struct QueryReader<Q: QueryCapability> {
    state: Rc<RefCell<QueryStateData<Q>>>,
}

impl<Q: QueryCapability> QueryReader<Q> {
    pub fn state(&'_ self) -> Ref<'_, QueryStateData<Q>> {
        self.state.borrow()
    }

    /// Get the result of the query.
    ///
    /// **This method will panic if the query is not settled.**
    pub fn as_settled(&'_ self) -> Ref<'_, Result<Q::Ok, Q::Err>> {
        Ref::map(self.state.borrow(), |state| match state {
            QueryStateData::Settled { res, .. } => res,
            _ => panic!("Query is not settled."),
        })
    }
}

pub struct UseQuery<Q: QueryCapability> {
    query: State<Query<Q>>,
}

impl<Q: QueryCapability> Clone for UseQuery<Q> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Q: QueryCapability> Copy for UseQuery<Q> {}

impl<Q: QueryCapability> UseQuery<Q> {
    /// Read the [Query] state.
    ///
    /// This **will** automatically subscribe.
    /// If you want a **non-subscribing** method have a look at [UseQuery::peek].
    pub fn read(&self) -> QueryReader<Q> {
        let storage = consume_context::<QueriesStorage<Q>>();
        let map = storage.storage.peek();
        let query_data = map.get(&self.query.peek()).cloned().unwrap();

        // Subscribe if possible
        if let Some(mut reactive_context) = ReactiveContext::try_current() {
            reactive_context.subscribe(&query_data.reactive_contexts);
        }

        QueryReader {
            state: query_data.state,
        }
    }

    /// Read the [Query] state.
    ///
    /// This **will not** automatically subscribe.
    /// If you want a **subscribing** method have a look at [UseQuery::read].
    pub fn peek(&self) -> QueryReader<Q> {
        let storage = consume_context::<QueriesStorage<Q>>();
        let map = storage.storage.peek();
        let query_data = map.get(&self.query.peek()).cloned().unwrap();

        QueryReader {
            state: query_data.state,
        }
    }

    /// Invalidate this query and await its result.
    ///
    /// For a `sync` version use [UseQuery::invalidate].
    pub async fn invalidate_async(&self) -> QueryReader<Q> {
        let storage = consume_context::<QueriesStorage<Q>>();

        let query = self.query.peek().clone();
        let map = storage.storage.peek();
        let query_data = map.get(&query).cloned().unwrap();

        // Run the query
        QueriesStorage::run_queries(&[(&query, &query_data)]).await;

        QueryReader {
            state: query_data.state.clone(),
        }
    }

    /// Invalidate this query in the background.
    ///
    /// For an `async` version use [UseQuery::invalidate_async].
    pub fn invalidate(&self) {
        let storage = consume_context::<QueriesStorage<Q>>();

        let query = self.query.peek().clone();
        let map = storage.storage.peek();
        let query_data = map.get(&query).cloned().unwrap();

        // Run the query
        spawn(async move { QueriesStorage::run_queries(&[(&query, &query_data)]).await });
    }
}

/// Queries are used to get data asynchronously (e.g external resources such as HTTP APIs), which can later be cached or refreshed.
///
/// Important concepts:
///
/// ### Stale time
/// This is how long will a value that is cached, considered to be recent enough.
/// So in other words, if a value is stale it means that its outdated and therefore it should be refreshed.
///
/// By default the stale time is `0ms`, so if a value is cached and a new query subscriber
/// is interested in this value, it will get refreshed automatically.
///
/// See [Query::stale_time].
///
/// ### Clean time
/// This is how long will a value kept cached after there are no more subscribers of that query.
///
/// Imagine there is `Subscriber 1` of a query, the data is requested and cached.
/// But after some seconds the `Subscriber 1` is unmounted, but the data is not cleared as the default clean time is `5min`.
/// A few seconds later the `Subscriber 1` gets mounted again, it requests the data again but this time it is returned directly from the cache.
///
/// See [Query::clean_time].
///
/// ### Interval time
/// This is how often do you want a query to be refreshed in the background automatically.
/// By default it never refreshes automatically.
///
/// See [Query::interval_time].
pub fn use_query<Q: QueryCapability>(query: Query<Q>) -> UseQuery<Q> {
    let mut storage = match try_consume_context::<QueriesStorage<Q>>() {
        Some(storage) => storage,
        None => {
            provide_context_for_scope_id(QueriesStorage::<Q>::new_in_root(), Some(ScopeId::ROOT));
            try_consume_context::<QueriesStorage<Q>>().unwrap()
        }
    };

    let mut make_query = |query: &Query<Q>, mut prev_query: Option<Query<Q>>| {
        let query_data = storage.insert_or_get_query(query.clone());

        // Update the query tasks if there has been a change in the query
        if let Some(prev_query) = prev_query.take() {
            storage.update_tasks(prev_query);
        }

        // Immediately run the query if enabled and the value is stale
        if query.enabled && query_data.state.borrow().is_stale(query) {
            let query = query.clone();
            spawn(async move {
                QueriesStorage::run_queries(&[(&query, &query_data)]).await;
            });
        }
    };

    let mut current_query = use_hook(|| {
        make_query(&query, None);
        State::create(query.clone())
    });

    if *current_query.read() != query {
        let prev = mem::replace(&mut *current_query.write(), query.clone());
        make_query(&query, Some(prev));
    }

    // Update the query tasks when the scope is dropped
    use_drop({
        move || {
            storage.update_tasks(current_query.peek().clone());
        }
    });

    let query = UseQuery {
        query: current_query,
    };

    // Used to consider this use_query call as a subscriber without rerunning the component
    use_side_effect(move || {
        let _ = query.read();
    });

    query
}
