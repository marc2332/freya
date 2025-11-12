use std::{
    cell::Cell,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{
        Context,
        Poll,
        Waker,
    },
};

#[derive(Clone, Default)]
pub struct Notify {
    state: Rc<State>,
}

#[derive(Default)]
struct State {
    flag: Cell<bool>,
    waker: Cell<Option<Waker>>,
}

impl Notify {
    pub fn new() -> Self {
        Self {
            state: Rc::new(State {
                flag: Cell::new(false),
                waker: Cell::new(None),
            }),
        }
    }

    pub fn notify(&self) {
        self.state.flag.set(true);

        if let Some(w) = self.state.waker.take() {
            w.wake();
        }
    }

    pub fn notified(&self) -> Notified {
        Notified {
            state: self.state.clone(),
        }
    }
}

pub struct Notified {
    state: Rc<State>,
}

impl Future for Notified {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.state.flag.replace(false) {
            Poll::Ready(())
        } else {
            self.state.waker.set(Some(cx.waker().clone()));
            Poll::Pending
        }
    }
}
