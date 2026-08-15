use std::{
    cell::{
        Cell,
        RefCell,
    },
    future::Future,
    pin::Pin,
    rc::Rc,
    sync::{
        Arc,
        Mutex,
        atomic::AtomicBool,
    },
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

#[derive(Clone, Default)]
pub struct ArcNotify {
    state: Arc<StateArc>,
}

#[derive(Default)]
struct StateArc {
    flag: AtomicBool,
    waker: Mutex<Option<Waker>>,
}

impl ArcNotify {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn notify(&self) {
        self.state
            .flag
            .store(true, std::sync::atomic::Ordering::SeqCst);

        if let Ok(mut w) = self.state.waker.lock()
            && let Some(waker) = w.take()
        {
            waker.wake();
        }
    }

    pub fn notified(&self) -> NotifiedArc {
        NotifiedArc {
            state: self.state.clone(),
        }
    }
}

pub struct NotifiedArc {
    state: Arc<StateArc>,
}

impl Future for NotifiedArc {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self
            .state
            .flag
            .swap(false, std::sync::atomic::Ordering::SeqCst)
        {
            Poll::Ready(())
        } else {
            if let Ok(mut w) = self.state.waker.lock() {
                *w = Some(cx.waker().clone());
            }
            Poll::Pending
        }
    }
}

/// Like [Notify] but wakes every waiter, notifications are observed rather than consumed.
#[derive(Clone, Default)]
pub struct BroadcastNotify {
    state: Rc<BroadcastState>,
}

#[derive(Default)]
struct BroadcastState {
    version: Cell<u64>,
    wakers: RefCell<Vec<Waker>>,
}

impl BroadcastNotify {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn notify(&self) {
        self.state
            .version
            .set(self.state.version.get().wrapping_add(1));
        for waker in self.state.wakers.take() {
            waker.wake();
        }
    }

    /// Completes on the first [BroadcastNotify::notify] made after this call.
    pub fn notified(&self) -> NotifiedBroadcast {
        NotifiedBroadcast {
            state: self.state.clone(),
            version: self.state.version.get(),
        }
    }
}

pub struct NotifiedBroadcast {
    state: Rc<BroadcastState>,
    version: u64,
}

impl Future for NotifiedBroadcast {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.state.version.get() != self.version {
            Poll::Ready(())
        } else {
            let mut wakers = self.state.wakers.borrow_mut();
            let waker = cx.waker();
            if !wakers.iter().any(|registered| registered.will_wake(waker)) {
                wakers.push(waker.clone());
            }
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod test {
    use std::pin::pin;

    use futures_lite::future::{
        block_on,
        poll_once,
    };

    use crate::notify::BroadcastNotify;

    #[test]
    fn broadcast_notify() {
        block_on(async {
            let notify = BroadcastNotify::new();
            let mut first = pin!(notify.notified());
            let mut second = pin!(notify.notified());
            assert_eq!(poll_once(&mut first).await, None);
            assert_eq!(poll_once(&mut second).await, None);
            notify.notify();
            assert_eq!(poll_once(&mut first).await, Some(()));
            assert_eq!(poll_once(&mut second).await, Some(()));

            // A cancelled waiter must not complete a waiter registered later
            {
                let mut cancelled = pin!(notify.notified());
                assert_eq!(poll_once(&mut cancelled).await, None);
                notify.notify();
            }
            let mut late = pin!(notify.notified());
            assert_eq!(poll_once(&mut late).await, None);
            notify.notify();
            assert_eq!(poll_once(&mut late).await, Some(()));
        });
    }
}
