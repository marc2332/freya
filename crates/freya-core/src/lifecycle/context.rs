use std::{
    any::TypeId,
    rc::Rc,
};

use crate::{
    current_context::CurrentContext,
    prelude::use_hook,
    scope_id::ScopeId,
};

pub fn provide_context<T: Clone + 'static>(value: T) {
    provide_context_for_scope_id(value, None)
}

pub fn provide_context_for_scope_id<T: Clone + 'static>(
    value: T,
    scope_id: impl Into<Option<ScopeId>>,
) {
    CurrentContext::with(|context| {
        let mut scopes_storages = context.scopes_storages.borrow_mut();
        let scopes_storage = scopes_storages
            .get_mut(&scope_id.into().unwrap_or(context.scope_id))
            .unwrap();
        let type_id = TypeId::of::<T>();
        scopes_storage.contexts.insert(type_id, Rc::new(value));
    })
}

pub fn try_consume_context<T: Clone + 'static>() -> Option<T> {
    try_consume_context_from_scope_id(None)
}

pub fn try_consume_root_context<T: Clone + 'static>() -> Option<T> {
    try_consume_context_from_scope_id(Some(ScopeId::ROOT))
}

pub fn consume_context<T: Clone + 'static>() -> T {
    try_consume_context_from_scope_id(None)
        .unwrap_or_else(|| panic!("Context <{}> was not found.", std::any::type_name::<T>()))
}

pub fn consume_root_context<T: Clone + 'static>() -> T {
    try_consume_context_from_scope_id(Some(ScopeId::ROOT)).unwrap_or_else(|| {
        panic!(
            "Root context <{}> was not found.",
            std::any::type_name::<T>()
        )
    })
}

pub fn try_consume_context_from_scope_id<T: Clone + 'static>(
    scope_id: Option<ScopeId>,
) -> Option<T> {
    CurrentContext::with(|context| {
        let scopes_storages = context.scopes_storages.borrow_mut();

        let mut ladder = vec![scope_id.unwrap_or(context.scope_id)];

        let type_id = TypeId::of::<T>();

        while let Some(scope_id) = ladder.pop() {
            let scopes_storage = scopes_storages.get(&scope_id)?;

            if let Some(context) = scopes_storage.contexts.get(&type_id) {
                return context.downcast_ref::<T>().cloned();
            } else if let Some(parent_scope_id) = scopes_storage.parent_id {
                ladder.push(parent_scope_id);
            }
        }

        None
    })
}

pub fn use_provide_context<T: Clone + 'static>(init: impl FnOnce() -> T) -> T {
    use_hook(|| {
        let ctx = init();
        provide_context(ctx.clone());
        ctx
    })
}

pub fn use_consume<T: Clone + 'static>() -> T {
    use_hook(|| consume_context())
}

pub fn use_try_consume<T: Clone + 'static>() -> Option<T> {
    use_hook(|| try_consume_context())
}
