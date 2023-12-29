use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::tree::TreeRef;
use dioxus_native_core::NodeId;
use freya_core::prelude::*;
use freya_dom::prelude::{DioxusNode, SafeDOM};
use std::sync::{Arc, Mutex};

use crate::test_node::TestNode;

#[derive(Clone)]
pub struct TestUtils {
    pub(crate) sdom: SafeDOM,
    pub(crate) layers: Arc<Mutex<Layers>>,
}

impl TestUtils {
    pub fn sdom(&self) -> &SafeDOM {
        &self.sdom
    }

    pub fn layers(&self) -> &Arc<Mutex<Layers>> {
        &self.layers
    }

    /// Get a Node by the given ID
    pub fn get_node_by_id(&self, node_id: NodeId) -> TestNode {
        let utils = self.clone();

        let dom = self.sdom.get();
        let rdom = dom.rdom();

        let height = rdom.tree_ref().height(node_id).unwrap();
        let children_ids = rdom.tree_ref().children_ids(node_id);
        let node: DioxusNode = rdom.get(node_id).unwrap();

        let state = get_node_state(&node);
        let node_type = node.node_type().clone();

        TestNode {
            node_id,
            utils,
            children_ids,
            height,
            state,
            node_type,
        }
    }
}
