use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use dioxus_native_core::{
    node::{Node, NodeData, NodeType},
    NodeId,
};
use freya_common::{LayoutMemorizer, NodeArea};
use freya_layers::{Layers, DOMNode};
use freya_layout::measure_node_layout;
use freya_node_state::{DirectionMode, NodeState, Size, SizeMode};
use lazy_static::lazy_static;
use rustc_hash::FxHashMap;
use skia_safe::textlayout::FontCollection;

lazy_static! {
    static ref TEST_NODE: DOMNode = DOMNode {
        node: Node {
            node_data: NodeData {
                node_id: NodeId(0),
                element_id: None,
                node_type: NodeType::Element {
                    tag: "rect".to_string(),
                    namespace: None,
                    attributes: FxHashMap::default(),
                    listeners: HashSet::default(),
                }
            },
            state: NodeState::default(),
        },
        height: 0,
        parent_id: None,
        children: None
    };
}

#[test]
fn percentage() {
    let mut node = TEST_NODE.clone();
    node.node.state = node.node.state.with_size(Size {
        width: SizeMode::Percentage(50.0),
        height: SizeMode::Percentage(25.0),
        ..expanded_size()
    });
    let result = measure_node_layout(
        &node,
        NodeArea {
            x: 0.0,
            y: 0.0,
            height: 300.0,
            width: 200.0,
        },
        NodeArea {
            x: 0.0,
            y: 0.0,
            height: 300.0,
            width: 200.0,
        },
        &mut (),
        &mut Layers::default(),
        |_, _| None,
        0,
        &mut FontCollection::new(),
        &Arc::new(Mutex::new(LayoutMemorizer::new())),
        true,
    );

    assert_eq!(result.height, 75.0);
    assert_eq!(result.width, 100.0);
}

#[test]
fn manual() {
    let mut node = TEST_NODE.clone();
    node.node.state = node.node.state.with_size(Size {
        width: SizeMode::Manual(250.0),
        height: SizeMode::Manual(150.0),
        ..expanded_size()
    });
    let result = measure_node_layout(
        &node,
        NodeArea {
            x: 0.0,
            y: 0.0,
            height: 300.0,
            width: 200.0,
        },
        NodeArea {
            x: 0.0,
            y: 0.0,
            height: 300.0,
            width: 200.0,
        },
        &mut (),
        &mut Layers::default(),
        |_, _| None,
        0,
        &mut FontCollection::new(),
        &Arc::new(Mutex::new(LayoutMemorizer::new())),
        true,
    );

    assert_eq!(result.height, 150.0);
    assert_eq!(result.width, 250.0);
}

#[test]
fn auto() {
    let result = measure_node_layout(
        &DOMNode {
            node: Node {
                node_data: NodeData {
                    node_id: NodeId(0),
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
            },
            height: 0,
            parent_id: None,
            children: Some(vec![NodeId(1)]),
        },
        NodeArea {
            x: 0.0,
            y: 0.0,
            height: 300.0,
            width: 200.0,
        },
        NodeArea {
            x: 0.0,
            y: 0.0,
            height: 300.0,
            width: 200.0,
        },
        &mut (),
        &mut Layers::default(),
        |_, _| {
            Some(DOMNode {
                node: Node {
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
                        width: SizeMode::Manual(170.0),
                        height: SizeMode::Manual(25.0),
                        ..expanded_size()
                    }),
                },
                height: 0,
                parent_id: None,
                children: None,
            })
        },
        0,
        &mut FontCollection::new(),
        &Arc::new(Mutex::new(LayoutMemorizer::new())),
        true,
    );

    assert_eq!(result.height, 25.0);
    assert_eq!(result.width, 170.0);
}

#[test]
fn x_y() {
    let mut node = TEST_NODE.clone();
    node.node.state = node.node.state.with_size(Size {
        width: SizeMode::Manual(250.0),
        height: SizeMode::Manual(150.0),
        ..expanded_size()
    });
    let result = measure_node_layout(
        &node,
        NodeArea {
            x: 15.0,
            y: 25.0,
            height: 300.0,
            width: 200.0,
        },
        NodeArea {
            x: 15.0,
            y: 25.0,
            height: 300.0,
            width: 200.0,
        },
        &mut (),
        &mut Layers::default(),
        |_, _| {
            Some(DOMNode {
                node: Node {
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
                        width: SizeMode::Manual(170.0),
                        height: SizeMode::Manual(25.0),
                        ..expanded_size()
                    }),
                },
                height: 0,
                parent_id: None,
                children: None,
            })
        },
        0,
        &mut FontCollection::new(),
        &Arc::new(Mutex::new(LayoutMemorizer::new())),
        true,
    );

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
