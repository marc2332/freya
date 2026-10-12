//! # UI and Components
//!
//! Freya uses a [declarative](https://en.wikipedia.org/wiki/Declarative_programming) model for the UI.
//! This means that you dont instantiate your UI, you rather declare it and Freya will take care of manage its lifecycle.
//!
//! Example of how the UI is declared:
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! # fn app() -> impl IntoElement {
//! rect()
//!     .background((255, 0, 0))
//!     .width(Size::fill())
//!     .height(Size::px(100.))
//!     .on_press(|_| println!("Clicked!"))
//! # }
//! ```
//!
//! You may split your UI in utility functions or **Components**
//!
//! ### [Component](freya_core::prelude::Component) trait
//!
//! Components are not just a way to split your UI into reusable pieces, they also serve as state boundaries where this means that when a state in your app changes only the components that are subscribed to such state
//! are actually recomputed, this is an optimization done by the user for Freya to do less work during diffing.
//!
//! ## App/Root Component
//! The app/root component is the component passed to [WindowConfig](crate::prelude::WindowConfig).
//!
//! For convenience it can be a `Fn() -> Element` instead of a struct that implements [App](freya_core::prelude::App).
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> impl IntoElement {
//!     "Hello, World!"
//! }
//! ```
//!
//! If you wanted to pass data from your **main** function to your **root** component you would need to make it use a struct that implements the [App](freya_core::prelude::App) trait, like this:
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! fn main() {
//!     launch(LaunchConfig::new().with_window(WindowConfig::new_app(MyApp { number: 1 })))
//! }
//!
//! struct MyApp {
//!     number: u8,
//! }
//!
//! impl App for MyApp {
//!     fn render(&self) -> impl IntoElement {
//!         label().text(self.number.to_string())
//!     }
//! }
//! ```
//!
//! To create reusable components you use the [Component](freya_core::prelude::Component) trait.
//!
//! ```rust
//! # use freya::prelude::*;
//! # use std::borrow::Cow;
//! // Reusable component that we might create as many times we want
//! #[derive(PartialEq)]
//! struct TextLabel(Cow<'static, str>);
//! impl Component for TextLabel {
//!     fn render(&self) -> impl IntoElement {
//!         label().text(self.0.clone())
//!     }
//! }
//!
//! fn app() -> impl IntoElement {
//!     rect()
//!         .child(TextLabel("Number 1".into()))
//!         .child("Number 2")
//!         .child(TextLabel("Number 3".into()))
//! }
//! ```
//!
//! Notice how all these component are returning an [`Element`](freya_core::prelude::Element), this is because `rect()` gives you a [`Rect`](freya_core::elements::rect::Rect) which implements `Into<Element>` / `IntoElement`, same happens for the rest of elements.
//! So, in other words, the [`Element`](freya_core::prelude::Element) contains the UI of that component.
//! Every time the component render function reruns a new UI is created and later diffed by Freya internally.
//!
//! ## Renders
//!
//! "Components renders" are simply when the component's `render` function runs, this can happen in multiple scenarios:
//!
//! 1. The component just got instantiated for the first time (also called mounted in other UI libraries)
//! 2. A state that this component is reading (thus subscribed to), got mutated
//! 3. The component data (also called props) changed (this is why `PartialEq` is required)
//!
//! > **Note:** The naming of `render` might give you the impression that it means the window canvas will effectively rerender again, it has nothing to do with it, in fact, a component might render (run its function) a thousand times but generate the exact same UI, if that was the case Freya would not render the canvas again.
//!
//! Consider this simple component:
//!
//! ```rust
//! # use freya::prelude::*;
//! #[derive(PartialEq)]
//! struct CoolComp;
//!
//! impl Component for CoolComp {
//!     // One run of this function is the same as saying one render of this component
//!     fn render(&self) -> impl IntoElement {
//!         let mut count = use_state(|| 0);
//!
//!         label()
//!             .on_press(move |_| *count.write() += 1)
//!             // Here we subscribe to `count` because we called .read() on it
//!             .text(format!("Increase {}", count.read()))
//!     }
//! }
//! ```
//!
//! ## Lists and Keys
//!
//! When you render a dynamic list, Freya needs to match each element to its previous version across
//! renders. Without any hint it pairs them up by index, which is fine until items are inserted,
//! removed or reordered. When that happens index-based matching can missassociate an element with the
//! wrong previous one, making local state or layout jump to another item.
//!
//! The [`key`](freya_core::elements::extensions::KeyExt::key) method gives an element a stable
//! identity so Freya can reconcile it correctly no matter where it ends up in the list.
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> impl IntoElement {
//!     let items = use_state(|| Vec::<String>::new());
//!
//!     rect().children(
//!         items
//!             .read()
//!             .iter()
//!             .enumerate()
//!             .map(|(index, item)| label().key(index).text(item.clone())),
//!     )
//! }
//! ```
//!
//! The key can be derived from any [`Hash`](std::hash::Hash) value. Whenever items can be inserted,
//! removed or reordered, prefer a value that uniquely and stably identifies the item.
//!
//! A custom [`Component`](freya_core::prelude::Component) can expose the same `.key` method by
//! implementing [`KeyExt`](freya_core::elements::extensions::KeyExt) over a stored
//! [`DiffKey`](freya_core::prelude::DiffKey) and forwarding it from
//! [`render_key`](freya_core::prelude::Component::render_key).
//!
//! ```rust
//! # use freya::prelude::*;
//! #[derive(PartialEq)]
//! struct Task {
//!     title: String,
//!     key: DiffKey,
//! }
//!
//! impl Task {
//!     fn new(title: String) -> Self {
//!         Self {
//!             title,
//!             key: DiffKey::None,
//!         }
//!     }
//! }
//!
//! impl KeyExt for Task {
//!     fn write_key(&mut self) -> &mut DiffKey {
//!         &mut self.key
//!     }
//! }
//!
//! impl Component for Task {
//!     fn render(&self) -> impl IntoElement {
//!         label().text(self.title.clone())
//!     }
//!
//!     // Use the key set through `.key(..)` and fall back to the default one when none was given.
//!     fn render_key(&self) -> DiffKey {
//!         self.key.clone().or(self.default_key())
//!     }
//! }
//!
//! fn app(ids: Vec<u64>) -> impl IntoElement {
//!     rect().children(
//!         ids.iter()
//!             .map(|id| Task::new(format!("Task {id}")).key(*id)),
//!     )
//! }
//! ```
//!
//! ## Utility Functions
//!
//! Not every piece of reusable UI needs to be a full [Component](freya_core::prelude::Component).
//! Sometimes a plain Rust function is simpler and more appropriate.
//!
//! When you just want to reuse or encapsulate a chunk of UI with no internal state, a plain
//! function is the simplest option, no boilerplate, no trait to implement.
//!
//! ```rust
//! # use freya::prelude::*;
//! fn colored_label(color: Color, text: &str) -> impl IntoElement {
//!     label().color(color).text(text.to_string())
//! }
//!
//! fn app() -> impl IntoElement {
//!     rect()
//!         .child(colored_label(Color::RED, "Error"))
//!         .child(colored_label(Color::GREEN, "Success"))
//! }
//! ```
