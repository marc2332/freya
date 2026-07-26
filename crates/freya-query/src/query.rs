use core::fmt;
use std::{
    cell::{
        Cell,
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
    ///
    /// [QueryStateData::Pending] and [QueryStateData::Loading] are always stale as neither holds a
    /// settled value that could age. Staleness says nothing about whether an execution is already
    /// in flight, so it alone is not enough to decide to dispatch a new one.
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

    /// How many executions of this query are in flight right now.
    ///
    /// The state alone cannot tell: a brand new entry is [QueryStateData::Pending] before its
    /// first execution is even dispatched, so [QueryStateData::is_stale] is `true` both for a
    /// query that nobody is running and for one that is mid flight. This is a counter and not a
    /// flag because the imperative paths ([QueriesStorage::get], [UseQuery::invalidate] and the
    /// interval task) run the query on demand and may overlap with an execution already running.
    running: Rc<Cell<usize>>,

    /// How many [use_query] subscribers are mounted on this query right now.
    ///
    /// This is what cleanup keys on, not [QueryData::reactive_contexts]: one subscriber
    /// registers several reactive contexts (its scope plus every side effect that reads the
    /// query), and a context only unsubscribes when its scope storage drops, which happens
    /// after the scope's drop callbacks have already run. Counting subscribers explicitly
    /// makes "the last subscriber unmounted" independent of both fan out and teardown order.
    subscribers: Rc<Cell<usize>>,

    interval_task: Rc<RefCell<Option<(Duration, TaskHandle)>>>,
    clean_task: Rc<RefCell<Option<TaskHandle>>>,
}

impl<Q: QueryCapability> Clone for QueryData<Q> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            reactive_contexts: self.reactive_contexts.clone(),

            running: self.running.clone(),
            subscribers: self.subscribers.clone(),

            interval_task: self.interval_task.clone(),
            clean_task: self.clean_task.clone(),
        }
    }
}

impl<Q: QueryCapability> QueryData<Q> {
    /// Check if there is any execution of this query in flight.
    fn is_running(&self) -> bool {
        self.running.get() > 0
    }

    /// Mark an execution of this query as in flight for as long as the returned guard is alive.
    fn running_guard(&self) -> RunningGuard {
        RunningGuard::new(&self.running)
    }
}

/// Keeps a [QueryData] marked as running until it is dropped.
///
/// The count is restored on [Drop] rather than after awaiting the execution so that it is also
/// restored when the future running the query is cancelled instead of settling, e.g the interval
/// task being cancelled by [QueriesStorage::update_tasks] while it awaits a run.
struct RunningGuard(Rc<Cell<usize>>);

impl RunningGuard {
    fn new(running: &Rc<Cell<usize>>) -> Self {
        running.set(running.get() + 1);
        Self(running.clone())
    }
}

