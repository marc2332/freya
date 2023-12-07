use dioxus::prelude::*;
use dioxus_native_core::NodeId;
use dioxus_router::prelude::use_navigator;
use freya_components::*;

use crate::{node::NodeElement, NodeIdSerializer, Route, TreeNode};

#[allow(non_snake_case)]
#[component]
pub fn NodesTree<'a>(
    cx: Scope<'a>,
    height: &'a str,
    selected_node_id: Option<NodeId>,
    onselected: EventHandler<'a, &'a TreeNode>,
) -> Element<'a> {
    let router = use_navigator(cx);
    let nodes = use_shared_state::<Vec<TreeNode>>(cx).unwrap();

    render!(VirtualScrollView {
        width: "100%",
        height: "{height}",
        padding: "15",
        show_scrollbar: true,
        length: nodes.read().len(),
        item_size: 27.0,
        builder_values: (nodes, selected_node_id, onselected, router),
        builder: Box::new(move |(_k, i, _, values)| {
            let (nodes, selected_node_id, onselected, router) = values.unwrap();
            let nodes = nodes.read();
            let node = nodes.get(i).cloned().unwrap();
            rsx! {
                NodeElement {
                    key: "{node.id:?}",
                    is_selected: Some(node.id) == *selected_node_id,
                    onselected: |node: &TreeNode| {
                        onselected.call(node);
                        router.replace(Route::TreeStyleTab { node_id: node.id.serialize() });
                    },
                    node: node
                }
            }
        })
    })
}
