use dioxus_core::{AttributeValue, ScopeState};
use dioxus_hooks::{use_effect, use_ref, use_state, UseRef};
use freya_common::NodeReferenceLayout;
use freya_node_state::{CustomAttributeValues, NodeReference};
use tokio::sync::mpsc::unbounded_channel;

/// Subscribe to a Node layout changes.
pub fn use_node(cx: &ScopeState) -> (AttributeValue, NodeReferenceLayout) {
    let status = use_state::<NodeReferenceLayout>(cx, NodeReferenceLayout::default);

    let channel = cx.use_hook(|| {
        let (tx, rx) = unbounded_channel::<NodeReferenceLayout>();
        (tx, Some(rx))
    });

    let node_ref = NodeReference(channel.0.clone());

    use_effect(cx, (), move |_| {
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

    (
        cx.any_value(CustomAttributeValues::Reference(node_ref)),
        status.get().clone(),
    )
}

/// Silently read the latest layout from a Node.
pub fn use_node_ref(cx: &ScopeState) -> (AttributeValue, &UseRef<NodeReferenceLayout>) {
    let status = use_ref::<NodeReferenceLayout>(cx, NodeReferenceLayout::default);

    let channel = cx.use_hook(|| {
        let (tx, rx) = unbounded_channel::<NodeReferenceLayout>();
        (tx, Some(rx))
    });

    let node_ref = NodeReference(channel.0.clone());

    use_effect(cx, (), move |_| {
        let rx = channel.1.take();
        let status = status.clone();
        cx.spawn(async move {
            let mut rx = rx.unwrap();
            while let Some(new_status) = rx.recv().await {
                if *status.read() != new_status {
                    *status.write_silent() = new_status;
                }
            }
        });
        async move {}
    });

    (
        cx.any_value(CustomAttributeValues::Reference(node_ref)),
        status,
    )
}

#[cfg(test)]
mod test {
    use crate::use_node;
    use freya::prelude::*;
    use freya_testing::launch_test;

    #[tokio::test]
    pub async fn track_size() {
        fn use_node_app(cx: Scope) -> Element {
            let (reference, size) = use_node(cx);

            render!(
                rect {
                    reference: reference,
                    width: "50%",
                    height: "25%",
                    "{size.width}"
                }
            )
        }

        let mut utils = launch_test(use_node_app);

        utils.wait_for_update((500.0, 800.0)).await;
        let root = utils.root().child(0).unwrap();
        assert_eq!(
            root.child(0).unwrap().text().unwrap().parse::<f32>(),
            Ok(500.0 * 0.5)
        );

        utils.wait_for_update((300.0, 800.0)).await;

        let root = utils.root().child(0).unwrap();
        assert_eq!(
            root.child(0).unwrap().text().unwrap().parse::<f32>(),
            Ok(300.0 * 0.5)
        );
    }
}
