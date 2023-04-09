use dioxus_core::{AttributeValue, ScopeState};
use dioxus_hooks::{use_effect, use_state};
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
            TestingConfig::default().with_size((500.0, 800.0).into()),
        );

        utils.wait_for_update().await;
        let root = utils.root().child(0).unwrap();
        assert_eq!(
            root.child(0).unwrap().text().unwrap().parse::<f32>(),
            Ok(500.0 * 0.5)
        );

        utils.set_config(TestingConfig::default().with_size((300.0, 800.0).into()));
        utils.wait_for_update().await;

        let root = utils.root().child(0).unwrap();
        assert_eq!(
            root.child(0).unwrap().text().unwrap().parse::<f32>(),
            Ok(300.0 * 0.5)
        );
    }
}
