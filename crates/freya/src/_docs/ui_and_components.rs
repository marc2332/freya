//! # UI and Components
//!
//! Freya uses a [declarative](https://en.wikipedia.org/wiki/Declarative_programming) model for the UI.
//! This means that you dont instantiate e.g Buttons, you simply declare them and Freya will take care of running them and painting them on screen.
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
//!     .on_mouse_up(|_| println!("Clicked!"))
//! # }
//! ```
//!
//! You can also split your UI in reusable pieces called **Components**.
//!
//! ### [Component](freya_core::prelude::Component) trait
//!
//! For normal components you may use the [Component](freya_core::prelude::Component) trait.
//!
//! ```rust
//! # use freya::prelude::*;
//! #[derive(PartialEq)]
//! struct App;
//!
//! impl Component for App {
//!     fn render(&self) -> impl IntoElement {
//!         "Hello, World!"
//!     }
//! }
//! ```
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
//! To separate the UI of the app you may create more components.
//!
//! ```rust
//! # use freya::prelude::*;
//! # use std::borrow::Cow;
//! // Reusable component that we might call as many times we want
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
//!             .on_mouse_up(move |_| *count.write() += 1)
//!             // Here we subscribe to `count` because we called .read() on it
//!             .text(format!("Increase {}", count.read()))
//!     }
//! }
//! ```
