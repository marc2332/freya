//! # State Management
//!
//! Freya provides several options for managing state in your applications.
//!
//! ### Available APIs
//! - Local state ([`use_state`](crate::prelude::use_state)): State of components that is not trivial to be shared with the whole app, example: hovering a button.
//! - [Freya Radio](freya_radio): Shared application state with channel-based updates, for example tabs with independent state.
//! - `Readable`/`Writable`: When you want to receive some kind of reactive state without knowing its specific backing storage, example: a reusable component.
//!
//! ## Local State
//!
//! Local state is managed with the [`use_state`](crate::prelude::use_state) hook.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! #[derive(PartialEq)]
//! struct Counter;
//!
//! impl Component for Counter {
//!     fn render(&self) -> impl IntoElement {
//!         let mut count = use_state(|| 0);
//!
//!         rect().child(format!("Count: {}", *count.read())).child(
//!             Button::new()
//!                 .on_press(move |_| *count.write() += 1)
//!                 .child("+"),
//!         )
//!     }
//! }
//! ```
//!
//! You can pass it to descendant components too:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! #[derive(PartialEq)]
//! struct Counter;
//!
//! impl Component for Counter {
//!     fn render(&self) -> impl IntoElement {
//!         let mut count = use_state(|| 0);
//!
//!         rect().child(AnotherCounter(count)).child(
//!             Button::new()
//!                 .on_press(move |_| *count.write() += 1)
//!                 .child("+"),
//!         )
//!     }
//! }
//!
//! #[derive(PartialEq)]
//! struct AnotherCounter(State<i32>);
//! impl Component for AnotherCounter {
//!     fn render(&self) -> impl IntoElement {
//!         self.0.read().to_string()
//!     }
//! }
//! ```
//!
//! ## Global State with Freya Radio 🧬
//!
//! Use [Freya Radio](freya_radio) when components need shared application state
//! with fine-grained updates, including state shared across windows.
//! Components subscribe to channels so unrelated changes don't trigger re-renders.
//!
//! Channels control notifications, not access to the state. Each radio can read
//! and change the whole state, so choose a channel that describes the change.
//!
//! See the [Freya Radio guide](freya_radio) for setup, channel relationships,
//! reducers, slices, and sharing a station across windows.
//!
//! ## Readable and Writable interfaces
//!
//! Freya provides [`Readable<T>`](crate::prelude::Readable) and [`Writable<T>`](crate::prelude::Writable)
//! as type-erased abstractions over different state sources. These allow components to accept state
//! without knowing whether it comes from local state (`use_state`) or global state (Freya Radio).
//!
//! ### Writable
//!
//! [`Writable<T>`](crate::prelude::Writable) is for state that can be both read and written to.
//! Components like [`Input`](crate::components::Input) accept `Writable` values, allowing you to
//! pass any state source that can be converted to a `Writable`.
//!
//! Sources that can be converted to `Writable`:
//! - [`State<T>`](crate::prelude::State) from `use_state` via [`IntoWritable`](crate::prelude::IntoWritable)
//! - [`RadioSliceMut`](freya_radio::prelude::RadioSliceMut) from Freya Radio via [`IntoWritable`](crate::prelude::IntoWritable)
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # use freya_radio::prelude::*;
//! # #[derive(Default, Clone)]
//! # struct AppState { name: String }
//! #
//! # #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
//! # enum AppChannel { Name }
//! #
//! # impl RadioChannel<AppState> for AppChannel {}
//! #[derive(PartialEq)]
//! struct NameInput {
//!     name: Writable<String>,
//! }
//!
//! impl Component for NameInput {
//!     fn render(&self) -> impl IntoElement {
//!         // Can read and write to the state
//!         Input::new(self.name.clone())
//!     }
//! }
//!
//! fn app() -> impl IntoElement {
//!     use_init_radio_station::<AppState, AppChannel>(AppState::default);
//!
//!     let local_name = use_state(|| "Alice".to_string());
//!     let radio = use_radio(AppChannel::Name);
//!     let name_slice = radio.slice_mut_current(|s| &mut s.name);
//!
//!     rect()
//!         // Pass local state as Writable
//!         .child(NameInput {
//!             name: local_name.into_writable(),
//!         })
//!         // Pass radio slice as Writable
//!         .child(NameInput {
//!             name: name_slice.into_writable(),
//!         })
//! }
//! ```
//!
//! ### Readable
//!
//! [`Readable<T>`](crate::prelude::Readable) is for read-only state. It's the same concept as
//! `Writable` but only exposes read operations. This is useful when a component only needs to
//! display data without modifying it.
//!
//! Note that calling `.read()` on a `Readable` only subscribes to changes if the underlying
//! source supports it. For example, a `Readable` created from a [`State<T>`](crate::prelude::State)
//! or a `RadioSlice` will subscribe, but a `Readable` created from a plain value will not.
//!
//! Sources that can be converted to `Readable`:
//! - [`State<T>`](crate::prelude::State) from `use_state` via [`IntoReadable`](crate::prelude::IntoReadable)
//! - [`Memo<T>`](crate::prelude::Memo) from `use_memo` via [`IntoReadable`](crate::prelude::IntoReadable)
//! - [`RadioSlice`](freya_radio::prelude::RadioSlice) from Freya Radio via [`IntoReadable`](crate::prelude::IntoReadable)
//! - [`Writable<T>`](crate::prelude::Writable) via [`From<Writable<T>>`](crate::prelude::Readable)
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # use freya_radio::prelude::*;
//! # #[derive(Default, Clone)]
//! # struct AppState { count: i32 }
//! #
//! # #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
//! # enum AppChannel { Count }
//! #
//! # impl RadioChannel<AppState> for AppChannel {}
//! #[derive(PartialEq)]
//! struct Counter {
//!     count: Readable<i32>,
//! }
//!
//! impl Component for Counter {
//!     fn render(&self) -> impl IntoElement {
//!         // Can only read the value
//!         format!("Count: {}", self.count.read())
//!     }
//! }
//!
//! fn app() -> impl IntoElement {
//!     use_init_radio_station::<AppState, AppChannel>(AppState::default);
//!
//!     let local_count = use_state(|| 0);
//!     let radio = use_radio(AppChannel::Count);
//!     let count_slice = radio.slice_current(|s| &s.count);
//!
//!     rect()
//!         // Pass local state as Readable
//!         .child(Counter {
//!             count: local_count.into_readable(),
//!         })
//!         // Pass radio slice as Readable
//!         .child(Counter {
//!             count: count_slice.into_readable(),
//!         })
//! }
//! ```
//!
//! ## Choosing Between Local and Global State
//!
//! - **Use local state** (`use_state`) for:
//!   - Component-specific data
//!   - Simple state that doesn't need precise updates
//!
//! - **Use [Freya Radio](freya_radio)** for:
//!   - Application-wide state
//!   - Complex state logic with multiple subscribers
//!   - Apps that require precise updates for max performance
//!   - Multi-window applications
//!
//! - **Use [Freya Query](freya_query)** for:
//!   - Async data from external sources (HTTP APIs, databases)
//!   - Cached, deduplicated, and automatically refreshed data
