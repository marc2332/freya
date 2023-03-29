use std::collections::HashSet;

use dioxus_native_core::{
    node::{Node, NodeData, NodeType},
    real_dom::RealDom,
    tree::TreeLike,
    NodeId,
};
use freya_common::NodeArea;
use freya_dom::{DioxusNode, FreyaDOM};
use freya_layout::Layers;
use freya_layout::NodeLayoutMeasurer;
use freya_node_state::{DirectionMode, NodeState, Size, SizeMode};
use lazy_static::lazy_static;
use rustc_hash::FxHashMap;
use skia_safe::textlayout::FontCollection;

lazy_static! {
    static ref TEST_NODE: DioxusNode = Node {
        node_data: NodeData {
            node_id: NodeId(1),
            element_id: None,
            node_type: NodeType::Element {
                tag: "rect".to_string(),
                namespace: None,
                attributes: FxHashMap::default(),
                listeners: HashSet::default(),
            }
        },
        state: NodeState::default(),
    };
}

#[test]
fn percentage() {
    let mut dom = FreyaDOM::new(RealDom::new());

    let mut node = TEST_NODE.clone();
    node.state = node.state.with_size(Size {
        width: SizeMode::Percentage(50.0),
        height: SizeMode::Percentage(25.0),
        ..expanded_size()
    });
    let root = dom.dom_mut().tree.create_node(node.clone());
    dom.dom_mut().tree.add_child(NodeId(0), root);

    let mut remaining_area = NodeArea {
        x: 0.0,
        y: 0.0,
        height: 300.0,
        width: 200.0,
    };
    let mut layers = Layers::default();
    let mut fonts = FontCollection::new();
    let mut measurer = NodeLayoutMeasurer::new(
        &node,
        &mut remaining_area,
        NodeArea {
            x: 0.0,
            y: 0.0,
            height: 300.0,
            width: 200.0,
        },
        &dom,
        &mut layers,
        0,
        &mut fonts,
    );
    let result = measurer.measure_area(true);

    assert_eq!(result.height, 75.0);
    assert_eq!(result.width, 100.0);
}

#[test]
fn manual() {
    let mut dom = FreyaDOM::new(RealDom::new());
    let mut node = TEST_NODE.clone();
    node.state = node.state.with_size(Size {
        width: SizeMode::Manual(250.0),
        height: SizeMode::Manual(150.0),
        ..expanded_size()
    });

    let root = dom.dom_mut().tree.create_node(node.clone());
    dom.dom_mut().tree.add_child(NodeId(0), root);

    let mut remaining_area = NodeArea {
        x: 0.0,
        y: 0.0,
        height: 300.0,
        width: 200.0,
    };
    let mut layers = Layers::default();
    let mut fonts = FontCollection::new();
    let mut measurer = NodeLayoutMeasurer::new(
        &node,
        &mut remaining_area,
        NodeArea {
            x: 0.0,
            y: 0.0,
            height: 300.0,
            width: 200.0,
        },
        &dom,
        &mut layers,
        0,
        &mut fonts,
    );
    let result = measurer.measure_area(true);

    assert_eq!(result.height, 150.0);
    assert_eq!(result.width, 250.0);
}

#[test]
fn auto() {
    let mut dom = FreyaDOM::new(RealDom::new());
    let node = Node {
        node_data: NodeData {
            node_id: NodeId(1),
            element_id: None,
            node_type: NodeType::Element {
                tag: "rect".to_string(),
                namespace: None,
                attributes: FxHashMap::default(),
                listeners: HashSet::default(),
            },
        },
        state: NodeState::default().with_size(Size {
            width: SizeMode::Auto,
            height: SizeMode::Auto,
            direction: DirectionMode::Both,
            ..expanded_size()
        }),
    };
    let root = dom.dom_mut().tree.create_node(node.clone());
    dom.dom_mut().tree.add_child(NodeId(0), root);

    let root_child = Node {
        node_data: NodeData {
            node_id: NodeId(2),
            element_id: None,
            node_type: NodeType::Element {
                tag: "rect".to_string(),
                namespace: None,
                attributes: FxHashMap::default(),
                listeners: HashSet::default(),
            },
        },
        state: NodeState::default().with_size(Size {
            width: SizeMode::Manual(170.0),
            height: SizeMode::Manual(25.0),
            ..expanded_size()
        }),
    };

    let root_child = dom.dom_mut().tree.create_node(root_child);
    dom.dom_mut().tree.add_child(root, root_child);

    let mut remaining_area = NodeArea {
        x: 0.0,
        y: 0.0,
        height: 300.0,
        width: 200.0,
    };
    let mut layers = Layers::default();
    let mut fonts = FontCollection::new();
    let mut measurer = NodeLayoutMeasurer::new(
        &node,
        &mut remaining_area,
        NodeArea {
            x: 0.0,
            y: 0.0,
            height: 300.0,
            width: 200.0,
        },
        &dom,
        &mut layers,
        0,
        &mut fonts,
    );
    let result = measurer.measure_area(true);

    assert_eq!(result.height, 25.0);
    assert_eq!(result.width, 170.0);
}

#[test]
fn x_y() {
    let mut dom = FreyaDOM::new(RealDom::new());
    let mut node = TEST_NODE.clone();
    node.state = node.state.with_size(Size {
        width: SizeMode::Manual(250.0),
        height: SizeMode::Manual(150.0),
        ..expanded_size()
    });
    let mut remaining_area = NodeArea {
        x: 15.0,
        y: 25.0,
        height: 300.0,
        width: 200.0,
    };

    let root = dom.dom_mut().tree.create_node(node.clone());
    dom.dom_mut().tree.add_child(NodeId(0), root);

    let root_child = Node {
        node_data: NodeData {
            node_id: NodeId(2),
            element_id: None,
            node_type: NodeType::Element {
                tag: "rect".to_string(),
                namespace: None,
                attributes: FxHashMap::default(),
                listeners: HashSet::default(),
            },
        },
        state: NodeState::default().with_size(Size {
            width: SizeMode::Manual(170.0),
            height: SizeMode::Manual(25.0),
            ..expanded_size()
        }),
    };

    let root_child = dom.dom_mut().tree.create_node(root_child);
    dom.dom_mut().tree.add_child(root, root_child);

    let mut layers = Layers::default();
    let mut fonts = FontCollection::new();
    let mut measurer = NodeLayoutMeasurer::new(
        &node,
        &mut remaining_area,
        NodeArea {
            x: 15.0,
            y: 25.0,
            height: 300.0,
            width: 200.0,
        },
        &dom,
        &mut layers,
        0,
        &mut fonts,
    );

    let result = measurer.measure_area(true);

    assert_eq!(result.x, 15.0);
    assert_eq!(result.y, 25.0);
}

fn expanded_size() -> Size {
    Size {
        width: SizeMode::Percentage(100.0),
        height: SizeMode::Percentage(100.0),
        min_height: SizeMode::Manual(0.0),
        min_width: SizeMode::Manual(0.0),
        max_height: SizeMode::Auto,
        max_width: SizeMode::Auto,
        padding: (0.0, 0.0, 0.0, 0.0),
        direction: DirectionMode::Both,
    }
}
