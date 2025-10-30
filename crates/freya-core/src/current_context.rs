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

    pub fn with<T>(with: impl FnOnce(&CurrentContext) -> T) -> T {
        CURRENT_CONTEXT.with(|context| with(context.borrow().as_ref().unwrap()))
    }
}

thread_local! {
    static CURRENT_CONTEXT: RefCell<Option<CurrentContext>> = const { RefCell::new(None) }
}
