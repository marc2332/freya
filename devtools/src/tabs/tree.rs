use dioxus::prelude::*;
use dioxus_native_core::NodeId;
use dioxus_router::*;
use freya_components::*;

use crate::{node::NodeElement, TreeNode};

#[allow(non_snake_case)]
#[inline_props]
pub fn NodesTree<'a>(
    cx: Scope<'a>,
    nodes: &'a Vec<TreeNode>,
    height: &'a str,
    selected_node_id: &'a Option<NodeId>,
    onselected: EventHandler<'a, &'a TreeNode>,
) -> Element<'a> {
    let router = use_router(cx);

    render!(VirtualScrollView {
        width: "100%",
        height: "{height}",
        padding: "15",
        show_scrollbar: true,
        length: nodes.len(),
        item_size: 27.0,
        builder_values: (nodes, selected_node_id, onselected, router),
        builder: Box::new(move |(_k, i, _, values)| {
            let (nodes, selected_node_id, onselected, router) = values.unwrap();
            let node = nodes.get(i).unwrap();
            rsx! {
                NodeElement {
                    key: "{node.id:?}",
                    is_selected: Some(node.id) == **selected_node_id,
                    onselected: |node: &TreeNode| {
                        onselected.call(node);
                        router.replace_route("/elements/style", None, None)
                    },
                    node: node
                }
            }
        })
    })
}
