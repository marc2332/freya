use std::{collections::HashMap, sync::Arc};

use freya_native_core::prelude::SendAnyMap;
use torin::prelude::*;

// Custom measurer, useful to measure certain elements such as text with other libraries
pub struct CustomMeasurer;

impl LayoutMeasurer<usize> for CustomMeasurer {
    fn measure(
        &mut self,
        _node_id: usize,
        _node: &Node,
        _size: &Size2D,
    ) -> Option<(Size2D, Arc<SendAnyMap>)> {
        None
    }

    fn should_measure_inner_children(&mut self, _node_id: usize) -> bool {
        true
    }
}

#[derive(Clone)]
struct DemoNode {
    parent: Option<usize>,
    children: Vec<usize>,
    height: u16,
    node: Node,
}

#[derive(Default)]
pub struct DemoDOM {
    nodes: HashMap<usize, DemoNode>,
}

impl DemoDOM {
    /// Add the Node to the DOM
    pub fn add(&mut self, node_id: usize, parent: Option<usize>, children: Vec<usize>, node: Node) {
        // Get the parent's height in the DOM
        let parent_height = parent
            .map(|p| self.nodes.get(&p).unwrap().height)
            .unwrap_or(0);

        // Assign the node a height just below its parent
        let height = parent_height + 1;

        self.nodes.insert(
            node_id,
            DemoNode {
                parent,
                children,
                height,
                node,
            },
        );
    }

    /// Update a Node
    pub fn set_node(&mut self, node_id: usize, node: Node) {
        self.nodes.get_mut(&node_id).unwrap().node = node;
    }

    // Recursively remove a Node from the DOM
    pub fn remove(&mut self, node_id: usize) {
        let node = self.nodes.get(&node_id).unwrap().clone();

        if let Some(DemoNode { children, .. }) = node.parent.and_then(|p| self.nodes.get_mut(&p)) {
            children.retain(|c| *c != node_id);
        }

        self.nodes.remove(&node_id);

        for child in node.children {
            self.remove(child);
        }
    }
}

impl DOMAdapter<usize> for DemoDOM {
    fn children_of(&mut self, node_id: &usize) -> Vec<usize> {
        self.nodes
            .get(node_id)
            .map(|c| c.children.clone())
            .unwrap_or_default()
    }

    fn parent_of(&self, node_id: &usize) -> Option<usize> {
        self.nodes.get(node_id).and_then(|c| c.parent)
    }

    fn height(&self, node_id: &usize) -> Option<u16> {
        self.nodes.get(node_id).map(|c| c.height)
    }

    fn get_node(&self, node_id: &usize) -> Option<Node> {
        self.nodes.get(node_id).map(|c| c.node.clone())
    }

    fn is_node_valid(&mut self, _node_id: &usize) -> bool {
        // We assume all the nodes in this Demo DOM are actually available for measurement
        // This could not be the case in certain implementations of DOMs, for example Dioxus has a concept of
        // Placeholders, which are elements in the DOM but are not to be measured
        true
    }

    fn root_id(&self) -> usize {
        0 // We assume 0 is the root ID of the DOM
    }
}

fn main() {
    let mut layout = Torin::<usize>::new();
    let mut measurer = Some(CustomMeasurer);

    let mut demo_dom = DemoDOM::default();

    // Node A: Root Node
    demo_dom.add(
        0,       // ID
        None,    // Parent ID
        vec![1], // Children IDs
        Node::from_size_and_alignments_and_direction(
            Size::Pixels(Length::new(200.0)),
            Size::Pixels(Length::new(200.0)),
            Alignment::Center,
            Alignment::Center,
            DirectionMode::Horizontal,
        ),
    );

    // Node B: Child of the Root Node
    demo_dom.add(
        1,       // ID
        Some(0), // Parent ID
        vec![2], // Children IDs
        Node::from_size_and_direction(
            Size::Pixels(Length::new(100.0)),
            Size::Pixels(Length::new(100.0)),
            DirectionMode::Vertical,
        ),
    );

    // Node C: Child of Node B
    demo_dom.add(
        2,       // ID
        Some(1), // Parent ID
        vec![],  // Children IDs
        Node::from_size_and_direction(
            Size::Percentage(Length::new(50.0)),
            Size::Percentage(Length::new(50.0)),
            DirectionMode::Vertical,
        ),
    );

    // Measure our DOM layout
    layout.measure(
        0,                                                              // Root ID
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)), // Available Area
        &mut measurer,
        &mut demo_dom,
    );

    // Mutate the Node B
    demo_dom.set_node(
        1, // ID
        Node::from_size_and_direction(
            Size::Percentage(Length::new(80.0)), // We change this from 50% to 80%
            Size::Percentage(Length::new(80.0)), // We change this from 50% to 80%
            DirectionMode::Vertical,
        ),
    );
    layout.invalidate(1);

    println!("Initial measurement");
    for (id, node) in &layout.results {
        println!("{id:?} -> {:?}", node.area);
    }

    // Make Torin calculate from what Node it is the most efficiente to start measuring again
    layout.find_best_root(&mut demo_dom);

    // If Torin wasn't able to find a Root candidate, it will just use the ID we pass as fist argument
    layout.measure(
        0,                                                              // Fallback Root ID
        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)), // Available Area
        &mut measurer,
        &mut demo_dom,
    );

    println!("\nSecond measurement");
    for (id, node) in &layout.results {
        println!("{id:?} -> {:?}", node.area);
    }
}
