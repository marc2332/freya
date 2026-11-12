use crate::{
    prelude::{
        State,
        WritableUtils,
        spawn,
        use_hook,
    },
    reactive_context::ReactiveContext,
};

/// Creates values that are recomputed when their reactive dependencies change.
pub struct Derived;

impl Derived {
    /// Computes the initial value immediately and returns it as [`State<T>`].
    /// Reactive changes rerun the callback in a task and update the value.
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// # fn app() -> impl IntoElement {
    /// let count = use_state(|| 1);
    /// let doubled = use_hook(move || Derived::create(move || *count.read() * 2));
    /// rect().child(format!("Doubled: {}", *doubled.read()))
    /// # }
    /// ```
    pub fn create<T: 'static>(mut callback: impl FnMut() -> T + 'static) -> State<T> {
        let (rx, rc) = ReactiveContext::new_for_task();
        let mut state = State::create(ReactiveContext::run(rc.clone(), &mut callback));
        spawn(async move {
            let mut current_gen = 0;
            loop {
                if current_gen > 0 {
                    state.set(ReactiveContext::run(rc.clone(), &mut callback));
                }
                rx.notified().await;
                current_gen += 1;
            }
        });
        state
    }
}

/// Hook for [`Derived::create`].
/// Computes a derived value immediately and returns its [`State<T>`].
/// Recomputes asynchronously when reactive values read by the callback change.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn app() -> impl IntoElement {
/// let count = use_state(|| 1);
/// let doubled = use_derived(move || *count.read() * 2);
/// rect().child(format!("Doubled: {}", *doubled.read()))
/// # }
/// ```
pub fn use_derived<T: 'static>(callback: impl FnMut() -> T + 'static) -> State<T> {
    use_hook(|| Derived::create(callback))
}
