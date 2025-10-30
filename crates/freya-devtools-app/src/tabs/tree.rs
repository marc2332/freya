use std::collections::HashSet;

use freya::prelude::*;
use freya_core::integration::NodeId;
use freya_radio::prelude::use_radio;
use freya_router::prelude::{
    Navigator,
    RouterContext,
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

#[derive(PartialEq)]
pub struct NodesTree {
    pub selected_node_id: Option<NodeId>,
    pub selected_window_id: Option<u64>,
    pub on_selected: EventHandler<(u64, NodeId)>,
}

impl Render for NodesTree {
    fn render(&self) -> Element {
        let mut radio = use_radio(DevtoolsChannel::UpdatedTree);

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

        if items.is_empty() {
            return rect()
                .center()
                .expanded()
                .child("Waiting for an app to connect...")
                .into();
        }

        let items_len = items.len() as i32;

        VirtualScrollView::new_with_data(
            (
                self.selected_node_id,
                self.selected_window_id,
                self.on_selected.clone(),
            ),
            move |i, (selected_node_id, selected_window_id, on_selected)| {
                let NodeTreeItem {
                    window_id,
                    node_id,
                    is_open,
                } = items[i];
                let on_selected = on_selected.clone();
                NodeElement {
                    is_selected: Some(node_id) == *selected_node_id
                        && Some(window_id) == *selected_window_id,
                    is_open,
                    on_arrow: EventHandler::new(move |_| {
                        let mut radio = radio.write();
                        if radio.expanded_nodes.contains(&(window_id, node_id)) {
                            radio.expanded_nodes.remove(&(window_id, node_id));
                        } else {
                            radio.expanded_nodes.insert((window_id, node_id));
                        }
                    }),
                    on_selected: EventHandler::new(move |_| {
                        on_selected.call((window_id, node_id));
                        match RouterContext::get().current::<Route>() {
                            Route::NodeInspectorComputedLayout { .. } => {
                                Navigator::get().push(Route::NodeInspectorComputedLayout {
                                    node_id,
                                    window_id,
                                });
                            }
                            Route::NodeInspectorStyle { .. } => {
                                Navigator::get()
                                    .push(Route::NodeInspectorStyle { node_id, window_id });
                            }
                            Route::NodeInspectorTextStyle { .. } => {
                                Navigator::get()
                                    .push(Route::NodeInspectorTextStyle { node_id, window_id });
                            }
                            Route::NodeInspectorLayout { .. } => {
                                Navigator::get()
                                    .push(Route::NodeInspectorLayout { node_id, window_id });
                            }
                            _ => {
                                Navigator::get()
                                    .push(Route::NodeInspectorStyle { node_id, window_id });
                            }
                        }
                    }),
                    node_id,
                    window_id,
                }
                .into()
            },
        )
        .length(items_len)
        .item_size(27.)
        .into()
    }
}
