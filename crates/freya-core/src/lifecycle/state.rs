use std::{
    cell::RefCell,
    fmt::{
        Debug,
        Display,
    },
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

/// A reactive state container that holds a value of type `T` and manages subscriptions to changes.
///
/// `State<T>` is the fundamental reactive primitive in Freya. It allows you to store mutable state
/// that automatically triggers re-renders in components that read from it when the value changes.
///
/// # Key Features
///
/// - **Reactive**: Components automatically re-render when the state value changes.
/// - **Copy**: `State<T>` implements `Copy`, making it cheap to pass around.
/// - **Shared**: Multiple components can read from and write to the same state.
/// - **Scoped**: State is automatically cleaned up when its owning component unmounts.
///
/// # Basic Usage
///
/// ```rust,no_run
/// use freya::prelude::*;
///
/// fn counter() -> impl IntoElement {
///     // Create reactive state
///     let mut count = use_state(|| 0);
///
///     rect().child(format!("Count: {}", count.read())).child(
///         Button::new()
///             .child("Increment")
///             .on_press(move |_| *count.write() += 1),
///     )
/// }
/// ```
///
/// # Reading State
///
/// - `state.read()` - Reads the current value and subscribes the current component to changes.
/// - `state.peek()` - Reads the current value without subscribing (rarely needed).
///
/// # Writing State
///
/// - `state.write()` - Gets a mutable reference to modify the value.
/// - `state.set(new_value)` - Replaces the current value.
/// - `state.with_mut(|mut_ref| { /* modify */ })` - Modifies using a closure.
///
/// # Advanced Patterns
///
/// ## Conditional Updates
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// let mut count = use_state(|| 0);
///
/// // Only update if the new value is different
/// count.set_if_modified(5);
///
/// // Update and run additional logic
/// count.set_if_modified_and_then(10, || {
///     println!("Count reached 10!");
/// });
/// ```
///
/// ## Working with Options
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// let mut optional_value = use_state(|| Some(42));
///
/// // Take ownership of the contained value
/// let taken_value = optional_value.take(); // Returns Option<i32>
/// ```
///
/// ## Copy Types
///
/// For `Copy` types, you can call the state as a function to read:
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// let count = use_state(|| 0);
///
/// // These are equivalent:
/// let value1 = count.read().clone();
/// let value2 = count(); // Only works for Copy types
/// ```
///
/// # Global State
///
/// For state that persists across the entire application lifecycle:
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// // Create global state (use sparingly)
/// let global_count = State::create_global(0);
/// ```
///
/// # Thread Safety
///
/// `State<T>` is not thread-safe and should only be used within the main UI thread.
/// For cross-thread communication, consider using channels or other synchronization primitives.
///
/// # Performance Notes
///
/// - Reading state subscribes the current component, causing re-renders when it changes.
/// - Use `peek()` only when you specifically don't want reactivity.
/// - Prefer `set_if_modified()` over `set()` when the value might not have changed.
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
    /// Toggle the boolean-like value and return the new value.
    ///
    /// This method negates the current value using the `!` operator and returns
    /// the new value after updating the state.
    ///
    /// # Requirements
    ///
    /// The type `T` must implement `std::ops::Not<Output = T> + Clone`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// let mut flag = use_state(|| false);
    ///
    /// // Toggle and get the new value
    /// let new_value = flag.toggled(); // false -> true, returns true
    /// assert_eq!(new_value, true);
    /// ```
    ///
    /// # Common Types
    ///
    /// Works with `bool`, custom enum types, etc.
    pub fn toggled(&mut self) -> T {
        let value = self.read().clone();
        let neg_value = !value;
        self.set(neg_value.clone());
        neg_value
    }

    /// Toggle the boolean-like value without returning it.
    ///
    /// This is a convenience method that toggles the value but discards the result.
    /// Equivalent to calling [toggled](Self::toggled) and ignoring the return value.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// let mut is_visible = use_state(|| false);
    ///
    /// // Toggle visibility
    /// is_visible.toggle(); // false -> true
    /// ```
    pub fn toggle(&mut self) {
        self.toggled();
    }
}

pub enum ReadableRef<T: 'static> {
    Ref(ReadRef<'static, T>),
    Borrowed(Rc<T>),
}

impl<T: 'static + Debug> Debug for ReadableRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ref(r) => r.fmt(f),
            Self::Borrowed(r) => r.deref().fmt(f),
        }
    }
}

impl<T: 'static + Display> Display for ReadableRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ref(r) => r.fmt(f),
            Self::Borrowed(r) => r.deref().fmt(f),
        }
    }
}

