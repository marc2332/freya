//! Wait until work is ready or something changes, without repeatedly checking.
//! Choose `Notify` or `ArcNotify` for one waiting caller, or `BroadcastNotify` for several.

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

/// Wait for work without repeatedly asking whether it is ready, such as waiting for a new frame.
/// Only one caller should wait at a time. For a wake-up from background work, use [`ArcNotify`].
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
    /// Creates a new way to wait for work, with no wake-up saved yet.
    pub fn new() -> Self {
        Self {
            state: Rc::new(State {
                flag: Cell::new(false),
                waker: Cell::new(None),
            }),
        }
    }

    /// Wakes the waiting caller, or saves one wake-up for later if nobody is waiting.
    /// Repeated calls count as one until the wake-up is received, so this does not count work items.
    pub fn notify(&self) {
        self.state.flag.set(true);

        if let Some(w) = self.state.waker.take() {
            w.wake();
        }
    }

    /// Waits for a wake-up, including one saved before this call.
    /// After waking, check the work itself to find out what needs doing.
    pub fn notified(&self) -> Notified {
        Notified {
            state: self.state.clone(),
        }
    }
}

/// A wait returned by [`Notify::notified`]. Use `.await` to pause until notified.
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

/// Let background work wake the part of your program that needs its results.
/// Only one caller should wait at a time. Store the results separately from the wake-up.
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
    /// Creates a new way for background work to wake a waiting caller.
    pub fn new() -> Self {
        Self::default()
    }

    /// Wakes the waiting caller, or saves one wake-up for later if nobody is waiting.
    /// Repeated calls count as one until the wake-up is received, so this does not count work items.
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

    /// Waits for a wake-up, including one saved before this call.
    /// After waking, check the work itself to find out what needs doing.
    pub fn notified(&self) -> NotifiedArc {
        NotifiedArc {
            state: self.state.clone(),
        }
    }
}

/// A wait returned by [`ArcNotify::notified`]. Use `.await` to pause until notified.
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

/// Wake several parts of your program for the same change, such as the start of a new frame.
/// Unlike [`Notify`], each notification can wake all waiting callers, not just one.
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
    /// Creates a notification shared by several waiting callers.
    pub fn new() -> Self {
        Self::default()
    }

    /// Wakes every caller whose wait was created before this call.
    /// Callers that start waiting afterward will wait for the next wake-up instead.
    pub fn notify(&self) {
        self.state
            .version
            .set(self.state.version.get().wrapping_add(1));
        for waker in self.state.wakers.take() {
            waker.wake();
        }
    }

    /// Waits for the next wake-up after this call, even if `.await` comes later.
    /// Earlier wake-ups do not count, so create the wait before the change you want to wait for.
    pub fn notified(&self) -> NotifiedBroadcast {
        NotifiedBroadcast {
            state: self.state.clone(),
            version: self.state.version.get(),
        }
    }
}

/// A wait returned by [`BroadcastNotify::notified`] that completes independently of other callers.
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

    use crate::BroadcastNotify;

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
