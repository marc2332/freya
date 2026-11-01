use std::{
    cell::RefCell,
    rc::Rc,
};

use generational_box::{
    AnyStorage,
    UnsyncStorage,
};

use crate::{
    lifecycle::writable_utils::WritableUtils,
    prelude::{
        State,
        TaskHandle,
        spawn,
        spawn_global,
        use_hook,
        use_reactive,
    },
    reactive_context::ReactiveContext,
};

/// Creates side effect callback that is rerun when reactive values read on it, change.
///
/// The usual way is to use `use_side_effect`, but you can use the several [`Effect`] methods like [`Effect::create`] inside `use_hook`
/// or [`Effect::create_global`] inside `LaunchConfig::with_task`.
pub struct Effect;

impl Effect {
    /// Runs the callback asynchronously in the current component.
    /// Reruns when a reactive value read by the callback changes.
    ///
    /// Unlike [`Effect::create_sync`], the first run is not immediate.
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// # fn app() -> impl IntoElement {
    /// let count = use_state(|| 0);
    /// use_hook(move || {
    ///     Effect::create(move || {
    ///         println!("Count: {}", *count.read());
    ///     })
    /// });
    /// rect()
    /// # }
    /// ```
    pub fn create(mut callback: impl FnMut() + 'static) {
        let (rx, rc) = ReactiveContext::new_for_task();
        spawn(async move {
            loop {
                ReactiveContext::run(rc.clone(), &mut callback);
                rx.notified().await;
            }
        });
    }

    /// Runs immediately with generation `0`.
    /// Then, on every change the callback reruns with an increased generation.
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// # fn app() -> impl IntoElement {
    /// let count = use_state(|| 0);
    /// use_hook(move || {
    ///     Effect::create_sync_with_gen(move |generation| {
    ///         println!("Run {generation}: {}", *count.read());
    ///     })
    /// });
    /// rect()
    /// # }
    /// ```
    pub fn create_sync_with_gen(mut callback: impl FnMut(usize) + 'static) {
        let (rx, rc) = ReactiveContext::new_for_task();
        ReactiveContext::run(rc.clone(), || callback(0));
        spawn(async move {
            let mut current_gen = 1;
            loop {
                rx.notified().await;
                ReactiveContext::run(rc.clone(), || callback(current_gen));
                current_gen += 1;
            }
        });
    }

    /// Runs immediately and then on every change the callback reruns with an increased generation.
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// # fn app() -> impl IntoElement {
    /// let count = use_state(|| 0);
    /// use_hook(move || {
    ///     Effect::create_sync(move || {
    ///         println!("Count: {}", *count.read());
    ///     })
    /// });
    /// rect()
    /// # }
    /// ```
    pub fn create_sync(mut callback: impl FnMut() + 'static) {
        let (rx, rc) = ReactiveContext::new_for_task();
        ReactiveContext::run(rc.clone(), &mut callback);
        spawn(async move {
            loop {
                rx.notified().await;
                ReactiveContext::run(rc.clone(), &mut callback);
            }
        });
    }

    /// Spawns each callback run in a separate task, including the first run.
    /// Reruns when a reactive value read by the callback changes.
    ///
    /// Unlike [`Effect::create`], the callback does not run in the task
    /// listening for changes.
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// # fn app() -> impl IntoElement {
    /// let count = use_state(|| 0);
    /// use_hook(move || {
    ///     Effect::create_after(move || {
    ///         println!("Count: {}", *count.read());
    ///     })
    /// });
    /// rect()
    /// # }
    /// ```
    pub fn create_after(callback: impl FnMut() + 'static) {
        let (rx, rc) = ReactiveContext::new_for_task();
        let callback = Rc::new(RefCell::new(callback));
        spawn(async move {
            loop {
                let callback = callback.clone();
                let rc = rc.clone();
                spawn(async move {
                    ReactiveContext::run(rc, &mut *callback.borrow_mut());
                });
                rx.notified().await;
            }
        });
    }

