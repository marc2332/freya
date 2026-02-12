//! Type-erased writable state that hides generic type parameters.

use std::rc::Rc;

use crate::prelude::*;

/// A type-erased writable state that only exposes the value type `T`.
///
/// This abstraction allows components to accept state from any source without knowing
/// whether it comes from local state ([`use_state`]) or
/// global state (Freya Radio). It hides the implementation details, providing a
/// uniform interface for reading and writing values.
///
/// # Sources
///
/// `Writable` can be created from:
/// - [`State<T>`] via [`From`] or [`IntoWritable`]
/// - `RadioSliceMut` via `IntoWritable`
///
/// # Example
///
/// ```rust, ignore
/// #[derive(PartialEq)]
/// struct NameInput {
///     name: Writable<String>,
/// }
///
/// impl Component for NameInput {
///     fn render(&self) -> impl IntoElement {
///         // The component doesn't care if this is local or global state
///         Input::new(self.name.clone())
///     }
/// }
///
/// fn app() -> impl IntoElement {
///     let local = use_state(|| "Alice".to_string());
///     let radio = use_radio(AppChannel::Name);
///     let slice = radio.slice_mut_current(|s| &mut s.name);
///
///     rect()
///         // Pass local state
///         .child(NameInput { name: local.into_writable() })
///         // Pass global radio slice
///         .child(NameInput { name: slice.into_writable() })
/// }
/// ```
pub struct Writable<T: 'static> {
    pub(crate) peek_fn: Rc<dyn Fn() -> ReadRef<'static, T>>,
    pub(crate) write_fn: Rc<dyn Fn() -> WriteRef<'static, T>>,
    pub(crate) subscribe_fn: Rc<dyn Fn()>,
    pub(crate) notify_fn: Rc<dyn Fn()>,
}

impl<T: 'static> PartialEq for Writable<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T: 'static> Clone for Writable<T> {
    fn clone(&self) -> Self {
        Self {
            peek_fn: self.peek_fn.clone(),
            write_fn: self.write_fn.clone(),
            subscribe_fn: self.subscribe_fn.clone(),
            notify_fn: self.notify_fn.clone(),
        }
    }
}

impl<T: 'static> Writable<T> {
    /// Create from local `State<T>`.
    pub fn from_state(state: State<T>) -> Self {
        Self {
            peek_fn: Rc::new(move || state.peek()),
            write_fn: Rc::new(move || state.write_unchecked()),
            subscribe_fn: Rc::new(move || state.subscribe()),
            notify_fn: Rc::new(move || state.notify()),
        }
    }

    /// Create a new `Writable` with custom peek, write, subscribe, and notify functions.
    pub fn new(
        peek_fn: Box<dyn Fn() -> ReadRef<'static, T>>,
        write_fn: Box<dyn Fn() -> WriteRef<'static, T>>,
        subscribe_fn: Box<dyn Fn()>,
        notify_fn: Box<dyn Fn()>,
    ) -> Self {
        Self {
            peek_fn: Rc::from(peek_fn),
            write_fn: Rc::from(write_fn),
            subscribe_fn: Rc::from(subscribe_fn),
            notify_fn: Rc::from(notify_fn),
        }
    }

    /// Read the value and subscribe to changes.
    pub fn read(&self) -> ReadRef<'static, T> {
        self.subscribe();
        self.peek()
    }

    /// Read the value without subscribing.
    pub fn peek(&self) -> ReadRef<'static, T> {
        (self.peek_fn)()
    }

    /// Write the value and notify subscribers.
    pub fn write(&mut self) -> WriteRef<'static, T> {
        self.notify();
        (self.write_fn)()
    }

    pub fn write_if(&mut self, with: impl FnOnce(WriteRef<'static, T>) -> bool) {
        let changed = with(self.write());
        if changed {
            self.notify();
        }
    }

    /// Subscribe to changes.
    fn subscribe(&self) {
        (self.subscribe_fn)()
    }

    /// Notify subscribers.
    fn notify(&self) {
        (self.notify_fn)()
    }
}

pub trait IntoWritable<T: 'static> {
    fn into_writable(self) -> Writable<T>;
}

impl<T: 'static> IntoWritable<T> for State<T> {
    fn into_writable(self) -> Writable<T> {
        Writable::from_state(self)
    }
}

impl<T> From<State<T>> for Writable<T> {
    fn from(value: State<T>) -> Self {
        value.into_writable()
    }
}

impl<T> From<Writable<T>> for Readable<T> {
    fn from(value: Writable<T>) -> Self {
        Readable {
            read_fn: Rc::new({
                let value = value.clone();
                move || {
                    value.subscribe();
                    ReadableRef::Ref((value.peek_fn)())
                }
            }),
            peek_fn: Rc::new(move || ReadableRef::Ref((value.peek_fn)())),
        }
    }
}
