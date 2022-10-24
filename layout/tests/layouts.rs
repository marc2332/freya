use std::sync::{Arc, Mutex};

use dioxus_core::ElementId;
use dioxus_native_core::real_dom::{Node, NodeType};
use freya_layers::{Layers, NodeData};
use freya_layout::calculate_node;
use freya_layout_memo::{LayoutMemorizer, NodeArea};
use freya_node_state::node::{DirectionMode, NodeState, Size, SizeMode};
use lazy_static::lazy_static;
use skia_safe::textlayout::FontCollection;

lazy_static! {
    static ref TEST_NODE: Node<NodeState> = Node {
        id: ElementId(0),
        parent: None,
        state: NodeState::default(),
        node_type: NodeType::Element {
            tag: "rect".to_string(),
            namespace: None,
            children: Vec::new()
        },
        height: 0,
    };
}

#[test]
fn percentage() {
    let mut node = TEST_NODE.clone();
    node.state = node.state.set_size(Size {
        width: SizeMode::Percentage(50.0),
        height: SizeMode::Percentage(25.0),
        ..Size::expanded()
    });
    let result = calculate_node(
        &NodeData { node },
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
    node.state = node.state.set_size(Size {
        width: SizeMode::Manual(250.0),
        height: SizeMode::Manual(150.0),
        ..Size::expanded()
    });
    let result = calculate_node(
        &NodeData { node },
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
    let result = calculate_node(
        &NodeData {
            node: Node {
                id: ElementId(0),
                parent: None,
                state: NodeState::default().set_size(Size {
                    width: SizeMode::Auto,
                    height: SizeMode::Auto,
                    direction: DirectionMode::Both,
                    ..Size::expanded()
                }),
                node_type: NodeType::Element {
                    tag: "rect".to_string(),
                    namespace: None,
                    children: vec![ElementId(1)],
                },
                height: 0,
            },
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
            Some(NodeData {
                node: Node {
                    id: ElementId(1),
                    parent: None,
                    state: NodeState::default().set_size(Size {
                        width: SizeMode::Manual(170.0),
                        height: SizeMode::Manual(25.0),
                        ..Size::expanded()
                    }),
                    node_type: NodeType::Element {
                        tag: "rect".to_string(),
                        namespace: None,
                        children: Vec::new(),
                    },
                    height: 0,
                },
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
    node.state = node.state.set_size(Size {
        width: SizeMode::Auto,
        height: SizeMode::Auto,
        ..Size::expanded()
    });
    let result = calculate_node(
        &NodeData { node },
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
            Some(NodeData {
                node: Node {
                    id: ElementId(1),
                    parent: None,
                    state: NodeState::default().set_size(Size {
                        width: SizeMode::Manual(170.0),
                        height: SizeMode::Manual(25.0),
                        ..Size::expanded()
                    }),
                    node_type: NodeType::Element {
                        tag: "rect".to_string(),
                        namespace: None,
                        children: Vec::new(),
                    },
                    height: 0,
                },
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
