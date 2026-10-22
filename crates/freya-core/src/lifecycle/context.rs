//! # Context
//!
//! Context passes a value down to descendant components without passing it through
//! every component's props. Values are identified by type and must implement `Clone + 'static`.
//!
//! ## Component context
//!
//! [`use_provide_context`] stores a value in the component and makes it available to
//! descendants. [`use_consume`] looks in the current component, then its parents,
//! using the first value of that type it finds.
//! The stored value is dropped when the component is dropped.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//!
//! #[derive(Clone)]
//! struct Greeting(&'static str);
//!
//! fn app() -> impl IntoElement {
//!     use_provide_context(|| Greeting("Hello"));
//!     rect().child(Message)
//! }
//!
//! #[derive(PartialEq)]
//! struct Message;
//!
//! impl Component for Message {
//!     fn render(&self) -> impl IntoElement {
//!         let greeting = use_consume::<Greeting>();
//!         label().text(greeting.0)
//!     }
//! }
//! ```
//!
//! ## Root context
//!
//! [`use_provide_root_context`] stores a value in the window's root, reusing any
//! existing value of that type. It stays there until replaced or the root is dropped,
//! even if the component that provided it is dropped.
//!
//! [`consume_root_context`] reads only from the root, not from the current component
//! or its parents. You can also provide root values with `WindowConfig::with_root_context`.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//!
//! #[derive(Clone)]
//! struct WindowName(&'static str);
//!
//! fn app() -> impl IntoElement {
//!     use_provide_root_context(|| WindowName("Editor"));
//!     let name = consume_root_context::<WindowName>();
//!     label().text(name.0)
//! }
//! ```

use std::{
    any::TypeId,
    rc::Rc,
};

use crate::{
    current_context::CurrentContext,
    prelude::use_hook,
    scope_id::ScopeId,
};

/// Store a value in the current component, replacing any value of the same type.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn render() {
/// provide_context(String::from("Hello"));
/// assert_eq!(consume_context::<String>(), "Hello");
/// # }
/// ```
pub fn provide_context<T: Clone + 'static>(value: T) {
    provide_context_for_scope_id(value, None)
}

/// Store a value in the window's root, replacing any value of the same type.
/// To share across windows, use [`GlobalContexts`](crate::prelude::GlobalContexts).
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn render() {
/// provide_root_context(String::from("Editor"));
/// assert_eq!(consume_root_context::<String>(), "Editor");
/// # }
/// ```
pub fn provide_root_context<T: Clone + 'static>(value: T) -> T {
    provide_context_for_scope_id(value.clone(), Some(ScopeId::ROOT));
    value
}

/// Store a value in the given scope, replacing any value of the same type. `None` uses the current scope.
/// The target scope must exist in the current window.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn render() {
/// provide_context_for_scope_id(String::from("Editor"), ScopeId::ROOT);
/// # }
/// ```
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

/// Get a value from the current component only, or `None` if not found.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn render() {
/// let local_name = try_consume_own_context::<String>();
/// # }
/// ```
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

/// Get a value from the window's root only, or `None` if not found.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn render() {
/// let window_name = try_consume_root_context::<String>();
/// # }
/// ```
pub fn try_consume_root_context<T: Clone + 'static>() -> Option<T> {
    try_consume_context_from_scope_id(Some(ScopeId::ROOT))
}

/// Look for a value in the current component, then its parents. Returns `None` if not found.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn render() {
/// let name = try_consume_context::<String>().unwrap_or_default();
/// # }
/// ```
pub fn try_consume_context<T: Clone + 'static>() -> Option<T> {
    try_consume_context_from_scope_id(None)
}

/// Look for a value in the current component, then its parents. Panics if not found.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn render() {
/// provide_context(String::from("Hello"));
/// let greeting = consume_context::<String>();
/// # }
/// ```
#[track_caller]
pub fn consume_context<T: Clone + 'static>() -> T {
    match try_consume_context_from_scope_id(None) {
        Some(context) => context,
        None => panic!("Context <{}> was not found.", std::any::type_name::<T>()),
    }
}

/// Get a value from the window's root only. Panics if not found.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn render() {
/// provide_root_context(String::from("Editor"));
/// let window_name = consume_root_context::<String>();
/// # }
/// ```
#[track_caller]
pub fn consume_root_context<T: Clone + 'static>() -> T {
    match try_consume_root_context() {
        Some(context) => context,
        None => panic!(
            "Root context <{}> was not found.",
            std::any::type_name::<T>()
        ),
    }
}

/// Look for a value in the given scope, then its parents. Returns `None` if not found.
/// Pass `None` to start at the current scope.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn render() {
/// let window_name = try_consume_context_from_scope_id::<String>(Some(ScopeId::ROOT));
/// # }
/// ```
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

/// Create a value once in this component and make it available to descendants.
/// Returns the same value on later renders.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn render() {
/// let greeting = use_provide_context(|| String::from("Hello"));
/// # }
/// ```
pub fn use_provide_context<T: Clone + 'static>(init: impl FnOnce() -> T) -> T {
    use_hook(|| {
        let ctx = init();
        provide_context(ctx.clone());
        ctx
    })
}

/// Reuse a value from the window's root, or call `init` to create it if missing.
/// Returns the same value on later renders. The value is stored in the root, not this component.
/// To share across windows, use [`GlobalContexts`](crate::prelude::GlobalContexts).
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn render() {
/// let window_name = use_provide_root_context(|| String::from("Editor"));
/// # }
/// ```
pub fn use_provide_root_context<T: Clone + 'static>(init: impl FnOnce() -> T) -> T {
    use_hook(|| match try_consume_root_context::<T>() {
        Some(context) => context,
        None => provide_root_context(init()),
    })
}

/// Look for a value in this component, then its parents. Panics if not found.
/// Returns the same value on later renders.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn render() {
/// use_provide_context(|| String::from("Hello"));
/// let greeting = use_consume::<String>();
/// # }
/// ```
#[track_caller]
pub fn use_consume<T: Clone + 'static>() -> T {
    match use_hook(|| try_consume_context()) {
        Some(context) => context,
        None => panic!("Context <{}> was not found.", std::any::type_name::<T>()),
    }
}

/// Look for a value in this component, then its parents. Returns `None` if not found.
/// Returns the same result on later renders.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn render() {
/// let greeting = use_try_consume::<String>().unwrap_or_default();
/// # }
/// ```
pub fn use_try_consume<T: Clone + 'static>() -> Option<T> {
    use_hook(|| try_consume_context())
}
