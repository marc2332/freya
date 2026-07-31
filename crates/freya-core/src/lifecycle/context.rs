use std::{
    any::{
        Any,
        TypeId,
    },
    cell::RefCell,
    rc::Rc,
};

use rustc_hash::FxHashMap;

use crate::{
    current_context::CurrentContext,
    prelude::use_hook,
    scope_id::ScopeId,
};

/// Root contexts shared by all windows.
#[derive(Clone, Default)]
pub struct GlobalContexts(Rc<RefCell<FxHashMap<TypeId, Rc<dyn Any>>>>);

impl GlobalContexts {
    pub fn get_or_insert<T: Clone + 'static>(&self, value: T) -> T {
        let mut contexts = self.0.borrow_mut();
        if let Some(context) = contexts
            .get(&TypeId::of::<T>())
            .and_then(|context| context.downcast_ref::<T>())
        {
            return context.clone();
        }
        contexts.insert(TypeId::of::<T>(), Rc::new(value.clone()));
        value
    }

    pub fn get<T: Clone + 'static>(&self) -> Option<T> {
        self.0
            .borrow()
            .get(&TypeId::of::<T>())?
            .downcast_ref::<T>()
            .cloned()
    }
}

pub fn provide_context<T: Clone + 'static>(value: T) {
    provide_context_for_scope_id(value, None)
}

/// Store the given value in the root scope, shared across windows. If a value of the same
/// type already exists, that one is returned instead.
///
/// Only needed for specific cases like values consumed by queries or mutations, which run in
/// the root scope. Prefer [provide_context] otherwise.
pub fn provide_root_context<T: Clone + 'static>(value: T) -> T {
    let value = match try_consume_root_context::<GlobalContexts>() {
        Some(global_contexts) => global_contexts.get_or_insert(value),
        None => value,
    };

    provide_context_for_scope_id(value.clone(), Some(ScopeId::ROOT));

    value
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

/// Try to get a context value stored only in the current scope, without walking up to ancestors.
pub fn try_consume_own_context<T: Clone + 'static>() -> Option<T> {
    CurrentContext::with(|context| {
        let scopes_storages = context.scopes_storages.borrow();
        let scopes_storage = scopes_storages.get(&context.scope_id)?;
        let type_id = TypeId::of::<T>();
        scopes_storage
            .contexts
            .get(&type_id)?
            .downcast_ref::<T>()
            .cloned()
    })
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

        // Fall back to the shared contexts
        let root_scope_storage = scopes_storages.get(&ScopeId::ROOT)?;
        let global_contexts = root_scope_storage
            .contexts
            .get(&TypeId::of::<GlobalContexts>())?
            .downcast_ref::<GlobalContexts>()?;
        global_contexts.get::<T>()
    })
}

/// Store the given value in this component instance.
/// Any descendant component of this component calling [use_consume] or [consume_context] will have access to it.
pub fn use_provide_context<T: Clone + 'static>(init: impl FnOnce() -> T) -> T {
    use_hook(|| {
        let ctx = init();
        provide_context(ctx.clone());
        ctx
    })
}

/// Store the given value in the root scope, shared across windows. If a value of the same
/// type already exists, that one is returned and `init` is not called.
///
/// Only needed for specific cases like values consumed by queries or mutations, which run in
/// the root scope. Prefer [use_provide_context] otherwise.
pub fn use_provide_root_context<T: Clone + 'static>(init: impl FnOnce() -> T) -> T {
    use_hook(|| match try_consume_root_context::<T>() {
        Some(context) => context,
        None => provide_root_context(init()),
    })
}

/// Get access to a value stored in this component instance or some ancestor.
pub fn use_consume<T: Clone + 'static>() -> T {
    use_hook(|| consume_context())
}

/// Try to get access to a value stored in this component instance or some ancestor.
pub fn use_try_consume<T: Clone + 'static>() -> Option<T> {
    use_hook(|| try_consume_context())
}
