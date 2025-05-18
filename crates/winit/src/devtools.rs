use std::sync::{
    Arc,
    Mutex,
};

use freya_core::{
    dom::FreyaDOM,
    node::{
        get_node_state,
        NodeState,
    },
};
use freya_native_core::{
    prelude::{
        NodeId,
        NodeImmutable,
    },
    tags::TagName,
};
use tokio::sync::watch;
use torin::prelude::LayoutNode;

pub type DevtoolsReceiver = watch::Receiver<Vec<NodeInfo>>;
pub type HoveredNode = Option<Arc<Mutex<Option<NodeId>>>>;

#[derive(Clone)]
pub struct Devtools {
    sender: watch::Sender<Vec<NodeInfo>>,
}

impl Devtools {
    pub fn new() -> (Self, DevtoolsReceiver) {
        let (sender, receiver) = watch::channel(Vec::new());

        (Self { sender }, receiver)
    }

    pub fn update(&self, fdom: &FreyaDOM) {
        let rdom = fdom.rdom();
        let layout = fdom.layout();

        let mut new_nodes = Vec::new();

        let mut root_found = false;
        let mut devtools_found = false;

        rdom.traverse_depth_first(|node| {
            let height = node.height();
            if height == 3 {
                if !root_found {
                    root_found = true;
                } else {
                    devtools_found = true;
                }
            }

            if !devtools_found && root_found {
                let layout_node = layout.get(node.id()).cloned();
                if let Some(layout_node) = layout_node {
                    let node_type = node.node_type();
                    new_nodes.push(NodeInfo {
                        id: node.id(),
                        parent_id: node.parent_id(),
                        children_len: node
                            .children()
                            .iter()
                            .filter(|node| layout.get(node.id()).is_some())
                            .count(),
                        tag: *node_type.tag().unwrap(),
                        height: node.height(),
                        state: get_node_state(&node),
                        layout_node,
                    });
                }
            }
        });

        self.sender
            .send(new_nodes)
            .expect("Failed to sync the Devtools.");
    }
}

#[derive(Clone, PartialEq)]
pub struct NodeInfo {
    pub id: NodeId,
    pub parent_id: Option<NodeId>,
    pub children_len: usize,
    pub tag: TagName,
    pub height: u16,
    pub state: NodeState,
    pub layout_node: LayoutNode,
}
