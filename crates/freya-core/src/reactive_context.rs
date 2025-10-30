use std::{
    cell::RefCell,
    hash::{
        Hash,
        Hasher,
    },
    rc::Rc,
};

use futures_channel::mpsc::UnboundedSender;
use generational_box::GenerationalBox;
use rustc_hash::FxHashSet;

use crate::{
    current_context::CurrentContext,
    notify::Notify,
    runner::Message,
    scope_id::ScopeId,
};

pub(crate) struct Inner {
    self_rc: Option<ReactiveContext>,
    update: Rc<dyn Fn()>,
    subscriptions: Vec<Rc<RefCell<FxHashSet<ReactiveContext>>>>,
}

impl Drop for Inner {
    fn drop(&mut self) {
        let Some(self_rc) = self.self_rc.take() else {
            return;
        };
        for subscription in self.subscriptions.drain(..) {
            subscription.borrow_mut().remove(&self_rc);
        }
    }
}

#[derive(Clone)]
pub struct ReactiveContext {
    pub(crate) inner: GenerationalBox<Inner>,
}

impl Hash for ReactiveContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.id().hash(state);
    }
}

impl PartialEq for ReactiveContext {
    fn eq(&self, other: &Self) -> bool {
        self.inner.ptr_eq(&other.inner)
    }
}

impl Eq for ReactiveContext {}

impl ReactiveContext {
    pub(crate) fn new_for_scope(
        sender: UnboundedSender<Message>,
        scope_id: ScopeId,
        writer: &dyn Fn(Inner) -> GenerationalBox<Inner>,
    ) -> Self {
        let rc = Self {
            inner: (writer)(Inner {
                self_rc: None,

                update: Rc::new(move || {
                    sender
                        .unbounded_send(Message::MarkScopeAsDirty(scope_id))
                        .unwrap();
                }),
                subscriptions: Vec::default(),
            }),
        };

        rc.inner.write().self_rc = Some(rc.clone());

        rc
    }

    pub fn new_for_task() -> (Notify, Self) {
        let notify = Notify::default();

        let rc = CurrentContext::with(|ctx| {
            let owner = ctx
                .scopes_storages
                .borrow()
                .get(&ctx.scope_id)
                .map(|s| s.owner.clone())
                .unwrap();
            let notify = notify.clone();
            Self {
                inner: owner.insert(Inner {
                    self_rc: None,
                    update: Rc::new(move || {
                        notify.notify();
                    }),
                    subscriptions: Vec::default(),
                }),
            }
        });

        rc.inner.write().self_rc = Some(rc.clone());

        (notify, rc)
    }

    pub fn run<T>(new_context: Self, run: impl FnOnce() -> T) -> T {
        for subscription in new_context.inner.write().subscriptions.drain(..) {
            subscription.borrow_mut().remove(&new_context);
        }
        REACTIVE_CONTEXTS_STACK.with_borrow_mut(|context| context.push(new_context));
        let res = run();
        REACTIVE_CONTEXTS_STACK.with_borrow_mut(|context| context.pop());

        res
    }

    pub fn current() -> Option<Self> {
        REACTIVE_CONTEXTS_STACK.with_borrow(|contexts| contexts.last().cloned())
    }

    pub fn notify(&self) -> bool {
        if let Ok(inner) = self.inner.try_write() {
            (inner.update)();
            true
        } else {
            false
        }
    }

    pub fn subscribe(&mut self, subscribers: &Rc<RefCell<FxHashSet<ReactiveContext>>>) {
        subscribers.borrow_mut().insert(self.clone());
        self.inner.write().subscriptions.push(subscribers.clone())
    }
}

thread_local! {
    static REACTIVE_CONTEXTS_STACK: RefCell<Vec<ReactiveContext>> = const { RefCell::new(Vec::new()) }
}
