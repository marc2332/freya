use std::rc::Rc;

use crate::{
    current_context::CurrentContext,
    runner::Message,
    scope_id::ScopeId,
};

pub fn use_hook<T: Clone + 'static>(init: impl FnOnce() -> T) -> T {
    if let Some(value) = CurrentContext::with(|context| {
        let mut scopes_storages = context.scopes_storages.borrow_mut();
        let scopes_storage = scopes_storages.get_mut(&context.scope_id).unwrap();
        if let Some(value) = scopes_storage
            .values
            .get(scopes_storage.current_value)
            .cloned()
        {
            scopes_storage.current_value += 1;
            Some(value.downcast_ref::<T>().cloned().unwrap())
        } else if scopes_storage.current_run > 0 {
            panic!("Hooks cannot be called conditionally.")
        } else {
            None
        }
    }) {
        value
    } else {
        let value = init();
        CurrentContext::with(|context| {
            let mut scopes_storages = context.scopes_storages.borrow_mut();
            let scopes_storage = scopes_storages.get_mut(&context.scope_id).unwrap();
            scopes_storage.values.push(Rc::new(value.clone()));
            scopes_storage.current_value += 1;
            value
        })
    }
}

struct DropInner(Option<Box<dyn FnOnce()>>);

impl std::ops::Drop for DropInner {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}

pub fn use_drop(drop: impl FnOnce() + 'static) {
    use_hook(|| Rc::new(DropInner(Some(Box::new(drop)))));
}

pub fn current_scope_id() -> ScopeId {
    CurrentContext::with(|context| context.scope_id)
}

pub fn mark_scope_as_dirty(scope_id: ScopeId) {
    CurrentContext::with(|context| {
        context
            .sender
            .unbounded_send(Message::MarkScopeAsDirty(scope_id))
            .unwrap();
    })
}