impl<T> Deref for ReadableRef<T> {
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
    /// Read the current value and subscribe the current component to changes.
    ///
    /// When the state value changes, any component or hook that has called `read()` will re-render.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// let count = use_state(|| 0);
    /// let current_value = count.read();
    /// ```
    pub fn read(&self) -> ReadRef<'static, T> {
        if let Some(mut rc) = ReactiveContext::try_current() {
            rc.subscribe(&self.subscribers.read());
        }
        self.key.read()
    }

    /// Read the current value without subscribing to changes.
    ///
    /// This method provides access to the current state value without registering the current
    /// component as a subscriber. The component will **not** re-render if the state changes.
    ///
    /// # When to Use
    ///
    /// Use `peek()` when you need to read the state value for a one-off operation where
    /// reactivity is not needed, such as:
    /// - Comparisons for conditional updates
    /// - Debugging/logging
    /// - Initial value checks
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// let count = use_state(|| 0);
    ///
    /// // Check if count is zero without subscribing
    /// if *count.peek() == 0 {
    ///     println!("Count is still zero");
    /// }
    ///
    /// // For reactive reading, use `read()` instead:
    /// let display_text = format!("Count: {}", count.read());
    /// ```
    ///
    /// # Performance Note
    ///
    /// Prefer `read()` over `peek()` unless you specifically need non-reactive access.
    pub fn peek(&self) -> ReadRef<'static, T> {
        self.key.read()
    }

    /// Get a mutable reference to the state value and notify subscribers.
    ///
    /// This method returns a `WriteRef<T>` that allows direct mutation of the state value.
    /// All subscribed components will be notified and will re-render on the next frame.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// let mut count = use_state(|| 0);
    ///
    /// // Direct mutation
    /// *count.write() += 1;
    ///
    /// // Multiple operations
    /// {
    ///     let mut value = count.write();
    ///     *value *= 2;
    ///     *value += 10;
    /// } // Subscribers notified here
    /// ```
    ///
    /// # See Also
    ///
    /// - `with_mut()` for closure-based mutations
    /// - `set()` for replacing the entire value
    pub fn write(&mut self) -> WriteRef<'static, T> {
        self.subscribers.write().borrow_mut().retain(|s| s.notify());
        self.key.write()
    }

    /// Modify the state value using a closure and notify subscribers.
    ///
    /// This method provides a convenient way to mutate the state value using a closure,
    /// automatically handling subscriber notification.
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
    ///
    /// // Equivalent to:
    /// *counter.write() += 1;
    /// *counter.write() *= 2;
    /// // But more efficient (single notification)
    /// ```
    pub fn with_mut(&mut self, with: impl FnOnce(WriteRef<'static, T>))
    where
        T: 'static,
    {
        self.subscribers.write().borrow_mut().retain(|s| s.notify());
        with(self.key.write());
    }

    /// Get a mutable reference without requiring a mutable borrow of the State.
    ///
    /// This is an advanced method that allows writing to the state without having
    /// mutable access to the `State` itself. Use with caution as it bypasses Rust's
    /// borrow checker guarantees.
    ///
    /// # Safety Considerations
    ///
    /// This method should only be used when you cannot obtain a mutable reference
    /// to the `State` but still need to modify it. Prefer `write()` when possible.
    pub fn write_unchecked(&self) -> WriteRef<'static, T> {
        self.subscribers.write().borrow_mut().retain(|s| s.notify());
        self.key.write()
    }

    /// Replace the current state value with a new one.
    ///
    /// This method completely replaces the existing value with the provided one
    /// and notifies all subscribers.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// let mut status = use_state(|| "idle");
    ///
    /// // Replace the value
    /// status.set("loading");
    /// status.set("complete");
    /// ```
    ///
    /// # See Also
    ///
    /// - `set_if_modified()` to avoid unnecessary updates when the value hasn't changed
    pub fn set(&mut self, value: T)
    where
        T: 'static,
    {
        *self.write() = value;
    }

    /// Replace the state value only if it's different from the current value.
    ///
    /// This method compares the new value with the current value using `PartialEq`.
    /// If they are different, it updates the state and notifies subscribers.
    /// If they are the same, no update occurs.
    ///
    /// # Performance Benefits
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
    ///
    /// # Requirements
    ///
    /// The type `T` must implement `PartialEq`.
    pub fn set_if_modified(&mut self, value: T)
    where
        T: 'static + PartialEq,
    {
        let is_equal = *self.peek() == value;
        if !is_equal {
            self.set(value);
        }
    }

    /// Replace the state value if modified and execute a callback.
    ///
    /// Similar to `set_if_modified()`, but also runs a callback function if the value
    /// was actually changed.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// let mut score = use_state(|| 0);
    ///
    /// score.set_if_modified_and_then(100, || {
    ///     println!("High score achieved!");
    ///     // Trigger additional logic like saving to storage
    /// });
    /// ```
    ///
    /// # Use Cases
    ///
    /// - Logging state changes
    /// - Triggering side effects only when value changes
    /// - Analytics tracking
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

    /// Create a new State attached to the current component's scope.
    ///
    /// This method creates a reactive state value that will be automatically cleaned up
    /// when the current component unmounts.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// // Usually used through use_state() hook instead:
    /// let count = use_state(|| 0);
    ///
    /// // Direct creation (rare):
    /// let state = State::create(42);
    /// ```
    ///
    /// # See Also
    ///
    /// - `use_state()` - The recommended way to create state in components
    /// - `create_global()` - For application-wide state
    pub fn create(value: T) -> Self
    where
        T: 'static, // TODO: Move this lifetime bound to impl
    {
        Self::create_in_scope(value, None)
    }

    /// Create a State attached to a specific scope.
    ///
    /// Advanced method for creating state in a different scope than the current one.
    /// Pass `None` to attach to the current scope (same as `create()`).
    ///
    /// # Parameters
    ///
    /// - `value`: The initial value for the state
    /// - `scope_id`: The scope to attach to, or `None` for current scope
    ///
    /// # Use Cases
    ///
    /// - Creating state in parent scopes
    /// - Advanced component patterns
    /// - Testing utilities
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

    /// Create a global State that persists for the entire application lifetime.
    ///
    /// This creates state that is not tied to any component scope and will live
    /// until the application shuts down. Use sparingly as it can lead to memory leaks
    /// if not managed carefully.
    ///
    /// # Warning
    ///
    /// Global state should be used judiciously. Prefer component-scoped state (`use_state()`)
    /// or shared state (`freya-radio`) for most use cases.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// // Create global state in a function
    /// fn create_global_config() -> State<i32> {
    ///     State::create_global(42)
    /// }
    /// ```
    ///
    /// # Memory Management
    ///
    /// Global state is leaked using `Box::leak()` and will not be automatically cleaned up.
    /// Ensure global state contains lightweight data or implement manual cleanup if needed.
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

    /// Subscribe the current reactive context to this state's changes.
    pub(crate) fn subscribe(&self) {
        if let Some(mut rc) = ReactiveContext::try_current() {
            rc.subscribe(&self.subscribers.read());
        }
    }

    /// Notify all subscribers that the state has changed.
    pub(crate) fn notify(&self) {
        self.subscribers.write().borrow_mut().retain(|s| s.notify());
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for State<T> {}

impl<T> State<Option<T>> {
    /// Take ownership of the contained value, leaving `None` in its place.
    ///
    /// This method is only available for `State<Option<T>>` and moves the value
    /// out of the state, replacing it with `None`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// let mut maybe_value = use_state(|| Some("hello".to_string()));
    ///
    /// // Take the value, state becomes None
    /// let taken = maybe_value.take(); // Some("hello")
    /// assert_eq!(*maybe_value.read(), None);
    /// ```
    ///
    /// # Use Cases
    ///
    /// - Moving values out of reactive state
    /// - One-time consumption of optional state
    /// - State transitions where the value is no longer needed
    pub fn take(&mut self) -> Option<T>
    where
        T: 'static,
    {
        self.write().take()
    }
}
/// Creates a reactive state value initialized with the returned value of the `init` callback.
///
/// This hook creates a `State<T>` that is automatically scoped to the current component.
/// The state will be cleaned up when the component unmounts.
///
/// # Parameters
///
/// - `init`: A closure that returns the initial value for the state
///
/// # Type Requirements
///
/// The type `T` must be `'static` (no borrowed references).
///
/// # Example
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// fn counter() -> impl IntoElement {
///     let mut count = use_state(|| 0);
///
///     rect().child(format!("Count: {}", count.read())).child(
///         Button::new()
///             .child("Increment")
///             .on_press(move |_| *count.write() += 1),
///     )
/// }
/// ```
///
/// # Advanced Usage
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// // Complex initialization
/// let mut user_data = use_state(|| {
///     // Expensive computation or data loading
///     String::from("default_preferences")
/// });
/// ```
///
/// # See Also
///
/// - [`State`] for the reactive state type
/// - `freya-radio` crate for global state management
pub fn use_state<T: 'static>(init: impl FnOnce() -> T) -> State<T> {
    use_hook(|| State::create(init()))
}
