use freya_native_core::SendAnyMap;

use crate::prelude::*;
use std::{collections::HashMap, sync::Arc};

pub struct TestingMeasurer;

impl LayoutMeasurer<usize> for TestingMeasurer {
    fn measure(
        &mut self,
        _node_id: usize,
        _node: &Node,
        _area_size: &Size2D,
    ) -> Option<(Size2D, Arc<SendAnyMap>)> {
        None
    }

    fn should_measure_inner_children(&mut self, _node_id: usize) -> bool {
        true
    }
}

#[derive(Default)]
pub struct TestingDOM {
    mapper: HashMap<usize, (Option<usize>, Vec<usize>, u16, Node)>,
}

impl TestingDOM {
    pub fn add(&mut self, node_id: usize, parent: Option<usize>, children: Vec<usize>, node: Node) {
        let depth = parent.map(|p| self.mapper.get(&p).unwrap().2).unwrap_or(0) + 1;
        self.mapper.insert(node_id, (parent, children, depth, node));
    }

    pub fn set_node(&mut self, node_id: usize, node: Node) {
        self.mapper.get_mut(&node_id).unwrap().3 = node;
    }

    pub fn remove(&mut self, node_id: usize) {
        let node = self.mapper.get(&node_id).unwrap().clone();

        if let Some((_, parent_children, _, _)) = node.0.and_then(|p| self.mapper.get_mut(&p)) {
            parent_children.retain(|c| *c != node_id);
        }

        self.mapper.remove(&node_id);

        for child in node.1 {
            self.remove(child);
        }
    }
}

impl DOMAdapter<usize> for TestingDOM {
    fn children_of(&mut self, node_id: &usize) -> Vec<usize> {
        self.mapper
            .get(node_id)
            .map(|c| c.1.clone())
            .unwrap_or_default()
    }

    fn parent_of(&self, node_id: &usize) -> Option<usize> {
        self.mapper.get(node_id).and_then(|c| c.0)
    }

    fn height(&self, node_id: &usize) -> Option<u16> {
        self.mapper.get(node_id).map(|c| c.2)
    }

    fn get_node(&self, node_id: &usize) -> Option<Node> {
        self.mapper.get(node_id).map(|c| c.3.clone())
    }

    fn is_node_valid(&mut self, _node_id: &usize) -> bool {
        true
    }

    fn root_id(&self) -> usize {
        0
    }
}

pub fn test_utils() -> (Torin<usize>, Option<TestingMeasurer>) {
    let layout = Torin::<usize>::new();
    let measurer = Some(TestingMeasurer);

    (layout, measurer)
}