    /// Computes the initial value immediately and returns it as [`State<T>`].
    /// Reactive changes rerun the callback in a task and update the value.
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// # fn app() -> impl IntoElement {
    /// let count = use_state(|| 1);
    /// let doubled = use_hook(move || Effect::create_value(move || *count.read() * 2));
    /// rect().child(format!("Doubled: {}", *doubled.read()))
    /// # }
    /// ```
    pub fn create_value<T: 'static>(mut callback: impl FnMut() -> T + 'static) -> State<T> {
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

    /// Runs asynchronously and reruns when reactive values read by the callback change.
    /// Lives independently of components and windows.
    ///
    /// Stops when the application exits or the returned [`TaskHandle`] is cancelled.
    ///
    /// You should usually call this inside `LaunchConfig::with_task`, not in components.
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// # fn main() {
    /// let count = State::create_global(0);
    /// launch(
    ///     LaunchConfig::new()
    ///         .with_exit_on_close(false)
    ///         .with_task(move |_| async move {
    ///             Effect::create_global(move || {
    ///                 println!("Count: {}", *count.read());
    ///             });
    ///         }),
    /// );
    /// # }
    /// ```
    pub fn create_global(mut callback: impl FnMut() + 'static) -> TaskHandle {
        let owner = UnsyncStorage::owner();
        let (notification, context) = ReactiveContext::new_for_async(&owner);
        spawn_global(async move {
            let _owner = owner;
            loop {
                ReactiveContext::run(context.clone(), &mut callback);
                notification.notified().await;
            }
        })
    }
}

/// Hook for [`Effect::create`](Effect::create).
/// Registers the effect once per component.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn app() -> impl IntoElement {
/// let count = use_state(|| 0);
/// use_side_effect(move || println!("Count: {}", *count.read()));
/// rect()
/// # }
/// ```
pub fn use_side_effect(callback: impl FnMut() + 'static) {
    use_hook(|| Effect::create(callback));
}

/// Hook for [`Effect::create_after`](Effect::create_after).
/// Registers the effect once per component.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn app() -> impl IntoElement {
/// let count = use_state(|| 0);
/// use_after_side_effect(move || println!("Count: {}", *count.read()));
/// rect()
/// # }
/// ```
pub fn use_after_side_effect(callback: impl FnMut() + 'static) {
    use_hook(|| Effect::create_after(callback));
}

/// Hook for [`Effect::create_value`](Effect::create_value).
/// Registers the effect once per component and returns its [`State<T>`].
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # fn app() -> impl IntoElement {
/// let count = use_state(|| 1);
/// let doubled = use_side_effect_value(move || *count.read() * 2);
/// rect().child(format!("Doubled: {}", *doubled.read()))
/// # }
/// ```
pub fn use_side_effect_value<T: 'static>(callback: impl FnMut() -> T + 'static) -> State<T> {
    use_hook(|| Effect::create_value(callback))
}

/// Hook for [`Effect::create`](Effect::create) with reactive `deps`.
/// Registers the effect once per component.
///
/// [`use_reactive`] compares `deps` on every render using [`PartialEq`].
/// The callback receives the latest value when `deps` changes.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// #[derive(PartialEq)]
/// struct Title(String);
/// impl Component for Title {
///     fn render(&self) -> impl IntoElement {
///         use_side_effect_with_deps(&self.0, |title| println!("Title: {title}"));
///         rect().child(self.0.clone())
///     }
/// }
/// ```
pub fn use_side_effect_with_deps<D: 'static + Clone + PartialEq>(
    deps: &D,
    mut callback: impl FnMut(&D) + 'static,
) {
    let deps = use_reactive(deps);
    use_hook(move || {
        Effect::create(move || {
            let deps = deps.read();
            callback(&deps)
        })
    });
}
