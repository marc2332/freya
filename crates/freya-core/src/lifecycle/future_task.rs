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
    pub fn try_as_fulfilled(&self) -> Option<&D> {
        if let Self::Fulfilled(d) = &self {
            Some(d)
        } else {
            None
        }
    }

    pub fn as_fulfilled(&self) -> &D {
        self.try_as_fulfilled()
            .expect("Future state is not fulfilled")
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

/// Creata [FutureTask] with the given callback.
///
/// It will automatically start polling.
///
/// To read it's state use [FutureTask::state].
/// You may restart/stop it using [FutureTask::start] and [FutureTask::cancel].
pub fn use_future<D: 'static, F: Future<Output = D> + 'static>(
    future: impl FnMut() -> F + 'static,
) -> FutureTask<D, F> {
    use_hook(|| {
        let mut future = FutureTask::create(future);
        future.start();
        future
    })
}
