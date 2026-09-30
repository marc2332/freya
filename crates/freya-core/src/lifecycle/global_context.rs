use std::{
    any::{
        Any,
        TypeId,
    },
    cell::RefCell,
    rc::Rc,
};

use rustc_hash::FxHashMap;

/// Application contexts shared by windows and global tasks.
#[derive(Clone, Default)]
pub struct GlobalContexts(Rc<RefCell<FxHashMap<TypeId, Rc<dyn Any>>>>);

impl GlobalContexts {
    #[track_caller]
    pub fn get() -> GlobalContexts {
        Self::try_get().expect("Global contexts are unavailable outside Freya.")
    }

    pub fn try_get() -> Option<Self> {
        CURRENT_GLOBAL_CONTEXT.with_borrow(Clone::clone)
    }

    /// Get or initialize the application contexts on the current thread.
    pub fn register() -> Self {
        CURRENT_GLOBAL_CONTEXT
            .with_borrow_mut(|current| current.get_or_insert_with(Self::default).clone())
    }

    /// Unregister this application's contexts from the current thread.
    pub fn unregister(&self) {
        let _removed_context = CURRENT_GLOBAL_CONTEXT.with_borrow_mut(|current| {
            if current
                .as_ref()
                .is_some_and(|context| Rc::ptr_eq(&context.0, &self.0))
            {
                current.take()
            } else {
                None
            }
        });
    }

    pub fn insert_context<T: Clone + 'static>(&self, value: T) -> T {
        let mut contexts = self.0.borrow_mut();
        contexts.insert(TypeId::of::<T>(), Rc::new(value.clone()));
        value
    }

    #[track_caller]
    pub fn get_context<T: Clone + 'static>(&self) -> T {
        match self.try_get_context() {
            Some(context) => context,
            None => panic!(
                "Global context <{}> was not found.",
                std::any::type_name::<T>()
            ),
        }
    }

    pub fn try_get_context<T: Clone + 'static>(&self) -> Option<T> {
        self.0
            .borrow()
            .get(&TypeId::of::<T>())?
            .downcast_ref::<T>()
            .cloned()
    }

    pub fn get_context_or_insert<T: Clone + 'static>(&self, insert: impl FnOnce() -> T) -> T {
        let mut contexts = self.0.borrow_mut();
        if let Some(context) = contexts
            .get(&TypeId::of::<T>())
            .and_then(|context| context.downcast_ref::<T>())
        {
            return context.clone();
        }
        let value = insert();
        contexts.insert(TypeId::of::<T>(), Rc::new(value.clone()));
        value
    }
}

thread_local! {
    static CURRENT_GLOBAL_CONTEXT: RefCell<Option<GlobalContexts>> = const { RefCell::new(None) };
}
