use std::sync::Arc;

use dioxus_core::use_hook;
use dioxus_hooks::{
    use_context,
    use_context_provider,
    use_signal,
};
use dioxus_signals::{
    CopyValue,
    MappedSignal,
    ReadableExt,
    ReadableRef,
    Signal,
    WritableExt,
};
use tokio::sync::Notify;

/// Created using [use_popup].
pub struct UsePopup<Data: 'static, Answer: 'static> {
    open: Signal<bool>,
    data: Signal<Option<Data>>,
    answer: Signal<Option<Answer>>,
    waker: CopyValue<Arc<Notify>>,
}

impl<Data, Answer> Copy for UsePopup<Data, Answer> {}

impl<Data, Answer> Clone for UsePopup<Data, Answer> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Data, Answer> UsePopup<Data, Answer> {
    /// Check if the popup needs to be open.
    pub fn is_open(&self) -> bool {
        *self.open.read()
    }

    /// Read the last answer.
    pub fn read(&self) -> ReadableRef<Signal<Option<Answer>>> {
        self.answer.read_unchecked()
    }

    /// Mark the popup as open and await for its response.
    pub async fn open(
        &mut self,
        data: impl Into<Option<Data>>,
    ) -> ReadableRef<Signal<Option<Answer>>> {
        self.open.set(true);
        self.data.set(data.into());
        let waker = self.waker.read().clone();
        waker.notified().await;
        self.open.set(false);
        self.answer.read_unchecked()
    }
}

/// Create a popups context which can later be answered using [use_popup_answer].
pub fn use_popup<Data, Answer>() -> UsePopup<Data, Answer> {
    let data = use_signal(|| None);
    let answer = use_signal(|| None);
    let waker = use_hook(|| CopyValue::new(Arc::new(Notify::new())));

    use_context_provider(move || UsePopupAnswer {
        data,
        answer,
        waker,
    });

    use_hook(move || UsePopup {
        open: Signal::new(false),
        data,
        answer,
        waker,
    })
}

pub struct UsePopupAnswer<Data: 'static, Answer: 'static> {
    data: Signal<Option<Data>>,
    answer: Signal<Option<Answer>>,
    waker: CopyValue<Arc<Notify>>,
}

impl<Data, Answer> Clone for UsePopupAnswer<Data, Answer> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Data, Answer> Copy for UsePopupAnswer<Data, Answer> {}

impl<Data, Answer> UsePopupAnswer<Data, Answer> {
    /// Answer the popup.
    pub fn answer(&mut self, data: impl Into<Option<Answer>>) {
        self.answer.set(data.into());
        self.waker.read().notify_waiters();
    }

    /// Read the data provided to this popup, if any.
    pub fn data(&self) -> MappedSignal<Data, Signal<Option<Data>>> {
        self.data.map(|data| data.as_ref().unwrap())
    }
}

/// Answer a popup created with [use_popup].
pub fn use_popup_answer<Data, Answer>() -> UsePopupAnswer<Data, Answer> {
    use_context::<UsePopupAnswer<Data, Answer>>()
}

#[cfg(test)]
mod test {
    use dioxus::prelude::component;
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn popup() {
        fn popup_app() -> Element {
            let mut my_popup = use_popup::<(), String>();

            let onpress = move |_| async move {
                let _name = my_popup.open(()).await;
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
            let mut popup_answer = use_popup_answer::<(), String>();

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
