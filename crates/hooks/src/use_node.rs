use dioxus_core::{prelude::spawn, use_hook, AttributeValue};
use dioxus_hooks::use_signal;
use dioxus_signals::{Readable, Signal, Writable};
use freya_common::NodeReferenceLayout;
use freya_node_state::{CustomAttributeValues, NodeReference};
use tokio::sync::mpsc::unbounded_channel;

/// Subscribe to a Node layout changes.
pub fn use_node() -> (AttributeValue, NodeReferenceLayout) {
    let mut layout = use_signal::<NodeReferenceLayout>(NodeReferenceLayout::default);

    let tx = use_hook(|| {
        let (tx, mut rx) = unbounded_channel::<NodeReferenceLayout>();

        spawn(async move {
            while let Some(new_layout) = rx.recv().await {
                if *layout.peek() != new_layout {
                    layout.set(new_layout);
                }
            }
        });

        tx
    });

    (
        AttributeValue::any_value(CustomAttributeValues::Reference(NodeReference(tx.clone()))),
        layout.read().clone(),
    )
}

/// Get a signal to read the latest layout from a Node.
pub fn use_node_signal() -> (AttributeValue, Signal<NodeReferenceLayout>) {
    let mut layout = use_signal::<NodeReferenceLayout>(NodeReferenceLayout::default);

    let tx = use_hook(|| {
        let (tx, mut rx) = unbounded_channel::<NodeReferenceLayout>();

        spawn(async move {
            while let Some(new_layout) = rx.recv().await {
                if *layout.peek() != new_layout {
                    layout.set(new_layout);
                }
            }
        });

        tx
    });

    (
        AttributeValue::any_value(CustomAttributeValues::Reference(NodeReference(tx.clone()))),
        layout,
    )
}

#[cfg(test)]
mod test {
    use crate::use_node;
    use freya::prelude::*;
    use freya_testing::{launch_test_with_config, TestingConfig};

    #[tokio::test]
    pub async fn track_size() {
        fn use_node_app() -> Element {
            let (reference, size) = use_node();

            rsx!(
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
