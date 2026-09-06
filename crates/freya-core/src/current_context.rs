use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::AtomicU64,
};

use rustc_hash::FxHashMap;

use crate::{
    prelude::{
        Task,
        TaskId,
    },
    reactive_context::ReactiveContext,
    runner::Message,
    scope::ScopeStorage,
    scope_id::ScopeId,
};

// TODO: rendering flag.
pub struct CurrentContext {
    pub scope_id: ScopeId,
    pub scopes_storages: Rc<RefCell<FxHashMap<ScopeId, ScopeStorage>>>,

    pub tasks: Rc<RefCell<FxHashMap<TaskId, Rc<RefCell<Task>>>>>,
    pub task_id_counter: Rc<AtomicU64>,
    pub sender: futures_channel::mpsc::UnboundedSender<Message>,
}

impl CurrentContext {
    pub fn run_with_reactive<T>(new_context: Self, run: impl FnOnce() -> T) -> T {
        let reactive_context = CURRENT_CONTEXT.with_borrow_mut(|context| {
            let reactive_context = {
                let scope_storages = new_context.scopes_storages.borrow();
                let scope_storage = scope_storages.get(&new_context.scope_id).unwrap();
                scope_storage.reactive_context.clone()
            };
            context.replace(new_context);
            reactive_context
        });
        let res = ReactiveContext::run(reactive_context, run);
        CURRENT_CONTEXT.with_borrow_mut(|context| context.take());
        res
    }

    pub fn run<T>(new_context: Self, run: impl FnOnce() -> T) -> T {
        CURRENT_CONTEXT.with_borrow_mut(|context| {
            context.replace(new_context);
        });
        let res = run();
        CURRENT_CONTEXT.with_borrow_mut(|context| context.take());
        res
    }

    /// Run a closure using `scope_id` as the current scope, restoring the previous one afterwards.
    ///
    /// The closure still runs when there is no current context, just without any scope change.
    pub(crate) fn run_in_scope<T>(scope_id: ScopeId, run: impl FnOnce() -> T) -> T {
        let previous_scope_id = CURRENT_CONTEXT.with_borrow_mut(|context| {
            context
                .as_mut()
                .map(|context| std::mem::replace(&mut context.scope_id, scope_id))
        });

        let res = run();

        CURRENT_CONTEXT.with_borrow_mut(|context| {
            if let Some(context) = context.as_mut()
                && let Some(previous_scope_id) = previous_scope_id
            {
                context.scope_id = previous_scope_id;
            }
        });

        res
    }

    pub fn with<T>(with: impl FnOnce(&CurrentContext) -> T) -> T {
        CURRENT_CONTEXT.with(|context| with(context.borrow().as_ref().expect("Your trying to access Freya's current context outside of it, you might be in a separate thread or async task that is not integrated with Freya.")))
    }

    pub fn try_with<T>(with: impl FnOnce(&CurrentContext) -> T) -> Option<T> {
        CURRENT_CONTEXT
            .try_with(|context| {
                if let Ok(context) = context.try_borrow()
                    && let Some(context) = context.as_ref()
                {
                    Some(with(context))
                } else {
                    None
                }
            })
            .ok()
            .flatten()
    }
}

thread_local! {
    static CURRENT_CONTEXT: RefCell<Option<CurrentContext>> = const { RefCell::new(None) }
}
