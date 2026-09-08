use std::{
    cell::RefCell,
    pin::Pin,
    rc::Rc,
    sync::{
        Arc,
        atomic::Ordering,
    },
};

use crate::{
    current_context::CurrentContext,
    prelude::current_scope_id,
    runner::Message,
    scope_id::ScopeId,
};

/// Spawn a task attached to the root scope.
///
/// Unlike [`spawn`], this task keeps running when the component that started it
/// unmounts. Use it for app-wide work like initializing a shared cache or a
/// background synchronization loop. It runs until it finishes, the app exits, or
/// its [`TaskHandle`] is cancelled.
///
/// Spawn it from a hook so rerenders do not create duplicates:
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # async fn initialize_shared_cache() {}
/// # fn app() -> impl IntoElement {
/// let _cache_task = use_hook(|| {
///     spawn_forever(async {
///         initialize_shared_cache().await;
///     })
/// });
///
/// rect()
/// # }
/// ```
pub fn spawn_forever(future: impl Future<Output = ()> + 'static) -> TaskHandle {
    spawn_in_scope(future, ScopeId::ROOT)
}

/// Spawn a task attached to the current component scope.
///
/// Use it for async work owned by a component, like handling an event, waiting
/// for a timer or loading component-specific data. Freya cancels the task when
/// that component unmounts. The returned [`TaskHandle`] lets you cancel it
/// earlier.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # async fn save_document() {}
/// # fn save_button() -> impl IntoElement {
/// Button::new().child("Save").on_press(|_| {
///     spawn(async {
///         save_document().await;
///     });
/// })
/// # }
/// ```
pub fn spawn(future: impl Future<Output = ()> + 'static) -> TaskHandle {
    spawn_in_scope(future, None)
}

/// Spawn a task attached to the given scope, or to the current one when passing `None`.
pub fn spawn_in_scope(
    future: impl Future<Output = ()> + 'static,
    scope_id: impl Into<Option<ScopeId>>,
) -> TaskHandle {
    let scope_id = scope_id.into().unwrap_or_else(current_scope_id);
    CurrentContext::with(|context| {
        let task_id = TaskId(context.task_id_counter.fetch_add(1, Ordering::Relaxed));
        context.tasks.borrow_mut().insert(
            task_id,
            Rc::new(RefCell::new(Task {
                scope_id,
                future: Box::pin(future),
                waker: futures_util::task::waker(Arc::new(TaskWaker {
                    task_id,
                    sender: context.sender.clone(),
                })),
            })),
        );
        context
            .sender
            .unbounded_send(Message::PollTask(task_id))
            .unwrap();
        task_id.into()
    })
}

/// A non-owning handle used to cancel a spawned task manually.
///
/// Dropping this handle does not cancel the task. Call [`TaskHandle::cancel`]
/// explicitly, or use [`TaskHandle::owned`] when the task should be cancelled as
/// its owner is dropped.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct TaskHandle(TaskId);

impl From<TaskId> for TaskHandle {
    fn from(value: TaskId) -> Self {
        TaskHandle(value)
    }
}

impl TaskHandle {
    /// Cancel the task.
    ///
    /// Use it when an event or state change makes an in-progress task no longer
    /// necessary. This method must run within Freya's current context, use
    /// [`TaskHandle::try_cancel`] for cleanup that may run outside it.
    pub fn cancel(&self) {
        CurrentContext::with(|context| context.tasks.borrow_mut().remove(&self.0));
    }

    /// Try to cancel the task if Freya's current context is available.
    ///
    /// Unlike [`TaskHandle::cancel`], this does nothing when called outside
    /// Freya's context. Prefer it in destructors and other cleanup paths where a
    /// context might no longer exist.
    pub fn try_cancel(&self) {
        CurrentContext::try_with(|context| context.tasks.borrow_mut().remove(&self.0));
    }

    /// Check whether the task is no longer scheduled.
    pub fn is_finished(&self) -> bool {
        CurrentContext::with(|context| !context.tasks.borrow().contains_key(&self.0))
    }

    /// Upgrade to an [`OwnedTaskHandle`] that cancels the task when its last
    /// clone is dropped.
    ///
    /// Retain the returned handle for as long as the task should run. Useful for
    /// a task owned by another long-lived value rather than by a component
    /// scope:
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// # async fn forward_messages() {}
    /// struct Worker {
    ///     _task: OwnedTaskHandle,
    /// }
    ///
    /// # fn start_worker() -> Worker {
    /// let worker = Worker {
    ///     _task: spawn_forever(forward_messages()).owned(),
    /// };
    /// # worker
    /// # }
    /// ```
    pub fn owned(self) -> OwnedTaskHandle {
        OwnedTaskHandle(Rc::new(InnerOwnedTaskHandle(self)))
    }
}

struct InnerOwnedTaskHandle(TaskHandle);

impl Drop for InnerOwnedTaskHandle {
    fn drop(&mut self) {
        self.0.try_cancel();
    }
}

/// An owning handle that cancels its task when the last clone is dropped.
///
/// Use [`TaskHandle::owned`] to create one. Clones share ownership of the same
/// task, so dropping one only cancels the task when no other clones remain.
#[derive(Clone)]
pub struct OwnedTaskHandle(Rc<InnerOwnedTaskHandle>);

impl PartialEq for OwnedTaskHandle {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl OwnedTaskHandle {
    /// Cancel the owned task immediately.
    ///
    /// This method has the same context requirement as [`TaskHandle::cancel`].
    pub fn cancel(&self) {
        self.0.0.cancel();
    }

    /// Try to cancel the owned task if Freya's current context is available.
    ///
    /// Use this instead of [`OwnedTaskHandle::cancel`] from cleanup code that
    /// may run after Freya's context has been removed.
    pub fn try_cancel(&self) {
        self.0.0.try_cancel();
    }

    /// Check whether the task is no longer scheduled.
    pub fn is_finished(&self) -> bool {
        self.0.0.is_finished()
    }

    /// Get a non-owning [`TaskHandle`] for the same task.
    ///
    /// The returned handle can cancel the task, but dropping it has no effect
    /// on the [`OwnedTaskHandle`]'s ownership.
    pub fn downgrade(&self) -> TaskHandle {
        self.0.0
    }
}

/// Wakes a Freya task by asking the runner to poll it again.
///
/// This is a runtime implementation detail, application code normally uses
/// [`spawn`] or [`spawn_forever`] instead.
pub struct TaskWaker {
    task_id: TaskId,
    sender: futures_channel::mpsc::UnboundedSender<Message>,
}

impl futures_util::task::ArcWake for TaskWaker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        _ = arc_self
            .sender
            .unbounded_send(Message::PollTask(arc_self.task_id));
    }
}

/// A future scheduled by Freya's async runtime.
///
/// This is a runtime implementation detail stored by the runner. Application
/// code should use [`TaskHandle`] to interact with spawned tasks.
pub struct Task {
    pub scope_id: ScopeId,
    pub future: Pin<Box<dyn Future<Output = ()>>>,
    /// Used to notify the runner that this task needs progress.
    pub waker: futures_util::task::Waker,
}

/// The opaque identifier of a task scheduled by Freya's async runtime.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(u64);
