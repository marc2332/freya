#![allow(clippy::missing_panics_doc)]

use std::collections::HashMap;

use crate::prelude::*;

#[derive(Default)]
pub struct TestingDOM {
    mapper: HashMap<usize, (Option<usize>, Vec<usize>, u16, Node)>,
}

impl TestingDOM {
    pub fn add(&mut self, node_id: usize, parent: Option<usize>, children: Vec<usize>, node: Node) {
        let depth = parent.map_or(0, |p| self.mapper.get(&p).unwrap().2) + 1;
        self.mapper.insert(node_id, (parent, children, depth, node));
    }

    pub fn set_node(&mut self, node_id: usize, node: Node) {
        self.mapper.get_mut(&node_id).unwrap().3 = node;
    }

    pub fn remove(&mut self, node_id: usize) {
        let node = self.mapper.remove(&node_id).unwrap();

        if let Some((_, parent_children, _, _)) = node.0.and_then(|p| self.mapper.get_mut(&p)) {
            parent_children.retain(|c| *c != node_id);
        }

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

pub fn test_utils() -> (Torin<usize>, Option<NoopMeasurer>) {
    let layout = Torin::<usize>::new();
    let measurer = None::<NoopMeasurer>;

    (layout, measurer)
}
