//! # Async
//!
//! Freya has its own async runtime, so tasks can update reactive state directly.
//! These are the async primitives, each one documents when to use it and how in
//! its own definition.
//!
//! ## Tasks
//!
//! - [`spawn`](crate::prelude::spawn) runs a future attached to the current
//!   component scope, it gets cancelled when that component unmounts. Use it for
//!   async work owned by a component.
//! - [`spawn_in_window`](crate::prelude::spawn_in_window) runs a future attached
//!   to the window's root scope, it survives its component but stops when the
//!   window closes.
//! - [`spawn_global`](crate::prelude::spawn_global) runs a future on the event
//!   loop, it survives window closure and stops when the event loop exits.
//!   It can access [`GlobalContexts`](crate::prelude::GlobalContexts), but not
//!   component or window context.
//!
//! All three return a [`TaskHandle`](crate::prelude::TaskHandle) to cancel the task
//! manually. [`.owned()`](crate::prelude::TaskHandle::owned) upgrades it to an
//! [`OwnedTaskHandle`](crate::prelude::OwnedTaskHandle) that cancels the task
//! when its last clone is dropped.
//!
//! ## Hooks
//!
//! - [`use_future`](crate::prelude::use_future) wraps `spawn` and exposes the
//!   progress of the future as reactive state through a
//!   [`FutureTask`](crate::prelude::FutureTask), so you can render the
//!   [`Pending`](crate::prelude::FutureState::Pending),
//!   [`Loading`](crate::prelude::FutureState::Loading) and
//!   [`Fulfilled`](crate::prelude::FutureState::Fulfilled) cases without
//!   managing the task by hand. Its callback is reactive, any state read inside
//!   of it (outside the async block) restarts the future when it changes.
//!
//! ## See also
//!
//! - [Tokio Integration](crate::_docs::tokio_integration) to use crates that
//!   depend on Tokio.
//! - [State Management](crate::_docs::state_management) for caching and syncing
//!   async data with Freya Query.
