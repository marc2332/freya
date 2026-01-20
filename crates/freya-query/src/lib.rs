//! # Freya Query
//!
//! A powerful, async-focused data management library for Freya applications.
//! Inspired by React Query and SWR, it provides intelligent caching, background
//! updates, and automatic invalidation for async operations.
//!
//! ## Overview
//!
//! Freya Query manages two types of async operations:
//!
//! - **Queries**: Read operations that fetch and cache data
//! - **Mutations**: Write operations that modify data and can invalidate queries
//!
//! ## Key Features
//!
//! - **Automatic Caching**: Query results are cached and reused across components
//! - **Background Refetching**: Stale data is automatically refreshed in the background
//! - **Invalidation**: Mutations can invalidate related queries to keep data fresh
//! - **Deduplication**: Multiple identical queries are automatically deduplicated
//! - **Error Handling**: Built-in error states
//! - **Reactive**: Integrates seamlessly with Freya's reactive state system
//!
//! ## Basic Usage
//!
//! ### Queries
//!
//! ```rust,no_run
//! use freya::prelude::*;
//! use freya_query::prelude::*;
//!
//! # #[derive(Debug)]
//! # struct User;
//!
//! # async fn fetch_user(_id: u32) -> Result<User, String> {
//! #   Ok(User)
//! # }
//!
//! // Define a query capability
//! #[derive(Clone, PartialEq, Hash, Eq)]
//! struct FetchUser;
//!
//! impl QueryCapability for FetchUser {
//!     type Ok = User;
//!     type Err = String;
//!     type Keys = u32;
//!
//!     async fn run(&self, keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
//!         // Fetch user from API
//!         fetch_user(*keys).await
//!     }
//! }
//!
//! #[derive(PartialEq)]
//! struct UserProfile(u32);
//!
//! impl Component for UserProfile {
//!     fn render(&self) -> impl IntoElement {
//!         let user_query = use_query(Query::new(self.0, FetchUser));
//!
//!         format!("{:?}", user_query.read().state())
//!     }
//! }
//! ```
//!
//! ### Mutations
//!
//! ```rust,no_run
//! use freya::prelude::*;
//! use freya_query::prelude::*;
//!
//! # struct User;
//!
//! # async fn update_user(_id: u32, _name: &str) -> Result<User, String> {
//! #   Ok(User)
//! # }
//!
//! #[derive(Clone, PartialEq, Hash, Eq)]
//! struct UpdateUser {
//!     id: u32,
//! }
//!
//! // Define a query capability
//! # #[derive(Clone, PartialEq, Hash, Eq)]
//! # struct FetchUser;
//!
//! # impl QueryCapability for FetchUser {
//! #    type Ok = User;
//! #    type Err = String;
//! #    type Keys = u32;
//! #
//! #    async fn run(&self, keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
//! #        Ok(User)
//! #    }
//! # }
//!
//! impl MutationCapability for UpdateUser {
//!     type Ok = ();
//!     type Err = String;
//!     type Keys = String;
//!
//!     async fn run(&self, keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
//!         update_user(self.id, &keys).await?;
//!         Ok(())
//!     }
//!
//!     async fn on_settled(&self, keys: &Self::Keys, result: &Result<Self::Ok, Self::Err>) {
//!         if result.is_ok() {
//!             QueriesStorage::<FetchUser>::invalidate_matching(self.id).await;
//!         }
//!     }
//! }
//!
//! #[derive(PartialEq)]
//! struct UserEditor {
//!     user_id: u32,
//! }
//!
//! impl Component for UserEditor {
//!     fn render(&self) -> impl IntoElement {
//!         let mutation = use_mutation(Mutation::new(UpdateUser { id: self.user_id }));
//!
//!         Button::new()
//!             .child("Update User")
//!             .on_press(move |_| mutation.mutate("New Name".to_string()))
//!     }
//! }
//! ```
//!
//! ## Advanced Patterns
//!
//! ### Query Invalidation
//!
//! Mutations can invalidate queries to ensure data consistency:
//!
//! ```rust, ignore
//! # use freya_query::prelude::*;
//! // Invalidate all user queries
//! QueriesStorage::<FetchUser>::invalidate_all().await;
//!
//! // Invalidate specific user query
//! QueriesStorage::<FetchUser>::invalidate_matching(1).await;
//! ```
//!
//! ### Custom Query Matching
//!
//! Control which queries get invalidated by implementing custom matching logic:
//!
//! ```rust, no_run
//! # use freya_query::prelude::*;
//! # #[derive(Hash, Clone, Eq, PartialEq)]
//! # struct FetchUser { id: u32 };
//! impl QueryCapability for FetchUser {
//!     # type Ok = ();
//!     # type Err = String;
//!     # type Keys = u32;
//!     // ... other methods
//!
//!     # async fn run(&self, keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
//!     #     Ok(())
//!     # }
//!
//!     fn matches(&self, keys: &Self::Keys) -> bool {
//!         // Only match queries with the same user ID
//!         &self.id == keys
//!     }
//! }
//! ```
//!
//! ### Background Refetching
//!
//! Queries automatically refetch data in the background when components remount
//! or when explicitly invalidated by mutations.
//!
//! ## Architecture
//!
//! Freya Query uses a hierarchical caching system:
//!
//! - **Query Store**: Global cache of query results by capability type and keys
//! - **Mutation Store**: Tracks running mutations and their invalidation logic
//! - **Reactive Integration**: Seamlessly integrates with Freya's state management
//!
//! ## Error Handling
//!
//! Both queries and mutations return `Result<T, E>` types. Freya Query provides
//! utilities for handling loading states, errors, and retries.
//!
//! ## Performance
//!
//! - **Queries Deduplication**: Identical concurrent queries are automatically deduplicated
//! - **Smart Caching**: Results are cached until invalidated or expired
//! - **Minimal Re-renders**: Only components reading changed data re-render

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
