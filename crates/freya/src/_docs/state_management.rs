//! # State Management
//!
//! Freya provides several options for managing state in your applications.
//! This guide covers local state with [`use_state`](crate::prelude::use_state) and global state with **Freya Radio**.
//!
//! ## Local State
//!
//! Local state is managed with the [`use_state`](crate::prelude::use_state) hook.
//! It's perfect for component-specific state that doesn't need to be shared.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! #[derive(PartialEq)]
//! struct Counter {}
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
//! ## Global State with Freya Radio 🧬
//!
//! For complex applications that need to share state across multiple components,
//! Freya Radio provides a powerful global state management system with fine-grained reactivity.
//!
//! ### Key Concepts
//!
//! - **RadioStation**: The central hub that holds the global state and manages subscriptions.
//! - **RadioChannel**: Defines channels for subscribing to specific types of state changes.
//! - **Radio**: A reactive handle to the state for a specific channel.
//!
//! ### Basic Usage
//!
//! First, define your state type and channels:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # use freya::radio::*;
//! #[derive(Default, Clone)]
//! struct AppState {
//!     count: i32,
//! }
//!
//! #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
//! enum AppChannel {
//!     Count,
//! }
//!
//! impl RadioChannel<AppState> for AppChannel {}
//! ```
//!
//! Then, initialize the radio station and use it in components:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # use freya::radio::*;
//! # #[derive(Default, Clone)]
//! # struct AppState { count: i32 }
//! #
//! # #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
//! # enum AppChannel { Count }
//! #
//! # impl RadioChannel<AppState> for AppChannel {}
//! fn app() -> impl IntoElement {
//!     // Initialize the radio station
//!     use_init_radio_station::<AppState, AppChannel>(AppState::default);
//!
//!     rect().child(Counter {})
//! }
//!
//! #[derive(PartialEq)]
//! struct Counter {}
//!
//! impl Component for Counter {
//!     fn render(&self) -> impl IntoElement {
//!         // Subscribe to the Count channel
//!         let mut radio = use_radio(AppChannel::Count);
//!
//!         rect()
//!             .child(format!("Count: {}", radio.read().count))
//!             .child(
//!                 Button::new()
//!                     .on_press(move |_| radio.write().count += 1)
//!                     .child("+"),
//!             )
//!     }
//! }
//! ```
//!
//! ### Multiple Channels
//!
//! You can use multiple channels for different types of updates:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # use freya::radio::*;
//! #[derive(Default, Clone)]
//! struct TodoState {
//!     todos: Vec<String>,
//!     filter: Filter,
//! }
//!
//! #[derive(Clone, Default)]
//! enum Filter {
//!     #[default]
//!     All,
//!     Completed,
//!     Pending,
//! }
//!
//! #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
//! enum TodoChannel {
//!     AddTodo,
//!     ToggleTodo(usize),
//!     ChangeFilter,
//! }
//!
//! impl RadioChannel<TodoState> for TodoChannel {
//!     fn derive_channel(self, _state: &TodoState) -> Vec<Self> {
//!         match self {
//!             TodoChannel::AddTodo | TodoChannel::ToggleTodo(_) => {
//!                 vec![self, TodoChannel::ChangeFilter] // Also notify filter subscribers
//!             }
//!             TodoChannel::ChangeFilter => vec![self],
//!         }
//!     }
//! }
//!
//! fn app() -> impl IntoElement {
//!     use_init_radio_station::<TodoState, TodoChannel>(TodoState::default);
//!
//!     rect().child(TodoList {}).child(FilterSelector {})
//! }
//!
//! #[derive(PartialEq)]
//! struct TodoList {}
//!
//! impl Component for TodoList {
//!     fn render(&self) -> impl IntoElement {
//!         let todos = use_radio(TodoChannel::AddTodo);
//!
//!         rect().child(format!("Todos: {}", todos.read().todos.len()))
//!     }
//! }
//!
//! #[derive(PartialEq)]
//! struct FilterSelector {}
//!
//! impl Component for FilterSelector {
//!     fn render(&self) -> impl IntoElement {
//!         let mut radio = use_radio(TodoChannel::ChangeFilter);
//!
//!         rect()
//!             .child(
//!                 Button::new()
//!                     .on_press(move |_| radio.write().filter = Filter::All)
//!                     .child("All"),
//!             )
//!             .child(
//!                 Button::new()
//!                     .on_press(move |_| radio.write().filter = Filter::Completed)
//!                     .child("Completed"),
//!             )
//!     }
//! }
//! ```
//!
//! ### Multi-Window Applications
//!
//! For applications with multiple windows, use a global radio station:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # use freya::radio::*;
//! #[derive(Default, Clone)]
//! struct AppState {
//!     count: i32,
//! }
//!
//! #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
//! enum AppChannel {
//!     Count,
//! }
//!
//! impl RadioChannel<AppState> for AppChannel {}
//!
//! fn main() {
//!     let radio_station = RadioStation::create_global(AppState::default());
//!
//!     launch(
//!         LaunchConfig::new()
//!             .with_window(WindowConfig::new(AppComponent::new(Window1 {
//!                 radio_station,
//!             })))
//!             .with_window(WindowConfig::new(AppComponent::new(Window2 {
//!                 radio_station,
//!             }))),
//!     );
//! }
//!
//! struct Window1 {
//!     radio_station: RadioStation<AppState, AppChannel>,
//! }
//!
//! impl Component for Window1 {
//!     fn render(&self) -> impl IntoElement {
//!         use_share_radio(move || self.radio_station);
//!         let mut radio = use_radio(AppChannel::Count);
//!
//!         rect()
//!             .child(format!("Window 1: {}", radio.read().count))
//!             .child(
//!                 Button::new()
//!                     .on_press(move |_| radio.write().count += 1)
//!                     .child("+"),
//!             )
//!     }
//! }
//!
//! struct Window2 {
//!     radio_station: RadioStation<AppState, AppChannel>,
//! }
//!
//! impl Component for Window2 {
//!     fn render(&self) -> impl IntoElement {
//!         use_share_radio(move || self.radio_station);
//!         let radio = use_radio(AppChannel::Count);
//!
//!         rect().child(format!("Window 2: {}", radio.read().count))
//!     }
//! }
//! ```
//!
//! ### Reducers
//!
//! For complex state updates, implement the reducer pattern:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # use freya::radio::*;
//! #[derive(Clone)]
//! struct CounterState {
//!     count: i32,
//! }
//!
//! #[derive(Clone)]
//! enum CounterAction {
//!     Increment,
//!     Decrement,
//!     Set(i32),
//! }
//!
//! #[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
//! enum CounterChannel {
//!     Count,
//! }
//!
//! impl RadioChannel<CounterState> for CounterChannel {}
//!
//! impl DataReducer for CounterState {
//!     type Channel = CounterChannel;
//!     type Action = CounterAction;
//!
//!     fn reduce(&mut self, action: CounterAction) -> ChannelSelection<CounterChannel> {
//!         match action {
//!             CounterAction::Increment => self.count += 1,
//!             CounterAction::Decrement => self.count -= 1,
//!             CounterAction::Set(value) => self.count = value,
//!         }
//!         ChannelSelection::Current
//!     }
//! }
//!
//! #[derive(PartialEq)]
//! struct CounterComponent {}
//!
//! impl Component for CounterComponent {
//!     fn render(&self) -> impl IntoElement {
//!         let mut radio = use_radio(CounterChannel::Count);
//!
//!         rect()
//!             .child(
//!                 Button::new()
//!                     .on_press(move |_| {
//!                         radio.apply(CounterAction::Increment);
//!                     })
//!                     .child("+"),
//!             )
//!             .child(format!("{}", radio.read().count))
//!             .child(
//!                 Button::new()
//!                     .on_press(move |_| {
//!                         radio.apply(CounterAction::Decrement);
//!                     })
//!                     .child("-"),
//!             )
//!     }
//! }
//! ```
//!
//! ## Choosing Between Local and Global State
//!
//! - **Use local state** (`use_state`) for:
//!   - Component-specific data
//!   - Simple state that doesn't need precise updates
//!
//! - **Use Freya Radio** for:
//!   - Application-wide state
//!   - Complex state logic with multiple subscribers
//!   - Apps that require precise updates for max performance
//!   - Multi-window applications
//!
//! ## Examples
//!
//! Check out these examples in the repository:
//!
//! - [`state_radio.rs`](https://github.com/marc2332/freya/tree/main/examples/state_radio.rs) - Basic radio usage
//! - [`feature_tray_radio_state.rs`](https://github.com/marc2332/freya/tree/main/examples/feature_tray_radio_state.rs) - Tray integration
//! - [`feature_multi_window_radio_state.rs`](https://github.com/marc2332/freya/tree/main/examples/feature_multi_window_radio_state.rs) - Multi-window state sharing
