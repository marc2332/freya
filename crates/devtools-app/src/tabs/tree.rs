use std::collections::HashSet;

use dioxus_radio::prelude::use_radio;
use freya::prelude::*;
use freya_native_core::NodeId;
use freya_router::prelude::{
    router,
    use_navigator,
};

use crate::{
    Route,
    node::NodeElement,
    state::DevtoolsChannel,
};

#[derive(Clone, PartialEq)]
struct NodeTreeItem {
    is_open: Option<bool>,
    window_id: u64,
    node_id: NodeId,
}

#[allow(non_snake_case)]
#[component]
pub fn NodesTree(
    selected_node_id: Option<NodeId>,
    selected_window_id: Option<u64>,
    onselected: EventHandler<NodeId>,
) -> Element {
    let navigator = use_navigator();
    let mut radio = use_radio(DevtoolsChannel::UpdatedDOM);

    let items = {
        let radio = radio.read();
        radio
            .nodes
            .iter()
            .flat_map(|(window_id, nodes)| {
                let mut allowed_nodes = HashSet::new();
                nodes
                    .iter()
                    .filter_map(|node| {
                        let parent_is_open = node
                            .parent_id
                            .map(|node_id| {
                                allowed_nodes.contains(&node_id)
                                    && radio.expanded_nodes.contains(&(*window_id, node_id))
                            })
                            .unwrap_or(false);
                        let is_top_height = node.height == 1;
                        if parent_is_open || is_top_height {
                            allowed_nodes.insert(node.node_id);
                            let is_open = (node.children_len != 0).then_some(
                                radio.expanded_nodes.contains(&(*window_id, node.node_id)),
                            );
                            Some(NodeTreeItem {
                                is_open,
                                node_id: node.node_id,
                                window_id: *window_id,
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };

    rsx!(VirtualScrollView {
        show_scrollbar: true,
        length: items.len(),
        item_size: 27.0,
        builder_args: (selected_node_id, selected_window_id, items),
        builder: move |i, options: &Option<(Option<NodeId>, Option<u64>, Vec<NodeTreeItem>)>| {
            let (selected_node_id, selected_window_id, items) = options.as_ref().unwrap();
            let NodeTreeItem {
                window_id,
                node_id,
                is_open,
            } = items[i];
            to_owned![onselected];
            rsx! {
                NodeElement {
                    key: "{node_id:?}-{window_id}",
                    is_selected: Some(node_id) == *selected_node_id && Some(window_id) == *selected_window_id,
                    is_open: is_open,
                    onarrow: move |_| {
                        let mut radio = radio.write();
                        if radio.expanded_nodes.contains(&(window_id, node_id)) {
                            radio.expanded_nodes.remove(&(window_id, node_id));
                        } else {
                            radio.expanded_nodes.insert((window_id, node_id));
                        }
                    },
                    onselected: move |_| {
                        onselected.call(node_id);

                        match router().current() {
                            Route::NodeInspectorComputedLayout { .. } => {
                                navigator.replace(Route::NodeInspectorComputedLayout { node_id, window_id });
                            }
                            _ => {
                                navigator.replace(Route::NodeInspectorLayout { node_id, window_id });
                            }
                        }
                    },
                    node_id,
                    window_id
                }
            }
        }
    })
}
