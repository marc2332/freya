//! # Global contexts
//!
//! [`GlobalContexts`] shares values across windows and global tasks, separately from
//! [component and root context](crate::lifecycle::context). Components cannot provide
//! a different global value just for their descendants.
//!
//! ## Providing and consuming
//!
//! Provide values with [`LaunchConfig::with_global_context`](freya::prelude::LaunchConfig::with_global_context), then access them through
//! [`GlobalContexts::get`] in components, event handlers, or Freya tasks.
//! Values must implement `Clone + 'static` and are cloned when read.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//!
//! #[derive(Clone)]
//! struct AppName(&'static str);
//!
//! fn main() {
//!     launch(
//!         LaunchConfig::new()
//!             .with_global_context(AppName("Editor"))
//!             .with_window(WindowConfig::new(app)),
//!     )
//! }
//!
//! fn app() -> impl IntoElement {
//!     let name = GlobalContexts::get().get_context::<AppName>();
//!     label().text(name.0)
//! }
//! ```
//!
//! ## Initializing on demand
//!
//! [`GlobalContexts::get_context_or_insert`] creates a value only if missing.
//! [`GlobalContexts::try_get_context`] returns `None` if not found, while
//! [`GlobalContexts::get_context`] panics.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//!
//! fn app() -> impl IntoElement {
//!     let globals = GlobalContexts::get();
//!     let name = globals.get_context_or_insert(|| String::from("Editor"));
//!     label().text(name)
//! }
//! ```

use std::{
    any::{
        Any,
        TypeId,
    },
    cell::RefCell,
    rc::Rc,
};

use rustc_hash::FxHashMap;

/// Stores values by type, shared across windows and global tasks.
/// See the [module guide](crate::lifecycle::global_context) for setup and usage.
#[derive(Clone, Default)]
pub struct GlobalContexts(Rc<RefCell<FxHashMap<TypeId, Rc<dyn Any>>>>);

impl GlobalContexts {
    /// Get the current thread's global contexts. Panics if none are registered.
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// let globals = GlobalContexts::get();
    /// ```
    #[track_caller]
    pub fn get() -> GlobalContexts {
        Self::try_get().expect("Global contexts are unavailable outside Freya.")
    }

    /// Get the current thread's global contexts, or `None` if none are registered.
    ///
    /// ```rust
    /// # use freya::prelude::*;
    /// let globals = GlobalContexts::try_get();
    /// ```
    pub fn try_get() -> Option<Self> {
        CURRENT_GLOBAL_CONTEXT.with_borrow(Clone::clone)
    }

    /// Set up global contexts on the current thread, or reuse the existing ones. Freya does this automatically.
    ///
    /// ```rust
    /// # use freya::prelude::*;
    /// let globals = GlobalContexts::register();
    /// globals.unregister();
    /// ```
    pub fn register() -> Self {
        CURRENT_GLOBAL_CONTEXT
            .with_borrow_mut(|current| current.get_or_insert_with(Self::default).clone())
    }

    /// Remove these global contexts from the current thread if registered, without clearing their values.
    /// Freya does this automatically when the application exits.
    ///
    /// ```rust
    /// # use freya::prelude::*;
    /// let globals = GlobalContexts::register();
    /// globals.unregister();
    /// assert!(GlobalContexts::try_get().is_none());
    /// ```
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

    /// Store and return a value, replacing any value of the same type.
    ///
    /// ```rust
    /// # use freya::prelude::*;
    /// let globals = GlobalContexts::default();
    /// globals.insert_context(String::from("Editor"));
    /// ```
    pub fn insert_context<T: Clone + 'static>(&self, value: T) -> T {
        let mut contexts = self.0.borrow_mut();
        contexts.insert(TypeId::of::<T>(), Rc::new(value.clone()));
        value
    }

    /// Get a clone of the stored value. Panics if not found.
    ///
    /// ```rust
    /// # use freya::prelude::*;
    /// let globals = GlobalContexts::default();
    /// globals.insert_context(String::from("Editor"));
    /// assert_eq!(globals.get_context::<String>(), "Editor");
    /// ```
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

    /// Get a clone of the stored value, or `None` if not found.
    ///
    /// ```rust
    /// # use freya::prelude::*;
    /// let globals = GlobalContexts::default();
    /// assert!(globals.try_get_context::<String>().is_none());
    /// ```
    pub fn try_get_context<T: Clone + 'static>(&self) -> Option<T> {
        self.0
            .borrow()
            .get(&TypeId::of::<T>())?
            .downcast_ref::<T>()
            .cloned()
    }

    /// Get a clone of the stored value, or call `insert` to create and store it if missing.
    ///
    /// ```rust
    /// # use freya::prelude::*;
    /// let globals = GlobalContexts::default();
    /// let name = globals.get_context_or_insert(|| String::from("Editor"));
    /// assert_eq!(name, "Editor");
    /// ```
    pub fn get_context_or_insert<T: Clone + 'static>(&self, insert: impl FnOnce() -> T) -> T {
        if let Some(context) = self.try_get_context() {
            return context;
        }

        let value = insert();
        self.insert_context(value)
    }
}

thread_local! {
    static CURRENT_GLOBAL_CONTEXT: RefCell<Option<GlobalContexts>> = const { RefCell::new(None) };
}
