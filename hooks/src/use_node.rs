use dioxus::prelude::*;
use freya_layout_common::NodeReferenceLayout;
use tokio::sync::{mpsc::unbounded_channel, mpsc::UnboundedSender};

/// Creates a reference to the desired node's layout size
pub fn use_node(
    cx: &ScopeState,
) -> (
    &UseRef<UnboundedSender<NodeReferenceLayout>>,
    &UseState<NodeReferenceLayout>,
) {
    let status = use_state::<NodeReferenceLayout>(cx, NodeReferenceLayout::default);
    let status_getter = status.current();
    let status_setter = status.setter();
    let node_ref = use_ref(cx, || {
        let (tx, rx) = unbounded_channel::<NodeReferenceLayout>();

        (tx, Some(rx))
    });
    let sender = use_ref(cx, || node_ref.read().0.clone());

    use_effect(cx, (), move |()| {
        let node_ref = node_ref.clone();
        let getter = status_getter.clone();

        async move {
            let rx = node_ref.write().1.take();
            let mut rx = rx.unwrap();
            let mut prev_status = (*getter).clone();

            loop {
                if let Some(status) = rx.recv().await {
                    if prev_status != status {
                        status_setter(status.clone());
                        prev_status = status;
                    }
                }
            }
        }
    });

    (sender, status)
}
