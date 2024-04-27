//! # Testing
//!
//! `freya-testing` is a headless renderer for freya components, which means you can simulate a graphical environment
//! with no need to actually draw anything, a perfect fit for testing.
//!
//! You can use the `launch_test` function to run the tests of the component.
//! It returns a set of utilities to interact with the component.
//!
//! ## Stateless example
//!
//! Simply asserts that the component renders a label with the text `"Hello World!"`.
//!
//! ```rust, no_run
//! #[tokio::test]
//! async fn test() {
//!     fn our_component() -> Element {
//!         rsx!(
//!             label {
//!                 "Hello World!"
//!             }
//!         )
//!     }
//!
//!     let mut utils = launch_test(our_component);
//!
//!     let root = utils.root(); // Get the root element of your app
//!     let label = root.get(0); // Get the children of the root in the index 0
//!     let label_text = label.get(0);
//!
//!     assert_eq!(label_text.text(), Some("Hello World!"));
//! }
//! ```
//!
//! ## Stateful example
//!
//! If the component has logic that might execute asynchronously, you need to wait for the component
//! to update using the `wait_for_update` function before asserting the result.
//!
//! Here, the component has a state that is `false` by default, but once mounted, it updates the state to `true`.
//!
//! ```rust, no_run
//! #[tokio::test]
//! async fn dynamic_test() {
//!     fn dynamic_component() -> Element {
//!         let mut state = use_signal(|| false);
//!
//!         use_hook(move || {
//!             state.set(true);
//!         });
//!
//!         rsx!(
//!             label {
//!                 "Is enabled? {state}"
//!             }
//!         )
//!     }
//!
//!     let mut utils = launch_test(dynamic_component);
//!
//!     let root = utils.root();
//!     let label = root.get(0);
//!
//!     assert_eq!(label.get(0).text(), Some("Is enabled? false"));
//!
//!     // This will poll the VirtualDOM and apply the new changes
//!     utils.wait_for_update().await;
//!
//!     assert_eq!(label.get(0).text(), Some("Is enabled? true"));
//! }
//! ```
//!
//! ## Events example
//!
//! You can simulate events on the component, for example, simulate a click event on a `rect` and assert that the state was updated.
//!
//! ```rust, no_run
//! #[tokio::test]
//! async fn event_test() {
//!     fn event_component() -> Element {
//!         let mut enabled = use_signal(|| false);
//!
//!         rsx!(
//!             rect {
//!                 width: "100%",
//!                 height: "100%",
//!                 background: "red",
//!                 onclick: move |_| {
//!                     enabled.set(true);
//!                 },
//!                 label {
//!                     "Is enabled? {enabled}"
//!                 }
//!             }
//!         )
//!     }
//!
//!     let mut utils = launch_test(event_component);
//!
//!     let rect = utils.root().get(0);
//!     let label = rect.get(0);
//!
//!     utils.wait_for_update().await;
//!
//!     let text = label.get(0);
//!     assert_eq!(text.text(), Some("Is enabled? false"));
//!
//!     // Push a click event to the events queue
//!     utils.push_event(PlatformEvent::Mouse {
//!         name: "click",
//!         cursor: (5.0, 5.0).into(),
//!         button: Some(MouseButton::Left),
//!     });
//!
//!     // Poll the VirtualDOM with the new events
//!     utils.wait_for_update().await;
//!
//!     // Because the click event was sent, and the state updated, the text was changed as well!
//!     let text = label.get(0);
//!     assert_eq!(text.text(), Some("Is enabled? true"));
//! }
//! ```
//!
//! ## Configuration example
//!
//! The `launch_test` comes with a default configuration, but you can pass your own config with the `launch_test_with_config` function.
//!
//! Here is an example of how to can set our custom window size:
//!
//! ```rust, no_run
//! #[tokio::test]
//! async fn test() {
//!     fn our_component() -> Element {
//!         rsx!(
//!             label {
//!                 "Hello World!"
//!             }
//!         )
//!     }
//!
//!     let mut utils = launch_test_with_config(
//!         our_component,
//!         TestingConfig {
//!             size: (500.0, 800.0).into(),
//!             ..TestingConfig::default()
//!         },
//!     );
//!
//!     let root = utils.root();
//!     let label = root.get(0);
//!     let label_text = label.get(0);
//!
//!     assert_eq!(label_text.text(), Some("Hello World!"));
//! }
//! ````

pub mod config;
pub mod launch;
pub mod test_handler;
pub mod test_node;
pub mod test_utils;

const SCALE_FACTOR: f64 = 1.0;

pub mod prelude {
    pub use crate::config::*;
    pub use crate::launch::*;
    pub use crate::test_handler::*;
    pub use crate::test_node::*;
    pub use crate::test_utils::*;
    pub use freya_core::prelude::*;
    pub use freya_node_state::*;
}
