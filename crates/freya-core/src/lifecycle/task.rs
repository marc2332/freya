use std::{
    cell::{
        Cell,
        RefCell,
    },
    hash::{
        Hash,
        Hasher,
    },
    mem::take,
    rc::{
        Rc,
        Weak,
    },
    sync::{
        Arc,
        atomic::Ordering,
    },
    task::{
        Context,
        Waker,
    },
};

use futures_channel::mpsc::UnboundedSender;
use futures_util::{
    future::LocalBoxFuture,
    task::{
        ArcWake,
        waker,
    },
};
use rustc_hash::FxHashMap;

use crate::{
    current_context::CurrentContext,
    lifecycle::global_context::GlobalContexts,
    prelude::current_scope_id,
    runner::Message,
    scope_id::ScopeId,
};

/// Spawn an event-loop task with global context that survives window closure.
/// It has no component context and must not retain window-owned state.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # async fn my_task() {}
/// # fn app() -> impl IntoElement {
/// use_hook(|| spawn_global(my_task()));
/// rect()
/// # }
/// ```
pub fn spawn_global(future: impl Future<Output = ()> + 'static) -> TaskHandle {
    if let Some(tasks) = GlobalTasks::try_get() {
        tasks.spawn(future)
    } else {
        spawn_in_window(future)
    }
}

/// Spawn a task that survives its component but is cancelled when its window closes.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # async fn my_task() {}
/// # fn app() -> impl IntoElement {
/// use_hook(|| spawn_in_window(my_task()));
/// rect()
/// # }
/// ```
pub fn spawn_in_window(future: impl Future<Output = ()> + 'static) -> TaskHandle {
    spawn_in_scope(future, ScopeId::ROOT)
}

/// Spawn a task cancelled when its current component unmounts.
/// The returned [`TaskHandle`] can cancel it earlier.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # async fn load_data() {}
/// # fn app() -> impl IntoElement {
/// use_hook(|| spawn(load_data()));
/// rect()
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
                waker: waker(Arc::new(TaskWaker {
                    task_id,
                    sender: context.sender.clone(),
                })),
            })),
        );
        context
            .sender
            .unbounded_send(Message::PollTask(task_id))
            .unwrap();
        TaskHandle {
            task_id,
            owner: TaskOwner::Window,
            global_tasks: Weak::new(),
        }
    })
}

/// A non-owning task handle whose drop does not cancel the task.
/// Use [`TaskHandle::owned`] for cancellation on drop.
#[derive(Clone, Debug)]
pub struct TaskHandle {
    task_id: TaskId,
    owner: TaskOwner,
    global_tasks: Weak<InnerGlobalTasks>,
}

impl PartialEq for TaskHandle {
    fn eq(&self, other: &Self) -> bool {
        self.task_id == other.task_id
            && self.owner == other.owner
            && Weak::ptr_eq(&self.global_tasks, &other.global_tasks)
    }
}

impl Eq for TaskHandle {}

impl Hash for TaskHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.task_id.hash(state);
        self.owner.hash(state);
        self.global_tasks.as_ptr().hash(state);
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
enum TaskOwner {
    Window,
    Global,
}

impl TaskHandle {
    /// Cancel the task. Component and window tasks require Freya's current context.
    pub fn cancel(&self) {
        if self.owner == TaskOwner::Window {
            CurrentContext::with(|context| context.tasks.borrow_mut().remove(&self.task_id));
        } else {
            self.try_cancel();
        }
    }

    /// Try to cancel the task if Freya's current context is available.
    ///
    /// Unlike [`TaskHandle::cancel`], this does nothing when called outside
    /// Freya's context. Prefer it in destructors and other cleanup paths where a
    /// context might no longer exist.
    pub fn try_cancel(&self) {
        if self.owner == TaskOwner::Window {
            CurrentContext::try_with(|context| context.tasks.borrow_mut().remove(&self.task_id));
        } else if let Some(tasks) = self.global_tasks.upgrade() {
            tasks.cancel(self.task_id);
        }
    }

    /// Check whether the task is no longer scheduled.
    pub fn is_finished(&self) -> bool {
        if self.owner == TaskOwner::Window {
            CurrentContext::with(|context| !context.tasks.borrow().contains_key(&self.task_id))
        } else {
            self.global_tasks
                .upgrade()
                .is_none_or(|tasks| !tasks.tasks.borrow().contains_key(&self.task_id))
        }
    }

