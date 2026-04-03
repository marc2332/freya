use crate::prelude::*;

/// A shared interface for types that provide reactive write access to a value of type `T`.
///
/// Implementors must provide [`WritableUtils::write_state`] and [`WritableUtils::peek_state`].
/// All convenience methods ([`WritableUtils::set`], [`WritableUtils::set_if_modified`], etc.) are provided as defaults.
pub trait WritableUtils<T: 'static> {
    /// Get a mutable reference to the value, notifying subscribers.
    fn write_state(&mut self) -> WriteRef<'static, T>;

    /// Read the current value without subscribing to changes.
    fn peek_state(&self) -> ReadRef<'static, T>;

    /// Replace the current value and notify subscribers.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// let mut status = use_state(|| "idle");
    ///
    /// status.set("loading");
    /// status.set("complete");
    /// ```
    fn set(&mut self, value: T) {
        *self.write_state() = value;
    }

    /// Replace the value only if it differs from the current one.
    ///
    /// This prevents unnecessary re-renders when setting the same value repeatedly.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// let mut count = use_state(|| 0);
    ///
    /// // This will update and notify subscribers
    /// count.set_if_modified(5);
    ///
    /// // This will do nothing (value is already 5)
    /// count.set_if_modified(5);
    /// ```
    fn set_if_modified(&mut self, value: T)
    where
        T: PartialEq,
    {
        if *self.peek_state() != value {
            self.set(value);
        }
    }

    /// Replace the value if modified, then run a callback.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// let mut score = use_state(|| 0);
    ///
    /// score.set_if_modified_and_then(100, || {
    ///     println!("High score achieved!");
    /// });
    /// ```
    fn set_if_modified_and_then(&mut self, value: T, then: impl FnOnce())
    where
        T: PartialEq,
    {
        if *self.peek_state() != value {
            self.set(value);
            then();
        }
    }

    /// Modify the value via a closure with a single notification.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// let mut counter = use_state(|| 0);
    ///
    /// counter.with_mut(|mut value| {
    ///     *value += 1;
    ///     *value *= 2;
    /// });
    /// ```
    fn with_mut(&mut self, with: impl FnOnce(WriteRef<'static, T>)) {
        with(self.write_state());
    }
}

impl<T: 'static> WritableUtils<T> for State<T> {
    fn write_state(&mut self) -> WriteRef<'static, T> {
        self.write()
    }

    fn peek_state(&self) -> ReadRef<'static, T> {
        self.peek()
    }
}

impl<T: 'static> WritableUtils<T> for Writable<T> {
    fn write_state(&mut self) -> WriteRef<'static, T> {
        self.write()
    }

    fn peek_state(&self) -> ReadRef<'static, T> {
        self.peek()
    }
}
