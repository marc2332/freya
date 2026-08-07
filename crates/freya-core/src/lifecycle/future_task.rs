use crate::prelude::*;
pub enum FutureState<D> {
    /// Has not started loading yet.
    Pending,
    /// Currently loading.
    Loading,
    /// Finished loading and has data.
    Fulfilled(D),
}

impl<D> FutureState<D> {
    pub fn ok(&self) -> Option<&D> {
        if let Self::Fulfilled(d) = &self {
            Some(d)
        } else {
            None
        }
    }

    pub fn unwrap(&self) -> &D {
        self.ok().expect("Future state is not fulfilled")
    }

    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading)
    }

    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }
}

pub struct FutureTask<D, F> {
    future: State<Box<dyn FnMut() -> F>>,
    state: State<FutureState<D>>,
    task: State<Option<TaskHandle>>,
}

impl<D, F> Clone for FutureTask<D, F> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<D, F> Copy for FutureTask<D, F> {}

impl<D: 'static, F: Future<Output = D> + 'static> FutureTask<D, F> {
    /// Create a [FutureTask] with the given callback.
    pub fn create(future: impl FnMut() -> F + 'static) -> FutureTask<D, F> {
        Self {
            future: State::create(Box::new(future)),
            state: State::create(FutureState::Pending),
            task: State::create(None),
        }
    }

    /// Create a [FutureTask] with a reactive callback. Any [State] read inside the
    /// callback (outside the async block) subscribes it, restarting the future when it changes.
    pub fn create_reactive(mut future: impl FnMut() -> F + 'static) -> FutureTask<D, F> {
        let (notify, reactive_context) = ReactiveContext::new_for_task();
        let mut future_task =
            Self::create(move || ReactiveContext::run(reactive_context.clone(), &mut future));
        spawn(async move {
            loop {
                future_task.start();
                notify.notified().await;
            }
        });
        future_task
    }

    /// Cancel the currently task if there is any.
    pub fn cancel(&mut self) {
        if let Some(task) = self.task.take() {
            task.cancel();
        }
    }

    /// Start the [FutureTask]. If it was running already then it will be restarted.
    pub fn start(&mut self) {
        self.cancel();
        let mut this = *self;
        let task = spawn(async move {
            let future = this.future.write()();
            this.state.set(FutureState::Loading);
            let data = future.await;
            this.state.set(FutureState::Fulfilled(data));
        });
        self.task.set(Some(task));
    }

    /// Read the state of the [FutureTask]. See [FutureState].
    pub fn state(&self) -> ReadRef<'static, FutureState<D>> {
        self.state.read()
    }
}

/// Create a [FutureTask] with the given callback.
///
/// This is a hook around [spawn] that exposes the progress of the future as
/// reactive state, so you can render the pending, loading and fulfilled cases
/// without managing the task by hand. It starts polling automatically.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// #[derive(PartialEq)]
/// struct Greeting;
///
/// impl Component for Greeting {
///     fn render(&self) -> impl IntoElement {
///         let future = use_future(|| async {
///             // Some async work...
///             "Hello!".to_string()
///         });
///
///         match &*future.state() {
///             FutureState::Pending | FutureState::Loading => "Loading...".to_string(),
///             FutureState::Fulfilled(text) => text.clone(),
///         }
///     }
/// }
/// ```
///
/// The callback is reactive, any [State] read inside of it (outside the async block)
/// subscribes the future, restarting it when that state changes. Reads inside the
/// async block do not subscribe.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # async fn load_user(user_id: usize) -> String { String::new() }
/// # fn app() -> impl IntoElement {
/// let user_id = use_state(|| 1);
///
/// // Restarts whenever `user_id` changes.
/// let user = use_future(move || {
///     let user_id = user_id();
///     async move { load_user(user_id).await }
/// });
/// # let _ = user;
/// # rect()
/// # }
/// ```
///
/// To read its state use [FutureTask::state].
/// You may restart/stop it using [FutureTask::start] and [FutureTask::cancel].
pub fn use_future<D: 'static, F: Future<Output = D> + 'static>(
    future: impl FnMut() -> F + 'static,
) -> FutureTask<D, F> {
    use_hook(|| FutureTask::create_reactive(future))
}
