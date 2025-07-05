use std::sync::{
    Arc,
    Mutex,
};

use freya_core::{
    accessibility::NodeAccessibility,
    dom::DioxusDOM,
    node::{
        get_node_state,
        NodeState,
    },
    types::AccessibilityId,
};
use freya_native_core::{
    prelude::{
        NodeId,
        NodeImmutable,
    },
    tags::TagName,
};
use tokio::sync::watch;
use torin::{
    prelude::LayoutNode,
    torin::Torin,
};

pub type DevtoolsReceiver = watch::Receiver<Vec<NodeInfo>>;
pub type HighlightedNode = Arc<Mutex<Option<NodeId>>>;

#[derive(Clone)]
pub struct Devtools {
    sender: watch::Sender<Vec<NodeInfo>>,
    pub highlighted_node: HighlightedNode,
}

impl Devtools {
    pub fn new() -> (Self, DevtoolsReceiver, HighlightedNode) {
        let (sender, receiver) = watch::channel(Vec::new());
        let highlighted_node = HighlightedNode::default();

        (
            Self {
                sender,
                highlighted_node: highlighted_node.clone(),
            },
            receiver,
            highlighted_node,
        )
    }

    pub fn update(&self, rdom: &DioxusDOM, layout: &Torin<NodeId>) {
        let mut new_nodes = Vec::new();

        let mut devtools_found = false;

        rdom.traverse_depth_first(|node| {
            let accessibility_id = node.get_accessibility_id();
            if matches!(accessibility_id, Some(AccessibilityId(u64::MAX))) {
                devtools_found = true;
            }

            if devtools_found {
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

#[derive(Clone, PartialEq, Debug)]
pub struct NodeInfo {
    pub id: NodeId,
    pub parent_id: Option<NodeId>,
    pub children_len: usize,
    pub tag: TagName,
    pub height: u16,
    pub state: NodeState,
    pub layout_node: LayoutNode,
}
