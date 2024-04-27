use std::sync::Arc;

use dioxus_core::{prelude::spawn, use_hook, AttributeValue};
use dioxus_signals::{ReadOnlySignal, Readable, Signal, Writable};
use freya_common::NodeReferenceLayout;
use freya_node_state::{CustomAttributeValues, NodeReference};
use tokio::sync::watch::channel;

/// Subscribe to a Node layout changes.
pub fn use_node() -> (AttributeValue, NodeReferenceLayout) {
    let (tx, signal) = use_hook(|| {
        let (tx, mut rx) = channel::<NodeReferenceLayout>(NodeReferenceLayout::default());
        let mut signal = Signal::new(NodeReferenceLayout::default());

        spawn(async move {
            while rx.changed().await.is_ok() {
                if *signal.peek() != *rx.borrow() {
                    signal.set(rx.borrow().clone());
                }
            }
        });

        (Arc::new(tx), signal)
    });

    (
        AttributeValue::any_value(CustomAttributeValues::Reference(NodeReference(tx))),
        signal.read_unchecked().clone(),
    )
}

/// Get a signal to read the latest layout from a Node.
pub fn use_node_signal() -> (AttributeValue, ReadOnlySignal<NodeReferenceLayout>) {
    let (tx, signal) = use_hook(|| {
        let (tx, mut rx) = channel::<NodeReferenceLayout>(NodeReferenceLayout::default());
        let mut signal = Signal::new(NodeReferenceLayout::default());

        spawn(async move {
            while rx.changed().await.is_ok() {
                if *signal.peek() != *rx.borrow() {
                    signal.set(rx.borrow().clone());
                }
            }
        });

        (Arc::new(tx), signal)
    });

    (
        AttributeValue::any_value(CustomAttributeValues::Reference(NodeReference(tx))),
        signal.into(),
    )
}

#[cfg(test)]
mod test {
    use crate::use_node;
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn track_size() {
        fn use_node_app() -> Element {
            let (reference, size) = use_node();

            rsx!(
                rect {
                    reference: reference,
                    width: "50%",
                    height: "25%",
                    label {
                        "{size.area.width()}"
                    }
                }
            )
        }

        let mut utils = launch_test_with_config(
            use_node_app,
            TestingConfig {
                size: (500.0, 800.0).into(),
                ..TestingConfig::default()
            },
        );

        utils.wait_for_update().await;
        let root = utils.root().get(0);
        assert_eq!(
            root.get(0).get(0).text().unwrap().parse::<f32>(),
            Ok(500.0 * 0.5)
        );

        utils.config().size = (300.0, 800.0).into();
        utils.wait_for_update().await;

        let root = utils.root().get(0);
        assert_eq!(
            root.get(0).get(0).text().unwrap().parse::<f32>(),
            Ok(300.0 * 0.5)
        );
    }
}
