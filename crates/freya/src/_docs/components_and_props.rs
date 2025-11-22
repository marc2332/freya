//! # Components
//!
//! Freya apps are composed of components, these are structs that implement the [Render](freya_core::prelude::Render) trait.
//!
//! > You can learn more about how the UI is defined in the [UI](crate::_docs::ui) chapter.
//!
//! For convenience the root component can be a `Fn() -> Element` instead of a struct that implements `Render`.
//!
//! ### `fn() -> Element`
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> impl IntoElement {
//!     "Hello, World!"
//! }
//! ```
//!
//!  ### `Render` trait
//!
//! ```rust
//! # use freya::prelude::*;
//! #[derive(PartialEq)]
//! struct App;
//!
//! impl Render for App {
//!     fn render(&self) -> impl IntoElement {
//!         "Hello, World!"
//!     }
//! }
//! ```
//!
//! To separate the UI of our app you may create more components.
//!
//! ```rust
//! # use freya::prelude::*;
//! # use std::borrow::Cow;
//!
//! // Reusable component that we might call as many times we want
//! #[derive(PartialEq)]
//! struct TextLabel(Cow<'static, str>);
//! impl Render for TextLabel {
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
//! ## Renders
//!
//! "Components renders" are simply when the component's `render` function runs, this can happen in multiple scanarios:
//!
//! 1. The component just got instanciated for the first time (also called mounted in other UI libraries)
//! 2. A state that this component is reading (thus subscribed to), got written
//! 3. The component data changed (this is why `PartialEq` is required)
//!
//! > **Note:** The naming of `render` might give you the impression that it means the window canvas will effectively rerender again, it has nothing to do with it, in fact, a component might render (run its function) a thousand times but generate the exact same UI, if that was the case Freya would not render the canvas again.
//!
//! Consider this simple component:
//!
//! ```rust
//! # use freya::prelude::*;
//! #[derive(PartialEq)]
//! struct CoolComp;
//! impl Render for CoolComp {
//!     // One run of this function is the same saying as one render of this component
//!     fn render(&self) -> impl IntoElement {
//!         let mut count = use_state(|| 0);
//!
//!         label()
//!             .on_mouse_up(move |_| *count.write() += 1)
//!             .text(format!("Increase {}", count.read()))
//!     }
//! }
//! ```
