use std::marker::PhantomData;

use crate::{
    prelude::{
        State,
        provide_context_for_scope_id,
        try_consume_context,
        use_hook,
    },
    scope_id::ScopeId,
};

pub struct UseId<T> {
    counter: State<usize>,
    _phantom: PhantomData<T>,
}

impl<T> Clone for UseId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for UseId<T> {}

impl<T: 'static> UseId<T> {
    /// Composable alternative to [use_id].
    pub fn get_in_hook() -> usize {
        let storage = match try_consume_context::<UseId<T>>() {
            Some(storage) => storage,
            None => {
                let use_id = UseId {
                    counter: State::create_in_scope(0, Some(ScopeId::ROOT)),
                    _phantom: PhantomData::<T>,
                };
                provide_context_for_scope_id(use_id, ScopeId::ROOT);
                use_id
            }
        };
        *storage.counter.peek()
    }
}

/// Get a unique for a given generic type.
///
/// Every component using this hook will get a different ID.
///
/// The ID does not change in between renders.
///
/// To use it inside other hooks check [UseId::get_in_hook]
pub fn use_id<T: 'static>() -> usize {
    use_hook(UseId::<T>::get_in_hook)
}