    /// Upgrade to an [`OwnedTaskHandle`] that cancels the task when its last
    /// clone is dropped.
    /// Retain the returned handle for as long as the task should run. Useful for
    /// a task owned by another long-lived value rather than by a component scope.
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
/// Created with [`TaskHandle::owned`].
#[derive(Clone)]
pub struct OwnedTaskHandle(Rc<InnerOwnedTaskHandle>);

impl PartialEq for OwnedTaskHandle {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl OwnedTaskHandle {
    /// Cancel the owned task. Component and window tasks require Freya's current context.
    pub fn cancel(&self) {
        self.0.0.cancel();
    }

    /// Try to cancel the task.
    pub fn try_cancel(&self) {
        self.0.0.try_cancel();
    }

    /// Check whether the task is no longer scheduled.
    pub fn is_finished(&self) -> bool {
        self.0.0.is_finished()
    }

    /// Get a non-owning handle.
    pub fn downgrade(&self) -> TaskHandle {
        self.0.0.clone()
    }
}

/// Wakes a Freya task by asking the runner to poll it again.
pub struct TaskWaker {
    task_id: TaskId,
    sender: UnboundedSender<Message>,
}

impl ArcWake for TaskWaker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        _ = arc_self
            .sender
            .unbounded_send(Message::PollTask(arc_self.task_id));
    }
}

/// A component or window task scheduled by the runner.
pub struct Task {
    pub scope_id: ScopeId,
    pub future: LocalBoxFuture<'static, ()>,
    /// Used to notify the runner that this task needs progress.
    pub waker: Waker,
}

/// The opaque identifier of a task scheduled by Freya's async runtime.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(u64);

/// Tasks owned by the event loop rather than a window runner.
#[derive(Clone)]
pub struct GlobalTasks {
    inner: Rc<InnerGlobalTasks>,
}

struct InnerGlobalTasks {
    tasks: RefCell<FxHashMap<TaskId, Rc<RefCell<LocalBoxFuture<'static, ()>>>>>,
    next_id: Cell<u64>,
    waker: Waker,
}

impl InnerGlobalTasks {
    fn cancel(&self, task_id: TaskId) {
        let _removed_task = self.tasks.borrow_mut().remove(&task_id);
    }
}

impl GlobalTasks {
    #[track_caller]
    pub fn get() -> Self {
        Self::try_get().expect("Global tasks are unavailable outside the event loop.")
    }

    pub fn try_get() -> Option<Self> {
        GlobalContexts::try_get().and_then(|contexts| contexts.try_get_context::<Self>())
    }

    /// Create a task pool woken by the event loop.
    pub fn new(waker: Waker) -> Self {
        Self {
            inner: Rc::new(InnerGlobalTasks {
                tasks: RefCell::default(),
                next_id: Cell::default(),
                waker,
            }),
        }
    }

    /// Schedule a future on the event loop.
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static) -> TaskHandle {
        let task_id = TaskId(self.inner.next_id.get());
        self.inner.next_id.set(self.inner.next_id.get() + 1);

        self.inner
            .tasks
            .borrow_mut()
            .insert(task_id, Rc::new(RefCell::new(Box::pin(future))));
        self.inner.waker.wake_by_ref();

        TaskHandle {
            task_id,
            owner: TaskOwner::Global,
            global_tasks: Rc::downgrade(&self.inner),
        }
    }

    /// Cancel all tasks before the event loop is dropped.
    pub fn clear(&self) {
        let _removed_tasks = take(&mut *self.inner.tasks.borrow_mut());
    }

    /// Poll all event-loop tasks that were scheduled before this call.
    pub fn poll(&self) {
        let task_ids: Vec<_> = self.inner.tasks.borrow().keys().copied().collect();
        let mut context = Context::from_waker(&self.inner.waker);

        for task_id in task_ids {
            let Some(future) = self.inner.tasks.borrow().get(&task_id).cloned() else {
                continue;
            };

            let poll_result = future.borrow_mut().as_mut().poll(&mut context);
            if poll_result.is_ready() {
                self.inner.cancel(task_id);
            }
        }
    }
}
