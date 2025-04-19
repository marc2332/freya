use std::sync::Arc;

use dioxus_core::use_hook;
use dioxus_hooks::{
    use_context,
    use_context_provider,
    use_signal,
};
use dioxus_signals::{
    CopyValue,
    Readable,
    ReadableRef,
    Signal,
    Writable,
};
use tokio::sync::Notify;

/// Created using [use_popup].
pub struct UsePopup<T: 'static> {
    open: Signal<bool>,
    value: Signal<Option<T>>,
    waker: CopyValue<Arc<Notify>>,
}

impl<T> Copy for UsePopup<T> {}

impl<T> Clone for UsePopup<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> UsePopup<T> {
    /// Check if the popup needs to be open.
    pub fn is_open(&self) -> bool {
        *self.open.read()
    }

    /// Mark the popup as open an await for its response.
    pub async fn open(&mut self) -> ReadableRef<Signal<Option<T>>> {
        self.open.set(true);
        let waker = self.waker.read().clone();
        waker.notified().await;
        self.open.set(false);
        self.value.read_unchecked()
    }
}

/// Create a popups context which can later be answered using [use_popup].
pub fn use_popup<T>() -> UsePopup<T> {
    let value = use_signal(|| None);
    let waker = use_hook(|| CopyValue::new(Arc::new(Notify::new())));

    use_context_provider(move || UsePopupAnswer { value, waker });

    use_hook(move || UsePopup {
        open: Signal::new(false),
        value,
        waker,
    })
}

pub struct UsePopupAnswer<T: 'static> {
    value: Signal<Option<T>>,
    waker: CopyValue<Arc<Notify>>,
}

impl<T> Clone for UsePopupAnswer<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for UsePopupAnswer<T> {}

impl<T> UsePopupAnswer<T> {
    /// Answer the popup.
    pub fn answer(&mut self, data: impl Into<Option<T>>) {
        self.value.set(data.into());
        self.waker.read().notify_waiters();
    }
}

/// Answer a popup created with [use_popup].
pub fn use_popup_answer<T>() -> UsePopupAnswer<T> {
    use_context::<UsePopupAnswer<T>>()
}
