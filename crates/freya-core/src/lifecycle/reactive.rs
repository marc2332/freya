use crate::prelude::{
    State,
    use_state,
};

pub fn use_reactive<T: 'static + Clone + PartialEq>(value: &T) -> State<T> {
    let mut state = use_state(|| value.clone());

    if &*state.peek() != value {
        state.set(value.clone());
    }
    state
}
