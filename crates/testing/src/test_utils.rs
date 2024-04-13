use freya_core::prelude::*;
use freya_native_core::real_dom::NodeImmutable;
use freya_native_core::tree::TreeRef;
use freya_native_core::NodeId;

use crate::test_node::TestNode;

#[derive(Clone)]
pub struct TestUtils {
    pub(crate) sdom: SafeDOM,
}

impl TestUtils {
    /// Get the SafeDOM.
    pub fn sdom(&self) -> &SafeDOM {
        &self.sdom
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

    /// Get a list of Nodes matching the text
    pub fn get_node_matching_inside_id(
        &self,
        node_id: NodeId,
        matcher: impl Fn(&DioxusNode) -> bool,
    ) -> Vec<TestNode> {
        fn traverse_dom(rdom: &DioxusDOM, start_id: NodeId, mut f: impl FnMut(&DioxusNode)) {
            let mut stack = vec![start_id];
            let tree = rdom.tree_ref();
            while let Some(id) = stack.pop() {
                if let Some(node) = rdom.get(id) {
                    f(&node);
                    let children = tree.children_ids_advanced(id, true);
                    stack.extend(children.iter().copied().rev());
                }
            }
        }

        let dom = self.sdom.get();
        let rdom = dom.rdom();

        let mut nodes = Vec::default();

        traverse_dom(rdom, node_id, |node| {
            if matcher(node) {
                let utils = self.clone();

                let height = node.height();
                let children_ids = node.child_ids();

                let state = get_node_state(node);
                let node_type = node.node_type().clone();

                nodes.push(TestNode {
                    node_id,
                    utils,
                    children_ids,
                    height,
                    state,
                    node_type,
                });
            }
        });

        nodes
    }
}
