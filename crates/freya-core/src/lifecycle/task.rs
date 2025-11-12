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

pub fn spawn_forever(future: impl Future<Output = ()> + 'static) -> TaskHandle {
    CurrentContext::with(|context| {
        let task_id = TaskId(context.task_id_counter.fetch_add(1, Ordering::Relaxed));
        context.tasks.borrow_mut().insert(
            task_id,
            Rc::new(RefCell::new(Task {
                scope_id: ScopeId::ROOT,
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

pub fn spawn(future: impl Future<Output = ()> + 'static) -> TaskHandle {
    CurrentContext::with(|context| {
        let task_id = TaskId(context.task_id_counter.fetch_add(1, Ordering::Relaxed));
        context.tasks.borrow_mut().insert(
            task_id,
            Rc::new(RefCell::new(Task {
                scope_id: current_scope_id(),
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

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct TaskHandle(TaskId);

impl From<TaskId> for TaskHandle {
    fn from(value: TaskId) -> Self {
        TaskHandle(value)
    }
}

impl TaskHandle {
    pub fn cancel(&self) {
        CurrentContext::with(|context| context.tasks.borrow_mut().remove(&self.0));
    }
}

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

pub struct Task {
    pub scope_id: ScopeId,
    pub future: Pin<Box<dyn Future<Output = ()>>>,
    /// Used to notify the runner that this task needs progress.
    pub waker: futures_util::task::Waker,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(u64);
