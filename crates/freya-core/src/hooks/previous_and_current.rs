use std::mem;

use crate::prelude::{
    IntoReadable,
    State,
    use_side_effect,
    use_state,
};

/// Track the previous and current values of a reactive value.
///
/// This hook returns a `State<(T, T)>` where the first element is the previous value
/// and the second is the current value. Useful for animations or effects that need to
/// transition between values.
///
/// # Example
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let value = use_state(|| 0);
///     let values = use_previous_and_current(value);
///
///     rect().child(label().text(format!(
///         "Previous: {}, Current: {}",
///         values.read().0,
///         values.read().1
///     )))
/// }
/// ```
pub fn use_previous_and_current<T: 'static + Clone + PartialEq>(
    value: impl IntoReadable<T>,
) -> State<(T, T)> {
    let value = value.into_readable();
    let mut state = use_state(|| (value.read().clone(), value.read().clone()));

    use_side_effect(move || {
        let value = value.read();
        let mut state = state.write();
        let old_current = mem::replace(&mut state.1, value.clone());
        state.0 = old_current;
    });

    state
}