impl Drop for RunningGuard {
    fn drop(&mut self) {
        self.0.set(self.0.get().saturating_sub(1));
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
            running: Rc::default(),
            subscribers: Rc::default(),
            interval_task: Rc::default(),
            clean_task: Rc::default(),
        });

        // One more subscriber is mounted on this query, see [QueryData::subscribers]
        query_data.subscribers.set(query_data.subscribers.get() + 1);
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

    /// One [use_query] subscriber of `query` unmounted (or moved to another query). Once the
    /// last one is gone the interval task is cancelled and the entry is scheduled for
    /// cleanup via [QueriesStorage::spawn_clean_task].
    fn update_tasks(&mut self, query: Query<Q>) {
        let query_data = {
            let mut storage = self.storage.write_unchecked();

            let Some(query_data) = storage.get_mut(&query) else {
                return;
            };
            query_data
                .subscribers
                .set(query_data.subscribers.get().saturating_sub(1));

            // Other subscribers are still mounted, so every task stays alive
            if query_data.subscribers.get() > 0 {
                return;
            }

            // Cancel interval task
            if let Some((_, interval_task)) = query_data.interval_task.take() {
                interval_task.cancel();
            }

            query_data.clone()
        };

        self.spawn_clean_task(query, &query_data);
    }

    /// Schedule `query` for removal once it has had no subscriber for [Query::clean_time].
    /// A clean time of [Duration::MAX] disables cleanup entirely.
    ///
    /// A subscriber that mounts meanwhile is protected by [QueriesStorage::insert_or_get_query]
    /// cancelling this task outright; the subscriber check at fire time is only a backstop
    /// should that cancellation ever be bypassed. The load-bearing check is the running one:
    /// an execution still in flight defers the removal for another clean time, because
    /// removing the entry under it would orphan its settlement and make the next subscriber
    /// dispatch a duplicate execution, the very thing [QueryData::running] exists to prevent.
    fn spawn_clean_task(&self, query: Query<Q>, query_data: &QueryData<Q>) {
        let mut clean_task = query_data.clean_task.borrow_mut();

        // A merely overwritten task would keep ticking towards a stale removal
        if let Some(prev_clean_task) = clean_task.take() {
            prev_clean_task.cancel();
        }

        if query.clean_time == Duration::MAX {
            return;
        }

        let storage = self.storage;
        let subscribers = query_data.subscribers.clone();
        let running = query_data.running.clone();
        *clean_task = Some(spawn_forever(async move {
            loop {
                // Wait as long as the clean time is configured
                Timer::after(query.clean_time).await;

                // Backstop only: a mounting subscriber cancels this task, so a mounted
                // subscriber here means that cancellation was somehow bypassed
                if subscribers.get() > 0 {
                    break;
                }
                // An execution is still in flight, check again in another clean time
                if running.get() > 0 {
                    continue;
                }

                // Finally clear the query
                storage.write_unchecked().remove(&query);
                break;
            }
        }));
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
                running: Rc::default(),
                subscribers: Rc::default(),
                interval_task: Rc::default(),
                clean_task: Rc::default(),
            })
            .clone();

        // Release the storage borrow before awaiting the run, as writing to a State panics when it
        // is already borrowed: holding it across the await would make any query subscriber
        // mounting meanwhile panic instead of attaching to this execution
        drop(map);

        // Run the query if the value is stale
        if query_data.state.borrow().is_stale(&query) {
            // This is an imperative read, so it runs the query even if another execution is
            // already in flight: its caller awaits a settled value. It is still marked as
            // running so that subscribers mounting meanwhile attach to it instead of
            // dispatching yet another execution.
            let _running_guard = query_data.running_guard();

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

        // Schedule cleanup if no subscriber is mounted on this query
        if query_data.subscribers.get() == 0 {
            storage.spawn_clean_task(query, &query_data);
        }

        QueryReader {
            state: query_data.state,
        }
    }

    /// Acquires query storage from context and invalidates all queries
    ///
    /// Panics if query storage is not in context
    pub async fn invalidate_all() {
        let storage = consume_context::<QueriesStorage<Q>>();

        storage.inner_invalidate_all().await;
    }

    /// Non-panicking version of [`QueriesStorage::invalidate_all()`]
    pub async fn try_invalidate_all() {
        let Some(storage) = try_consume_context::<QueriesStorage<Q>>() else {
            return;
        };

        storage.inner_invalidate_all().await;
    }

    async fn inner_invalidate_all(self) {
        // Get all the queries
        let matching_queries = self.storage.read().clone().into_iter().collect::<Vec<_>>();
        let matching_queries = matching_queries
            .iter()
            .map(|(q, d)| (q, d))
            .collect::<Vec<_>>();

        // Invalidate the queries
        Self::run_queries(&matching_queries).await
    }

    /// Acquires query storage from context and invalidates matching queries
    ///
    /// Panics if query storage is not in context
    pub async fn invalidate_matching(matching_keys: Q::Keys) {
        let storage = consume_context::<QueriesStorage<Q>>();

        storage.inner_invalidate_matching(matching_keys).await;
    }

    /// Non-panicking version of [`QueriesStorage::invalidate_matching()`]
    pub async fn try_invalidate_matching(matching_keys: Q::Keys) {
        let Some(storage) = try_consume_context::<QueriesStorage<Q>>() else {
            return;
        };

        storage.inner_invalidate_matching(matching_keys).await;
    }

    async fn inner_invalidate_matching(self, matching_keys: Q::Keys) {
        // Get those queries that match
        let mut matching_queries = Vec::new();
        for (query, data) in self.storage.read().iter() {
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
            // Mark as running until this execution settles, so that a subscriber mounting
            // meanwhile attaches to it instead of dispatching a duplicate execution
            let running_guard = query_data.running_guard();

            // Set to Loading
            let res = mem::replace(&mut *query_data.state.borrow_mut(), QueryStateData::Pending)
                .into_loading();
            *query_data.state.borrow_mut() = res;
            for reactive_context in query_data.reactive_contexts.borrow().iter() {
                reactive_context.notify();
            }

            tasks.push(Box::pin(async move {
                let _running_guard = running_guard;

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
    /// An entry whose execution is still in flight is never cleared, and
    /// [Duration::MAX] disables cleanup entirely.
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
    /// If an execution of this query is already in flight the mounting subscriber attaches to it
    /// rather than running the query a second time.
    ///
    /// Defaults to [Duration::ZERO], meaning it is marked stale immediately after it has been used.
    pub fn stale_time(self, stale_time: Duration) -> Self {
        Self { stale_time, ..self }
    }

    /// For how long the data is kept cached after there are no more query subscribers.
    /// An entry whose execution is still in flight is never cleared, and
    /// [Duration::MAX] disables cleanup entirely.
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
    ///
    /// A handle whose entry was already cleaned (it outlived its subscriber by more than
    /// the clean time) reads as [QueryStateData::Pending] instead of panicking.
    pub fn read(&self) -> QueryReader<Q> {
        let storage = consume_context::<QueriesStorage<Q>>();
        let map = storage.storage.peek();
        let Some(query_data) = map.get(&self.query.read()).cloned() else {
            return QueryReader {
                state: Rc::new(RefCell::new(QueryStateData::Pending)),
            };
        };

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
    ///
    /// A handle whose entry was already cleaned reads as [QueryStateData::Pending]
    /// instead of panicking.
    pub fn peek(&self) -> QueryReader<Q> {
        let storage = consume_context::<QueriesStorage<Q>>();
        let map = storage.storage.peek();
        let Some(query_data) = map.get(&self.query.peek()).cloned() else {
            return QueryReader {
                state: Rc::new(RefCell::new(QueryStateData::Pending)),
            };
        };

        QueryReader {
            state: query_data.state,
        }
    }

    /// Invalidate this query and await its result.
    ///
    /// For a `sync` version use [UseQuery::invalidate].
    ///
    /// A handle whose entry was already cleaned resolves [QueryStateData::Pending]
    /// without running anything: the next mounting subscriber recreates and runs it.
    pub async fn invalidate_async(&self) -> QueryReader<Q> {
        let storage = consume_context::<QueriesStorage<Q>>();

        let query = self.query.peek().clone();
        let query_data = storage.storage.peek().get(&query).cloned();
        let Some(query_data) = query_data else {
            return QueryReader {
                state: Rc::new(RefCell::new(QueryStateData::Pending)),
            };
        };

        // Run the query
        QueriesStorage::run_queries(&[(&query, &query_data)]).await;

        QueryReader {
            state: query_data.state.clone(),
        }
    }

    /// Invalidate this query in the background.
    ///
    /// For an `async` version use [UseQuery::invalidate_async].
    ///
    /// A handle whose entry was already cleaned does nothing: the next mounting
    /// subscriber recreates and runs it.
    pub fn invalidate(&self) {
        let storage = consume_context::<QueriesStorage<Q>>();

        let query = self.query.peek().clone();
        let Some(query_data) = storage.storage.peek().get(&query).cloned() else {
            return;
        };

        // Run the query
        spawn_forever(async move { QueriesStorage::run_queries(&[(&query, &query_data)]).await });
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
/// A subscriber that mounts while an execution of its query is already in flight attaches to that
/// execution instead of dispatching a new one, so remounting never runs the query twice at once.
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
/// An entry whose execution is still in flight is never cleared, no matter the clean time:
/// removing it would orphan the eventual settlement and make the next subscriber dispatch a
/// duplicate execution. Cleanup waits for it to settle instead.
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

        // Immediately run the query if enabled and the value is stale, unless an execution is
        // already in flight: this subscriber reads the very same state, so it attaches to that
        // execution instead of dispatching a duplicate one. Without this a subscriber that
        // unmounts and remounts while its query runs would execute the capability twice
        // concurrently, as the running execution is deliberately not cancelled on unmount.
        if query.enabled && !query_data.is_running() && query_data.state.borrow().is_stale(query) {
            // Marked as running here, before spawning, and not just inside `run_queries`:
            // `spawn_forever` only queues the task, so any other subscriber mounting before the
            // runner gets to poll it would otherwise still see this query as idle
            let running_guard = query_data.running_guard();
            let query = query.clone();
            spawn_forever(async move {
                let _running_guard = running_guard;
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
