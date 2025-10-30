use std::{
    marker::PhantomData,
    sync::{
        Arc,
        atomic::{
            AtomicUsize,
            Ordering,
        },
    },
};

use crate::{
    prelude::{
        provide_context_for_scope_id,
        try_consume_context,
        use_hook,
    },
    scope_id::ScopeId,
};

pub struct UseId<T> {
    counter: Arc<AtomicUsize>,
    _phantom: PhantomData<T>,
}

impl<T> Clone for UseId<T> {
    fn clone(&self) -> Self {
        Self {
            counter: self.counter.clone(),
            _phantom: self._phantom,
        }
    }
}

impl<T: 'static> UseId<T> {
    /// Composable alternative to [use_id].
    pub fn get_in_hook() -> usize {
        let storage = match try_consume_context::<UseId<T>>() {
            Some(storage) => storage,
            None => {
                provide_context_for_scope_id(
                    UseId {
                        counter: Arc::default(),
                        _phantom: PhantomData::<T>,
                    },
                    ScopeId::ROOT,
                );
                try_consume_context().unwrap()
            }
        };
        storage.counter.fetch_add(1, Ordering::SeqCst)
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
