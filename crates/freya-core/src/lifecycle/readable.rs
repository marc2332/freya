//! Type-erased readable state that hides generic type parameters.

use std::rc::Rc;

use crate::prelude::*;

/// A type-erased readable state that only exposes the value type `T`.
///
/// This hides the `Value` and `Channel` type parameters from `RadioSlice`,
/// allowing components to accept state without knowing the global state type.
///
/// # Example
///
/// ```rust, ignore
/// #[derive(PartialEq)]
/// struct Counter {
///     count: Readable<i32>,
/// }
///
/// impl Component for Counter {
///     fn render(&self) -> impl IntoElement {
///         format!("Count: {}", self.count.read())
///     }
/// }
///
/// fn app() -> impl IntoElement {
///     let local = use_state(|| 0);
///     let radio = use_radio(AppChannel::Count);
///     let slice = radio.slice_current(|s| &s.count);
///
///     rect()
///         .child(Counter { count: local.into_readable() })
///         .child(Counter { count: slice.into_readable() })
/// }
/// ```
pub struct Readable<T: 'static> {
    pub(crate) read_fn: Rc<dyn Fn() -> ReadableRef<T>>,
    pub(crate) peek_fn: Rc<dyn Fn() -> ReadableRef<T>>,
}

impl<T: 'static> Clone for Readable<T> {
    fn clone(&self) -> Self {
        Self {
            read_fn: self.read_fn.clone(),
            peek_fn: self.peek_fn.clone(),
        }
    }
}

impl<T: 'static> PartialEq for Readable<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> From<T> for Readable<T> {
    fn from(value: T) -> Self {
        Readable::from_value(value)
    }
}

impl<T: 'static> Readable<T> {
    /// Create from local `State<T>`.
    pub fn from_state(state: State<T>) -> Self {
        Self {
            read_fn: Rc::new(move || ReadableRef::Ref(state.read())),
            peek_fn: Rc::new(move || ReadableRef::Ref(state.peek())),
        }
    }

    /// Create from an owned value.
    pub fn from_value(value: T) -> Self {
        let value = Rc::new(value);
        Self {
            read_fn: Rc::new({
                let value = value.clone();
                move || ReadableRef::Borrowed(value.clone())
            }),
            peek_fn: Rc::new(move || ReadableRef::Borrowed(value.clone())),
        }
    }

    /// Create a new `Readable` with custom read and peek functions.
    pub fn new(
        read_fn: Box<dyn Fn() -> ReadableRef<T>>,
        peek_fn: Box<dyn Fn() -> ReadableRef<T>>,
    ) -> Self {
        Self {
            read_fn: Rc::from(read_fn),
            peek_fn: Rc::from(peek_fn),
        }
    }

    /// Read the value and subscribe to changes.
    pub fn read(&self) -> ReadableRef<T> {
        (self.read_fn)()
    }

    /// Read the value without subscribing.
    pub fn peek(&self) -> ReadableRef<T> {
        (self.peek_fn)()
    }
}

pub trait IntoReadable<T: 'static> {
    fn into_readable(self) -> Readable<T>;
}

impl<T: 'static> IntoReadable<T> for State<T> {
    fn into_readable(self) -> Readable<T> {
        Readable::from_state(self)
    }
}

impl<T> From<State<T>> for Readable<T> {
    fn from(value: State<T>) -> Self {
        value.into_readable()
    }
}
