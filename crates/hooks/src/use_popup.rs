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

    /// Read the last answer.
    pub fn read(&self) -> ReadableRef<Signal<Option<T>>> {
        self.value.read_unchecked()
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

#[cfg(test)]
mod test {
    use dioxus::prelude::component;
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn popup() {
        fn popup_app() -> Element {
            let mut my_popup = use_popup::<String>();

            let onpress = move |_| async move {
                let _name = my_popup.open().await;
            };

            rsx!(
                Button {
                    onpress,
                    label {
                        "{my_popup.read():?}"
                    }
                }
                if my_popup.is_open() {
                    AskNamePopup {}
                }
            )
        }

        #[component]
        fn AskNamePopup() -> Element {
            let mut popup_answer = use_popup_answer::<String>();

            rsx!(
                Button {
                    onpress: move |_| {
                        popup_answer.answer("Marc".to_string())
                    },
                    label {
                        "Answer 'Marc'"
                    }
                }
            )
        }

        let mut utils = launch_test(popup_app);
        let root = utils.root();
        let label = root.get(0).get(0);

        assert_eq!(label.get(0).text(), Some("None"));

        // Open popup
        utils.click_cursor((15.0, 15.0)).await;
        utils.wait_for_update().await;

        // Answer
        utils.click_cursor((15.0, 40.0)).await;
        utils.wait_for_update().await;

        assert_eq!(label.get(0).text(), Some("Some(\"Marc\")"));
    }
}
