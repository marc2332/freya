use crate::prelude::{
    State,
    use_state,
};

/// Allows converting a `&T` into a `State<T>` so that we can pass this value around without clonning it multiple times.
pub fn use_reactive<T: 'static + Clone + PartialEq>(value: &T) -> State<T> {
    let mut state = use_state(|| value.clone());

    if &*state.peek() != value {
        state.set(value.clone());
    }
    state
}
