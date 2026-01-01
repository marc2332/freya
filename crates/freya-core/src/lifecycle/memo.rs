use std::{
    mem::MaybeUninit,
    ops::Deref,
};

use crate::{
    prelude::{
        ReadRef,
        State,
        spawn,
        use_hook,
    },
    reactive_context::ReactiveContext,
};

/// Registers a callback that will run every time a [State] which was [.read()](State::read) inside, changes.
/// It also returning a type that will get cached after the callback runs, thus allowing this to be used as a way to cache expensive values.
/// ```rust
/// # use freya::prelude::*;
/// let state = use_state(|| 0);
///
/// let expensive_value = use_memo(|| {
///     // The moment `.read()` is called this side effect callback gets subscribed to it
///     let value = *state.read();
///     value * 2
/// });
/// ```
pub fn use_memo<T: 'static + PartialEq>(callback: impl FnMut() -> T + 'static) -> Memo<T> {
    use_hook(|| Memo::create(callback))
}

pub struct Memo<T> {
    state: State<T>,
}

impl<T> Clone for Memo<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Memo<T> {}

/// Allow calling the states as functions.
/// Limited to `Copy` values only.
impl<T: Copy + PartialEq + 'static> Deref for Memo<T> {
    type Target = dyn Fn() -> T;

    fn deref(&self) -> &Self::Target {
        unsafe { Memo::deref_impl(self) }
    }
}

impl<T: PartialEq> Memo<T> {
    /// Adapted from https://github.com/DioxusLabs/dioxus/blob/a4aef33369894cd6872283d6d7d265303ae63913/packages/signals/src/read.rs#L246
    /// SAFETY: You must call this function directly with `self` as the argument.
    /// This function relies on the size of the object you return from the deref
    /// being the same as the object you pass in
    #[doc(hidden)]
    unsafe fn deref_impl<'a>(memo: &Memo<T>) -> &'a dyn Fn() -> T
    where
        Self: Sized + 'a,
        T: Clone + 'static,
    {
        // https://github.com/dtolnay/case-studies/tree/master/callable-types

        // First we create a closure that captures something with the Same in memory layout as Self (MaybeUninit<Self>).
        let uninit_callable = MaybeUninit::<Self>::uninit();
        // Then move that value into the closure. We assume that the closure now has a in memory layout of Self.
        let uninit_closure = move || Memo::read(unsafe { &*uninit_callable.as_ptr() }).clone();

        // Check that the size of the closure is the same as the size of Self in case the compiler changed the layout of the closure.
        let size_of_closure = std::mem::size_of_val(&uninit_closure);
        assert_eq!(size_of_closure, std::mem::size_of::<Self>());

        // Then cast the lifetime of the closure to the lifetime of &self.
        fn cast_lifetime<'a, T>(_a: &T, b: &'a T) -> &'a T {
            b
        }
        let reference_to_closure = cast_lifetime(
            {
                // The real closure that we will never use.
                &uninit_closure
            },
            #[allow(clippy::missing_transmute_annotations)]
            // We transmute self into a reference to the closure. This is safe because we know that the closure has the same memory layout as Self so &Closure == &Self.
            unsafe {
                std::mem::transmute(memo)
            },
        );

        // Cast the closure to a trait object.
        reference_to_closure as &_
    }
}

impl<T: 'static + PartialEq> Memo<T> {
    pub fn create(mut callback: impl FnMut() -> T + 'static) -> Memo<T> {
        let (rx, rc) = ReactiveContext::new_for_task();
        let mut state = State::create(ReactiveContext::run(rc.clone(), &mut callback));
        spawn(async move {
            loop {
                rx.notified().await;
                state.set_if_modified(ReactiveContext::run(rc.clone(), &mut callback));
            }
        });
        Memo { state }
    }

    pub fn read(&self) -> ReadRef<'static, T> {
        self.state.read()
    }

    pub fn peek(&self) -> ReadRef<'static, T> {
        self.state.peek()
    }
}
