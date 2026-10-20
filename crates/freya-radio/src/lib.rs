//! # Freya Radio 🧬
//!
//! Typed, shared state with channel-based subscriptions for Freya applications.
//!
//! Share state across components and only re-render those affected by a change.
//! For local state, prefer `use_state`.
//!
//! ## State, stations, and channels
//!
//! - [`RadioStation`](prelude::RadioStation) holds state and subscriptions.
//!   [`use_init_radio_station`](prelude::use_init_radio_station) shares it with
//!   descendants through context.
//! - [`RadioChannel`](prelude::RadioChannel) defines which changes to listen for,
//!   usually with an enum variant for each kind of change.
//! - [`Radio`](prelude::Radio) is a copyable handle from [`use_radio`](prelude::use_radio).
//!   Reading it subscribes the current reactive context to its channel.
//!
//! Channels control notifications, not access. Every radio can read and change all
//! state, so choose a channel that describes the change.
//!
//! ## Getting started
//!
//! ```rust,no_run
//! use freya::{
//!     prelude::*,
//!     radio::*,
//! };
//!
//! #[derive(Default)]
//! struct AppState {
//!     count: i32,
//! }
//!
//! #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash, PartialOrd, Ord)]
//! enum AppChannel {
//!     Count,
//! }
//!
//! impl RadioChannel<AppState> for AppChannel {}
//!
//! fn app() -> impl IntoElement {
//!     use_init_radio_station::<AppState, AppChannel>(AppState::default);
//!     let mut radio = use_radio(AppChannel::Count);
//!     let count = radio.read().count;
//!
//!     rect()
//!         .on_press(move |_| radio.write().count += 1)
//!         .child(format!("Count: {count}"))
//! }
//! ```
//!
//! ## Why select only one channel per update?
//!
//! Writes and reducers choose one source channel.
//! [`RadioChannel::derive_channel`](prelude::RadioChannel::derive_channel) can then
//! notify several related channels.
//!
//! Defining these relationships once avoids repeated channel lists and forgotten
//! notifications that leave the UI out of date. Relationships can also change
//! without updating every caller.
//!
//! For example, changing an item can also notify a summary:
//!
//! ```rust,no_run
//! use freya::{
//!     prelude::*,
//!     radio::*,
//! };
//!
//! struct AppState;
//!
//! #[derive(PartialEq, Eq, Clone, Debug, Hash, PartialOrd, Ord)]
//! enum AppChannel {
//!     Item(usize),
//!     Summary,
//! }
//!
//! impl RadioChannel<AppState> for AppChannel {
//!     fn derive_channel(self, _state: &AppState) -> Vec<Self> {
//!         match self {
//!             Self::Item(index) => vec![Self::Item(index), Self::Summary],
//!             Self::Summary => vec![Self::Summary],
//!         }
//!     }
//! }
//! ```
//!
//! Returned channels are notified without calling `derive_channel` on them again.
//! Include the source channel to notify its subscribers too.
//!
//! ## Action-based updates with reducers
//!
//! Implement [`DataReducer`](prelude::DataReducer) to define state changes in one
//! place. Call [`RadioReducer::apply`](prelude::RadioReducer::apply) to run an action
//! and send notifications. The returned [`ChannelSelection`](prelude::ChannelSelection)
//! chooses the radio's channel, another channel, or silence. Both channel choices
//! use `derive_channel`.
//!
//! ```rust,no_run
//! use freya::{
//!     prelude::*,
//!     radio::*,
//! };
//!
//! struct Counter {
//!     count: i32,
//! }
//!
//! #[derive(PartialEq, Eq, Clone, Debug, Hash, PartialOrd, Ord)]
//! enum CounterChannel {
//!     Count,
//! }
//!
//! impl RadioChannel<Counter> for CounterChannel {}
//!
//! enum CounterAction {
//!     Increment,
//! }
//!
//! impl DataReducer for Counter {
//!     type Channel = CounterChannel;
//!     type Action = CounterAction;
//!
//!     fn reduce(&mut self, action: Self::Action) -> ChannelSelection<Self::Channel> {
//!         match action {
//!             CounterAction::Increment => self.count += 1,
//!         }
//!         ChannelSelection::Select(CounterChannel::Count)
//!     }
//! }
//!
//! fn increment(mut radio: Radio<Counter, CounterChannel>) {
//!     radio.apply(CounterAction::Increment);
//! }
//! ```
//!
//! For async actions, implement [`DataAsyncReducer`](prelude::DataAsyncReducer).
//! [`RadioAsyncReducer::async_apply`](prelude::RadioAsyncReducer::async_apply) starts
//! a Freya task and returns immediately. The reducer gets a radio handle, letting
//! it release state guards before awaiting.
//!
//! Use [`Radio::write_silently`](prelude::Radio::write_silently) to change state and
//! only notify through the final channel selection. Normal writes still notify
//! when their guards are dropped.
//!
//! ## Slices
//!
//! [`RadioSlice`](prelude::RadioSlice) and [`RadioSliceMut`](prelude::RadioSliceMut)
//! expose part of the state. Convert them to [`Readable`](prelude::Readable) or
//! [`Writable`](prelude::Writable) for components that don't depend on Radio.
//!
//! ## Sharing state across windows
//!
//! Create one [`RadioStation`](prelude::RadioStation) outside components with
//! [`RadioStation::create_global`](prelude::RadioStation::create_global).
//! Pass the same station to each window with `WindowConfig::with_root_context`.
//! Changes in either window notify subscribers in both.
//!
//! ```rust,no_run
//! use freya::{
//!     prelude::*,
//!     radio::*,
//! };
//!
//! #[derive(Default)]
//! struct AppState {
//!     count: i32,
//! }
//!
//! #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash, PartialOrd, Ord)]
//! enum AppChannel {
//!     Count,
//! }
//!
//! impl RadioChannel<AppState> for AppChannel {}
//!
//! fn main() {
//!     let station = RadioStation::<AppState, AppChannel>::create_global(AppState::default());
//!
//!     launch(
//!         LaunchConfig::new()
//!             .with_window(WindowConfig::new(app).with_root_context(station))
//!             .with_window(WindowConfig::new(app).with_root_context(station)),
//!     );
//! }
//!
//! fn app() -> impl IntoElement {
//!     let mut radio = use_radio(AppChannel::Count);
//!     let count = radio.read().count;
//!
//!     rect()
//!         .child(format!("Count: {count}"))
//!         .child(
//!             Button::new()
//!                 .on_press(move |_| radio.write().count += 1)
//!                 .child("+"),
//!         )
//! }
//! ```
//!
//! ## Examples
//!
//! - [`state_radio.rs`](https://github.com/marc2332/freya/blob/main/examples/state_radio.rs): Basic usage
//! - [`feature_tray_radio_state`](https://github.com/marc2332/freya/tree/main/examples/feature_tray_radio_state): Tray integration
//! - [`feature_multi_window_radio.rs`](https://github.com/marc2332/freya/blob/main/examples/feature_multi_window_radio.rs): Multi-window state sharing

pub mod hooks;
pub mod slice;

pub mod prelude {
    pub use freya_core::lifecycle::{
        readable::{
            IntoReadable,
            Readable,
        },
        writable::{
            IntoWritable,
            Writable,
        },
    };

    pub use crate::{
        hooks::*,
        slice::{
            RadioSlice,
            RadioSliceMut,
        },
    };
}

mod readable;
mod writable;
