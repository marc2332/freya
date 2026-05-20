//! # Freya Query
//!
//! A powerful, async-focused data management library for Freya applications,
//! inspired by React Query. It handles caching, background refetching,
//! deduplication, and automatic invalidation for async operations.
//!
//! It is available under the `query` feature flag of the `freya` crate:
//!
//! ```toml
//! [dependencies]
//! freya = { version = "...", features = ["query"] }
//! ```
//!
//! Freya's built-in async primitives (`spawn`, `spawn_forever`, `use_future`)
//! are great for *individual* async operations, but they don't share results
//! between components, deduplicate concurrent calls, or invalidate stale data.
//! Freya Query builds on top of those primitives and adds caching, background
//! refetching, mutations, and invalidation, making it the right choice for
//! fetching from an HTTP API, a database, or any other remote source.
//!
//! ## Overview
//!
//! Freya Query manages two types of operations:
//!
//! - **Queries** ([`use_query`](crate::query::use_query)): Read operations that
//!   fetch, cache, and reactively share data.
//! - **Mutations** ([`use_mutation`](crate::mutation::use_mutation)): Write
//!   operations that modify data and can invalidate queries.
//!
//! ## Key Features
//!
//! - **Automatic Caching**: Query results are cached and reused across components.
//! - **Background Refetching**: Stale data is automatically refreshed in the background.
//! - **Invalidation**: Mutations can invalidate related queries to keep data fresh.
//! - **Deduplication**: Multiple identical queries are automatically deduplicated.
//! - **Error Handling**: Built-in error states.
//! - **Reactive**: Integrates seamlessly with Freya's reactive state system.
//!
//! ## Queries
//!
//! ### Defining a query
//!
//! Implement [`QueryCapability`](crate::query::QueryCapability) on a type to
//! define how data is fetched:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # use freya::query::*;
//! #[derive(Clone, PartialEq, Hash, Eq)]
//! struct FetchUser;
//!
//! impl QueryCapability for FetchUser {
//!     type Ok = String;
//!     type Err = String;
//!     type Keys = u32;
//!
//!     async fn run(&self, user_id: &Self::Keys) -> Result<Self::Ok, Self::Err> {
//!         // Fetch from an API, database, etc.
//!         Ok(format!("User {user_id}"))
//!     }
//! }
//! ```
//!
//! ### Using a query in a component
//!
//! Call [`use_query`](crate::query::use_query) with a [`Query`](crate::query::Query) to subscribe
//! a component to cached, reactive data:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # use freya::query::*;
//! # #[derive(Clone, PartialEq, Hash, Eq)]
//! # struct FetchUser;
//! # impl QueryCapability for FetchUser {
//! #     type Ok = String;
//! #     type Err = String;
//! #     type Keys = u32;
//! #     async fn run(&self, user_id: &Self::Keys) -> Result<Self::Ok, Self::Err> {
//! #         Ok(format!("User {user_id}"))
//! #     }
//! # }
//! #[derive(PartialEq)]
//! struct UserProfile(u32);
//!
//! impl Component for UserProfile {
//!     fn render(&self) -> impl IntoElement {
//!         let query = use_query(Query::new(self.0, FetchUser));
//!
//!         match &*query.read().state() {
//!             QueryStateData::Pending => "Loading...".to_string(),
//!             QueryStateData::Loading { res } => {
//!                 format!("Refreshing... Previous: {:?}", res)
//!             }
//!             QueryStateData::Settled { res, .. } => {
//!                 format!("Result: {:?}", res)
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Multiple components using the same query (same capability type + same keys) share the
//! same cache entry. The query only runs once and all subscribers receive the result.
//!
//! ### Reading query state
//!
//! [`UseQuery`](crate::query::UseQuery) gives access to the query state, see its docs for
//! the full API. The state is exposed as [`QueryStateData`](crate::query::QueryStateData).
//!
//! ### Query configuration
//!
//! [`Query`](crate::query::Query) supports builder methods to control caching behavior.
//! See its docs for the full list of options (`stale_time`, `clean_time`, `interval_time`, `enable`).
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # use freya::query::*;
//! # use std::time::Duration;
//! # #[derive(Clone, PartialEq, Hash, Eq)]
//! # struct FetchUser;
//! # impl QueryCapability for FetchUser {
//! #     type Ok = String;
//! #     type Err = String;
//! #     type Keys = u32;
//! #     async fn run(&self, _: &Self::Keys) -> Result<Self::Ok, Self::Err> { Ok(String::new()) }
//! # }
//! # #[derive(PartialEq)]
//! # struct Example;
//! # impl Component for Example {
//! #     fn render(&self) -> impl IntoElement {
//! let user = use_query(
//!     Query::new(1, FetchUser)
//!         .stale_time(Duration::from_secs(300))
//!         .clean_time(Duration::from_secs(600))
//!         .interval_time(Duration::from_secs(30))
//!         .enable(true),
//! );
//! # rect()
//! #     }
//! # }
//! ```
//!
//! ### Invalidating queries
//!
//! You can manually trigger a re-fetch from a [`UseQuery`](crate::query::UseQuery):
//!
//! ```rust,ignore
//! // Fire-and-forget (spawns a background task)
//! user.invalidate();
//!
//! // Await the result
//! let result = user.invalidate_async().await;
//! ```
//!
//! For broader invalidation, use [`QueriesStorage`](crate::query::QueriesStorage):
//!
//! ```rust,ignore
//! // Re-run ALL FetchUser queries
//! QueriesStorage::<FetchUser>::invalidate_all().await;
//!
//! // Re-run only FetchUser queries matching specific keys
//! QueriesStorage::<FetchUser>::invalidate_matching(user_id).await;
//! ```
//!
//! `invalidate_matching` calls the [`matches()`](crate::query::QueryCapability::matches) method on each
//! cached query. By default `matches()` returns `true` (all queries match). Override it for selective invalidation:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # use freya::query::*;
//! #[derive(Clone, PartialEq, Hash, Eq)]
//! struct FetchUser {
//!     user_id: u32,
//! }
//!
//! impl QueryCapability for FetchUser {
//!     type Ok = String;
//!     type Err = String;
//!     type Keys = u32;
//!
//!     async fn run(&self, user_id: &Self::Keys) -> Result<Self::Ok, Self::Err> {
//!         Ok(format!("User {user_id}"))
//!     }
//!
//!     fn matches(&self, keys: &Self::Keys) -> bool {
//!         // Only invalidate if the user_id matches
//!         &self.user_id == keys
//!     }
//! }
//! ```
//!
//! ### Standalone queries
//!
//! To run a query outside of a component (e.g. from an async task), use
//! [`QueriesStorage::get()`](crate::query::QueriesStorage::get) with a [`GetQuery`](crate::query::GetQuery):
//!
//! ```rust,ignore
//! let result = QueriesStorage::<FetchUser>::get(
//!     GetQuery::new(user_id, FetchUser)
//!         .stale_time(Duration::from_secs(60))
//! ).await;
//! ```
//!
//! ## Mutations
//!
//! ### Defining a mutation
//!
//! Implement [`MutationCapability`](crate::mutation::MutationCapability) to define a write operation:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # use freya::query::*;
//! # #[derive(Clone, PartialEq, Hash, Eq)]
//! # struct FetchUser;
//! # impl QueryCapability for FetchUser {
//! #     type Ok = String;
//! #     type Err = String;
//! #     type Keys = u32;
//! #     async fn run(&self, _: &Self::Keys) -> Result<Self::Ok, Self::Err> { Ok(String::new()) }
//! # }
//! #[derive(Clone, PartialEq, Hash, Eq)]
//! struct UpdateUser {
//!     user_id: u32,
//! }
//!
//! impl MutationCapability for UpdateUser {
//!     type Ok = ();
//!     type Err = String;
//!     /// (field_name, new_value)
//!     type Keys = (String, String);
//!
//!     async fn run(&self, keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
//!         // Send update to the API
//!         Ok(())
//!     }
//!
//!     /// Called after `run()` completes. Use this to invalidate related queries.
//!     async fn on_settled(&self, _keys: &Self::Keys, result: &Result<Self::Ok, Self::Err>) {
//!         if result.is_ok() {
//!             QueriesStorage::<FetchUser>::invalidate_matching(self.user_id).await;
//!         }
//!     }
//! }
//! ```
//!
//! The [`on_settled`](crate::mutation::MutationCapability::on_settled) callback is the primary mechanism for
//! keeping query data consistent after mutations. It runs after `run()` regardless of success or failure.
//!
//! ### Using a mutation in a component
//!
//! Call [`use_mutation`](crate::mutation::use_mutation) with a [`Mutation`](crate::mutation::Mutation):
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # use freya::query::*;
//! # #[derive(Clone, PartialEq, Hash, Eq)]
//! # struct UpdateUser { user_id: u32 }
//! # impl MutationCapability for UpdateUser {
//! #     type Ok = ();
//! #     type Err = String;
//! #     type Keys = (String, String);
//! #     async fn run(&self, _: &Self::Keys) -> Result<Self::Ok, Self::Err> { Ok(()) }
//! # }
//! #[derive(PartialEq)]
//! struct UserEditor(u32);
//!
//! impl Component for UserEditor {
//!     fn render(&self) -> impl IntoElement {
//!         let mutation = use_mutation(Mutation::new(UpdateUser { user_id: self.0 }));
//!
//!         let status = match &*mutation.read().state() {
//!             MutationStateData::Pending => "Ready",
//!             MutationStateData::Loading { .. } => "Saving...",
//!             MutationStateData::Settled { res, .. } if res.is_ok() => "Saved!",
//!             MutationStateData::Settled { .. } => "Error",
//!         };
//!
//!         rect().child(status).child(
//!             Button::new()
//!                 .on_press(move |_| mutation.mutate(("name".to_string(), "Alice".to_string())))
//!                 .child("Save"),
//!         )
//!     }
//! }
//! ```
//!
//! See [`UseMutation`](crate::mutation::UseMutation) docs for the full API. The state is exposed
//! as [`MutationStateData`](crate::mutation::MutationStateData).
//!
//! [`Mutation`](crate::mutation::Mutation) also supports builder methods, see its docs.
//!
//! ## Captured values
//!
//! Query and mutation types must implement `PartialEq` and `Hash` since they are used as cache keys.
//! This is a problem for values like API clients or `State<T>` handles that should not affect cache identity.
//!
//! [`Captured<T>`](crate::captured::Captured) wraps a value so it is invisible to caching:
//! its `PartialEq` always returns `true` and its `Hash` is a no-op.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # use freya::query::*;
//! #[derive(Clone, PartialEq, Hash, Eq)]
//! struct FetchTodos(Captured<State<DbClient>>);
//!
//! # #[derive(Clone)]
//! # struct DbClient;
//! impl QueryCapability for FetchTodos {
//!     type Ok = Vec<String>;
//!     type Err = String;
//!     type Keys = ();
//!
//!     async fn run(&self, _keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
//!         let _client: &State<DbClient> = &self.0;
//!         Ok(vec![])
//!     }
//! }
//! ```
//!
//! `Captured<T>` implements `Deref<Target = T>` and `DerefMut`, so you can use the inner
//! value transparently.
//!
//! ## Examples
//!
//! - [`state_query.rs`](https://github.com/marc2332/freya/tree/main/examples/state_query.rs) - Basic query usage
//! - [`state_mutation.rs`](https://github.com/marc2332/freya/tree/main/examples/state_mutation.rs) - Query + mutation with invalidation
//! - [`hackernews.rs`](https://github.com/marc2332/freya/tree/main/examples/hackernews.rs) - Fetching from a real API with stale times
//! - [`state_query_sqlite/`](https://github.com/marc2332/freya/tree/main/examples/state_query_sqlite) - Full CRUD app with SQLite

pub mod captured;
pub mod mutation;
pub mod query;

pub mod prelude {
    pub use crate::{
        captured::*,
        mutation::*,
        query::*,
    };
}
