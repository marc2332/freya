use crate::prelude::*;

/// A readable value whose updates go through a setter callback.
///
/// ```rust
/// use freya::prelude::*;
///
/// fn app() -> impl IntoElement {
///     let mut value = use_state(String::new);
///     let value = WritableSetter::new(value, move |text| value.set(text));
///     Input::new(value)
/// }
/// ```
pub struct WritableSetter<T: 'static> {
    value: Readable<T>,
    on_change: EventHandler<T>,
}

impl<T: 'static> Clone for WritableSetter<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            on_change: self.on_change.clone(),
        }
    }
}

impl<T: 'static> PartialEq for WritableSetter<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.on_change == other.on_change
    }
}

impl<T: 'static> WritableSetter<T> {
    /// The callback may transform or reject updates to the readable value.
    pub fn new(value: impl IntoReadable<T>, on_change: impl Into<EventHandler<T>>) -> Self {
        Self {
            value: value.into_readable(),
            on_change: on_change.into(),
        }
    }

    /// Read the value and subscribe to changes.
    pub fn read(&self) -> ReadableRef<T> {
        self.value.read()
    }

    /// Read the value without subscribing to changes.
    pub fn peek(&self) -> ReadableRef<T> {
        self.value.peek()
    }

    /// Request an update through the setter callback.
    pub fn set(&mut self, value: T) {
        self.on_change.call(value);
    }

    /// Request an update only if the value differs from the current one.
    pub fn set_if_modified(&mut self, value: T)
    where
        T: PartialEq,
    {
        if *self.peek() != value {
            self.set(value);
        }
    }
}

pub trait IntoWritableSetter<T: 'static> {
    fn into_writable_setter(self) -> WritableSetter<T>;
}

impl<T: 'static, Value: Into<WritableSetter<T>>> IntoWritableSetter<T> for Value {
    fn into_writable_setter(self) -> WritableSetter<T> {
        self.into()
    }
}

impl<T: 'static> From<State<T>> for WritableSetter<T> {
    fn from(value: State<T>) -> Self {
        value.into_writable().into()
    }
}

impl<T: 'static> From<Writable<T>> for WritableSetter<T> {
    fn from(mut value: Writable<T>) -> Self {
        Self::new(Readable::from(value.clone()), move |updated| {
            *value.write() = updated;
        })
    }
}

impl<T: 'static> IntoReadable<T> for WritableSetter<T> {
    fn into_readable(self) -> Readable<T> {
        self.value
    }
}

impl<T: 'static> From<WritableSetter<T>> for Readable<T> {
    fn from(value: WritableSetter<T>) -> Self {
        value.into_readable()
    }
}
