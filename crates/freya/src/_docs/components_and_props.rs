//! # Components
//!
//! Freya apps are composed of components, these are functions that might or not receive some input as **Props** and return the UI as **Element**.
//!
//! > You can learn more about how the UI is defined in the [UI](crate::_docs::ui) chapter.
//!
//! This is how a simple root component looks like:
//!
//! ```rust
//! # use freya::prelude::*;
//! // Usually, the root component of a Freya app is named `app`,
//! // but it is not a requirement
//! fn app() -> Element {
//!     rsx!(
//!         label {
//!             "Hello, World!"
//!         }
//!     )
//! }
//! ```
//! This is perfectly fine but we might consider splitting the app in multiple components as it grows. This would allow to have reusable components
//! and also help maintaining and scaling the app.
//!
//! Lets create a reusable component:
//!
//! ```rust
//! # use freya::prelude::*;
//!
//! // Reusable component that we might call as many times we want
//! #[component]
//! fn TextLabel(text: String) -> Element {
//!     rsx!(
//!         label {
//!             "{text}"
//!         }
//!     )
//! }
//!
//! fn app() -> Element {
//!     rsx!(
//!         // By declaring this element using `TextLabel`
//!         // we are declaring an instance of the component
//!         TextLabel {
//!             text: "Number 1"
//!         }
//!         label {
//!             "Number 2"
//!         }
//!         // Another instance of the same component
//!         TextLabel {
//!             text: "Number 3"
//!         }
//!     )
//! }
//! ```
//!
//! Notice how we anotate our `TextLabel` component with the macro `#[component]`, this will transform every argument of the function (just `text: String` in this case) to a component prop.
//!
//! For more complex components you might want to put the props in an external struct intead of using the `#[components]` macro:
//!
//! ```rust
//! # use freya::prelude::*;
//! #[derive(Props, PartialEq, Clone)]
//! struct TextLabelProps {
//!     text: String
//! }
//!
//! fn TextLabel(TextLabelProps { text }: TextLabelProps) -> Element {
//!     rsx!(
//!         label {
//!             "{text}"
//!         }
//!     )
//! }
//! ```
//!
//! ## Renders
//!
//! Components renders are just when component function runs, this can happen in multiple scanarios:
//!
//! 1. The component just got instanciated for the first time
//! 2. A signal that this component is reading, got written
//! 3. The component props changed
//!
//! > **Note:** The naming of `render` might give you the impression that it means the app will effectively rerender again, it has nothing to do with it, in fact, a component might render (run its function) a thousand times but generate the exact same RSX, if that was the case Freya would not render it again.
//!
//! Consider this simple component:
//!
//! ```rust
//! # use freya::prelude::*;
//! #[component]
//! fn CoolComp() -> Element {
//!     let mut count = use_signal(|| 0);
//!
//!     // One run of this function is the same as one render of this component
//!
//!     rsx!(
//!         label {
//!             // Update the signal value
//!             onclick: move |_| count += 1,
//!
//!             // By embedding the count in this text the component is subscribed to any change of the `count` siganal
//!             "Increase {count}"
//!             // So, everytime the `count` signal is written, the component rerenders.
//!         }
//!     )
//! }
//! ```
//!
//! #### You can now learn about [Hooks](crate::_docs::hooks).
