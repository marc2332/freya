use dioxus_core::{AttributeValue, ScopeState};
use dioxus_hooks::{to_owned, use_ref, use_state, UseRef};
use freya_common::NodeReferenceLayout;
use freya_node_state::{CustomAttributeValues, NodeReference};
use tokio::sync::mpsc::unbounded_channel;

/// Subscribe to a Node layout changes.
pub fn use_node(cx: &ScopeState) -> (AttributeValue, NodeReferenceLayout) {
    let status = use_state::<NodeReferenceLayout>(cx, NodeReferenceLayout::default);

    let tx = cx.use_hook(|| {
        let (tx, mut rx) = unbounded_channel::<NodeReferenceLayout>();

        to_owned![status];
        cx.spawn(async move {
            while let Some(new_status) = rx.recv().await {
                if status.current().as_ref() != &new_status {
                    status.set(new_status);
                }
            }
        });

        tx
    });

    (
        cx.any_value(CustomAttributeValues::Reference(NodeReference(tx.clone()))),
        status.get().clone(),
    )
}

/// Silently read the latest layout from a Node.
pub fn use_node_ref(cx: &ScopeState) -> (AttributeValue, &UseRef<NodeReferenceLayout>) {
    let status = use_ref::<NodeReferenceLayout>(cx, NodeReferenceLayout::default);

    let tx = cx.use_hook(|| {
        let (tx, mut rx) = unbounded_channel::<NodeReferenceLayout>();

        to_owned![status];
        cx.spawn(async move {
            while let Some(new_status) = rx.recv().await {
                if *status.read() != new_status {
                    *status.write_silent() = new_status;
                }
            }
        });

        tx
    });

    (
        cx.any_value(CustomAttributeValues::Reference(NodeReference(tx.clone()))),
        status,
    )
}

#[cfg(test)]
mod test {
    use crate::use_node;
    use freya::prelude::*;
    use freya_testing::{launch_test_with_config, TestingConfig};

    #[tokio::test]
    pub async fn track_size() {
        fn use_node_app(cx: Scope) -> Element {
            let (reference, size) = use_node(cx);

            render!(
                rect {
                    reference: reference,
                    width: "50%",
                    height: "25%",
                    "{size.area.width()}"
                }
            )
        }

        let mut utils = launch_test_with_config(
            use_node_app,
            *TestingConfig::default().with_size((500.0, 800.0).into()),
        );

        utils.wait_for_update().await;
        let root = utils.root().get(0);
        assert_eq!(root.get(0).text().unwrap().parse::<f32>(), Ok(500.0 * 0.5));

        utils.config().with_size((300.0, 800.0).into());
        utils.wait_for_update().await;

        let root = utils.root().get(0);
        assert_eq!(root.get(0).text().unwrap().parse::<f32>(), Ok(300.0 * 0.5));
    }
}
