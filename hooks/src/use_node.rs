use dioxus_core::ScopeState;
use dioxus_hooks::{use_effect, use_state};
use freya_common::NodeReferenceLayout;
pub use freya_elements::NodeRefWrapper;
use tokio::sync::mpsc::unbounded_channel;

/// Creates a reference to the desired node's layout size
pub fn use_node(cx: &ScopeState) -> (NodeRefWrapper, NodeReferenceLayout) {
    let status = use_state::<NodeReferenceLayout>(&cx, || NodeReferenceLayout::default());

    let channel = cx.use_hook(|| {
        let (tx, rx) = unbounded_channel::<NodeReferenceLayout>();
        (tx, Some(rx))
    });

    let node_ref = NodeRefWrapper(channel.0.clone());

    use_effect(&cx, (), move |_| {
        let rx = channel.1.take();
        let status = status.clone();
        cx.spawn(async move {
            let mut rx = rx.unwrap();
            while let Some(new_status) = rx.recv().await {
                if status.current().as_ref() != &new_status {
                    status.set(new_status);
                }
            }
        });
        async move {}
    });

    (node_ref, status.get().clone())
}
