use crate::prelude::{
    State,
    use_state,
};

/// Convert a borrowed value (`&T`) into a component-scoped [`State<T>`].
///
/// Useful when a component owns fields (props) and you need a reactive `State<T>` that can
/// be passed to hooks, effects, or child components without cloning repeatedly.
///
/// The returned `State<T>` is initialized by cloning the provided `value`. On subsequent
/// renders the internal state will be updated to follow `value` whenever it changes
/// (using `PartialEq` to avoid unnecessary updates).
///
/// Example
/// ```rust,no_run
/// # use freya::prelude::*;
/// #[derive(Clone, PartialEq)]
/// struct Config {
///     value: i32,
/// }
///
/// struct MyComponent {
///     config: Config,
/// }
///
/// impl Component for MyComponent {
///     fn render(&self) -> impl IntoElement {
///         let config = use_reactive(&self.config);
///
///         use_side_effect(move || {
///             // `.read()` subscribes the effect to changes of `config`
///             let config = config.read();
///             println!("config value: {}", config.value);
///         });
///
///         rect()
///     }
/// }
/// ```
///
/// Notes:
/// - Call `use_reactive` at the top level of your component's `render` method like other hooks.
/// - The hook avoids extra cloning by only setting the internal state when `value` differs.
pub fn use_reactive<T: 'static + Clone + PartialEq>(value: &T) -> State<T> {
    let mut state = use_state(|| value.clone());

    if &*state.peek() != value {
        state.set(value.clone());
    }
    state
}
