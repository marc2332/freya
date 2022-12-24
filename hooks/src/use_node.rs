use dioxus_core::ScopeState;
use dioxus_hooks::{use_state, use_ref, use_effect};
use freya_common::NodeReferenceLayout;
pub use freya_elements::{NodeRefWrapper};
use tokio::sync::mpsc::unbounded_channel;


/// Creates a reference to the desired node's layout size
pub fn use_node(cx: &ScopeState) -> (&NodeRefWrapper, NodeReferenceLayout) {
    let status = use_state::<NodeReferenceLayout>(&cx, || NodeReferenceLayout::default());
    let status_getter = status.current();
    let status_setter = status.setter();
    let channel = use_ref(&cx, || {
        let (tx, rx) = unbounded_channel::<NodeReferenceLayout>();

        (tx, Some(rx))
    });
    let node_ref = cx.use_hook(|| NodeRefWrapper(channel.read().0.clone()));

    use_effect(&cx, (), move |_| {
        let channel = channel.clone();
        let getter = status_getter.clone();

        async move {
            let rx = channel.write().1.take();
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

    (node_ref, status.get().clone())
}