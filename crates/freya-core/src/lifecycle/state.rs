use std::{
    cell::RefCell,
    mem::MaybeUninit,
    ops::Deref,
    rc::Rc,
};

use generational_box::{
    AnyStorage,
    GenerationalBox,
    UnsyncStorage,
};
use rustc_hash::FxHashSet;

use crate::{
    current_context::CurrentContext,
    prelude::use_hook,
    reactive_context::ReactiveContext,
    scope_id::ScopeId,
};

pub trait MutView<'a, T: 'static> {
    fn read(&mut self) -> ReadRef<'a, T>;

    fn peek(&mut self) -> ReadRef<'a, T>;

    fn write(&mut self) -> WriteRef<'a, T>;

    fn write_if(&mut self, with: impl FnOnce(WriteRef<'a, T>) -> bool);
}

impl<T: 'static> MutView<'static, T> for State<T> {
    fn read(&mut self) -> ReadRef<'static, T> {
        if let Some(mut rc) = ReactiveContext::try_current() {
            rc.subscribe(&self.subscribers.read());
        }
        self.key.read()
    }

    fn peek(&mut self) -> ReadRef<'static, T> {
        self.key.read()
    }

    fn write(&mut self) -> WriteRef<'static, T> {
        self.subscribers.write().borrow_mut().retain(|s| s.notify());
        self.key.write()
    }

    fn write_if(&mut self, with: impl FnOnce(WriteRef<'static, T>) -> bool) {
        let chnaged = with(self.key.write());
        if chnaged {
            self.subscribers.write().borrow_mut().retain(|s| s.notify());
        }
    }
}

pub struct State<T> {
    key: GenerationalBox<T>,
    subscribers: GenerationalBox<Rc<RefCell<FxHashSet<ReactiveContext>>>>,
}

impl<T: 'static> PartialEq for State<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key.ptr_eq(&other.key)
    }
}

impl<T: 'static> Eq for State<T> {}

/// Allow calling the states as functions.
/// Limited to `Copy` values only.
impl<T: Copy + 'static> Deref for State<T> {
    type Target = dyn Fn() -> T;

    fn deref(&self) -> &Self::Target {
        unsafe { State::deref_impl(self) }
    }
}

impl<T> State<T> {
    /// Adapted from https://github.com/DioxusLabs/dioxus/blob/a4aef33369894cd6872283d6d7d265303ae63913/packages/signals/src/read.rs#L246
    /// SAFETY: You must call this function directly with `self` as the argument.
    /// This function relies on the size of the object you return from the deref
    /// being the same as the object you pass in
    #[doc(hidden)]
    unsafe fn deref_impl<'a>(state: &State<T>) -> &'a dyn Fn() -> T
    where
        Self: Sized + 'a,
        T: Clone + 'static,
    {
        // https://github.com/dtolnay/case-studies/tree/master/callable-types

        // First we create a closure that captures something with the Same in memory layout as Self (MaybeUninit<Self>).
        let uninit_callable = MaybeUninit::<Self>::uninit();
        // Then move that value into the closure. We assume that the closure now has a in memory layout of Self.
        let uninit_closure = move || Self::read(unsafe { &*uninit_callable.as_ptr() }).clone();

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
                std::mem::transmute(state)
            },
        );

        // Cast the closure to a trait object.
        reference_to_closure as &_
    }
}

impl<T: std::ops::Not<Output = T> + Clone + 'static> State<T> {
    pub fn toggled(&mut self) -> T {
        let value = self.read().clone();
        let neg_value = !value;
        self.set(neg_value.clone());
        neg_value
    }

    pub fn toggle(&mut self) {
        self.toggled();
    }
}

type ReadStateFunc<T> = Rc<dyn Fn() -> ReadRef<'static, T>>;

/// Given a type `T` you may pass an owned value, a `State<T>` or a function returning a `ReadRef<T>`
#[derive(Clone)]
pub enum ReadState<T: 'static> {
    State(State<T>),
    Func(ReadStateFunc<T>),
    Owned(T),
}

impl<T> From<T> for ReadState<T> {
    fn from(value: T) -> Self {
        ReadState::Owned(value)
    }
}

impl<T> From<State<T>> for ReadState<T> {
    fn from(value: State<T>) -> Self {
        ReadState::State(value)
    }
}

