use freya_core::prelude::{get_node_state, FreyaDOM, NodeState};
use freya_native_core::{
    prelude::{NodeId, NodeImmutable},
    tags::TagName,
};
use tokio::sync::watch;
use torin::prelude::LayoutNode;

pub type DevtoolsReceiver = watch::Receiver<Vec<NodeInfo>>;

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
                let has_layout = layout.get(node.id()).is_some();
                if has_layout {
                    let node_type = node.node_type();
                    new_nodes.push(NodeInfo {
                        id: node.id(),
                        tag: *node_type.tag().unwrap(),
                        height: node.height(),
                        state: get_node_state(&node),
                        layout_node: layout.get(node.id()).unwrap().clone(),
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
    pub tag: TagName,
    pub height: u16,
    pub state: NodeState,
    pub layout_node: LayoutNode,
}
