use std::collections::HashSet;

use dioxus::prelude::*;
use dioxus_radio::prelude::use_radio;
use freya_components::*;
use freya_native_core::NodeId;
use freya_router::prelude::{
    router,
    use_navigator,
};

use crate::{
    node::NodeElement,
    state::DevtoolsChannel,
    Route,
};

#[derive(Clone, PartialEq)]
struct NodeTreeItem {
    is_open: Option<bool>,
    node_id: NodeId,
}

#[allow(non_snake_case)]
#[component]
pub fn NodesTree(selected_node_id: Option<NodeId>, onselected: EventHandler<NodeId>) -> Element {
    let navigator = use_navigator();
    let mut radio = use_radio(DevtoolsChannel::UpdatedDOM);

    let items = {
        let radio = radio.read();
        let devtools_receiver = radio.devtools_receiver.borrow();
        let mut allowed_nodes = HashSet::new();
        devtools_receiver
            .iter()
            .enumerate()
            .filter_map(|(i, node)| {
                let parent_is_open = node
                    .parent_id
                    .map(|id| allowed_nodes.contains(&id) && radio.devtools_tree.contains(&id))
                    .unwrap_or(true);
                let is_root = i == 0;
                if parent_is_open || is_root {
                    allowed_nodes.insert(node.id);
                    let is_open =
                        (node.children_len != 0).then_some(radio.devtools_tree.contains(&node.id));
                    Some(NodeTreeItem {
                        is_open,
                        node_id: node.id,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };

    rsx!(VirtualScrollView {
        show_scrollbar: true,
        length: items.len(),
        item_size: 27.0,
        builder_args: (selected_node_id, items),
        builder: move |i, options: &Option<(Option<NodeId>, Vec<NodeTreeItem>)>| {
            let (selected_node_id, items) = options.as_ref().unwrap();
            let item = &items[i];
            let node_id = item.node_id;
            to_owned![onselected];
            rsx! {
                NodeElement {
                    key: "{node_id:?}",
                    is_selected: Some(node_id) == *selected_node_id,
                    is_open: item.is_open,
                    onarrow: move |_| {
                        let mut radio = radio.write();
                        if radio.devtools_tree.contains(&node_id) {
                            radio.devtools_tree.remove(&node_id);
                        } else {
                            radio.devtools_tree.insert(node_id);
                        }
                    },
                    onselected: move |_| {
                        onselected.call(node_id);

                        match router().current() {
                            Route::NodeInspectorLayout { .. } => {
                                navigator.replace(Route::NodeInspectorLayout { node_id: node_id.serialize() });
                            }
                            _ => {
                                navigator.replace(Route::NodeInspectorStyle { node_id: node_id.serialize() });
                            }
                        }
                    },
                    node_id
                }
            }
        }
    })
}