impl<T: PartialEq> PartialEq for ReadState<T> {
    fn eq(&self, other: &ReadState<T>) -> bool {
        match (self, other) {
            (Self::State(a), Self::State(b)) => a == b,
            (Self::Func(a), Self::Func(b)) => Rc::ptr_eq(a, b),
            (Self::Owned(a), Self::Owned(b)) => a == b,
            _ => false,
        }
    }
}
impl<T: 'static> ReadState<T> {
    pub fn read(&'_ self) -> ReadStateCow<'_, T> {
        match self {
            Self::Func(f) => ReadStateCow::Ref(f()),
            Self::State(s) => ReadStateCow::Ref(s.read()),
            Self::Owned(o) => ReadStateCow::Borrowed(o),
        }
    }
}

pub enum ReadStateCow<'a, T: 'static> {
    Ref(ReadRef<'static, T>),
    Borrowed(&'a T),
}

impl<'a, T> Deref for ReadStateCow<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Ref(r) => r.deref(),
            Self::Borrowed(b) => b,
        }
    }
}

pub type ReadRef<'a, T> =
    <generational_box::UnsyncStorage as generational_box::AnyStorage>::Ref<'a, T>;

pub type WriteRef<'a, T> =
    <generational_box::UnsyncStorage as generational_box::AnyStorage>::Mut<'a, T>;

impl<T> State<T> {
    pub fn read(&self) -> ReadRef<'static, T> {
        if let Some(mut rc) = ReactiveContext::try_current() {
            rc.subscribe(&self.subscribers.read());
        }
        self.key.read()
    }

    pub fn peek(&self) -> ReadRef<'static, T> {
        self.key.read()
    }

    pub fn write(&mut self) -> WriteRef<'static, T> {
        self.subscribers.write().borrow_mut().retain(|s| s.notify());
        self.key.write()
    }

    pub fn with_mut(&mut self, with: impl FnOnce(WriteRef<'static, T>))
    where
        T: 'static,
    {
        self.subscribers.write().borrow_mut().retain(|s| s.notify());
        with(self.key.write());
    }

    pub fn write_unchecked(&self) -> WriteRef<'static, T> {
        for subscriber in self.subscribers.write().borrow_mut().iter() {
            subscriber.notify();
        }
        self.key.write()
    }

    pub fn set(&mut self, value: T)
    where
        T: 'static,
    {
        *self.write() = value;
    }

    pub fn set_if_modified(&mut self, value: T)
    where
        T: 'static + PartialEq,
    {
        let is_equal = *self.peek() == value;
        if !is_equal {
            self.set(value);
        }
    }

    pub fn set_if_modified_and_then(&mut self, value: T, then: impl FnOnce())
    where
        T: 'static + PartialEq,
    {
        let is_equal = *self.peek() == value;
        if !is_equal {
            self.set(value);
            then();
        }
    }

    /// Create a [State] attached to the current Scope.
    pub fn create(value: T) -> Self
    where
        T: 'static, // TODO: Move this lifetime bound to impl
    {
        Self::create_in_scope(value, None)
    }

    /// Create a [State] attached to the given Scope.
    pub fn create_in_scope(value: T, scope_id: impl Into<Option<ScopeId>>) -> Self
    where
        T: 'static,
    {
        // TODO: Move this lifetime bound to impl
        let owner = CurrentContext::with(|context| {
            let scopes_storages = context.scopes_storages.borrow_mut();

            let scopes_storage = scopes_storages.get(&scope_id.into().unwrap_or(context.scope_id));
            scopes_storage.unwrap().owner.clone()
        });
        let key = owner.insert(value);
        let subscribers = owner.insert(Rc::default());
        State { key, subscribers }
    }

    /// Create a global [State] that is expected to live until the end of the process.
    pub fn create_global(value: T) -> Self
    where
        T: 'static,
    {
        let owner = UnsyncStorage::owner();
        Box::leak(Box::new(owner.clone()));
        let key = owner.insert(value);
        let subscribers = owner.insert(Rc::default());
        State { key, subscribers }
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for State<T> {}

impl<T> State<Option<T>> {
    pub fn take(&mut self) -> Option<T>
    where
        T: 'static,
    {
        self.write().take()
    }
}

pub fn use_state<T: 'static>(init: impl FnOnce() -> T) -> State<T> {
    use_hook(|| State::create(init()))
}
