use dioxus::core::ElementId;
use dioxus_core::GlobalNodeId;
use dioxus_native_core::real_dom::{Node, NodeData, NodeType};
use freya_layers::{Layers, NodeArea, NodeInfo};
use freya_layout::calculate_node;
use freya_node_state::node::{DirectionMode, NodeState, Size, SizeMode};
use fxhash::{FxHashMap, FxHashSet};
use lazy_static::lazy_static;
use skia_safe::textlayout::FontCollection;

lazy_static! {
    static ref TEST_NODE: Node<NodeState> = Node {
        state: NodeState::default(),
        node_data: NodeData {
            id: GlobalNodeId::VNodeId(ElementId(0)),
            parent: None,
            node_type: NodeType::Element {
                tag: "rect".to_string(),
                namespace: None,
                children: Vec::new(),
                listeners: FxHashSet::default(),
                attributes: FxHashMap::default()
            },
            height: 0,
        }
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
        &NodeInfo { node },
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
        &NodeInfo { node },
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
    );

    assert_eq!(result.height, 150.0);
    assert_eq!(result.width, 250.0);
}

#[test]
fn auto() {
    let result = calculate_node(
        &NodeInfo {
            node: Node {
                node_data: NodeData {
                    id: GlobalNodeId::VNodeId(ElementId(0)),
                    parent: None,
                    node_type: NodeType::Element {
                        tag: "rect".to_string(),
                        namespace: None,
                        children: vec![GlobalNodeId::VNodeId(ElementId(0))],
                        listeners: FxHashSet::default(),
                        attributes: FxHashMap::default(),
                    },
                    height: 0,
                },
                state: NodeState::default().set_size(Size {
                    width: SizeMode::Auto,
                    height: SizeMode::Auto,
                    direction: DirectionMode::Both,
                    ..Size::expanded()
                }),
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
            Some(NodeInfo {
                node: Node {
                    node_data: NodeData {
                        id: GlobalNodeId::VNodeId(ElementId(0)),
                        parent: None,
                        node_type: NodeType::Element {
                            tag: "rect".to_string(),
                            namespace: None,
                            children: Vec::new(),
                            listeners: FxHashSet::default(),
                            attributes: FxHashMap::default(),
                        },
                        height: 0,
                    },
                    state: NodeState::default().set_size(Size {
                        width: SizeMode::Manual(170.0),
                        height: SizeMode::Manual(25.0),
                        ..Size::expanded()
                    }),
                },
            })
        },
        0,
        &mut FontCollection::new(),
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
        &NodeInfo { node },
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
            Some(NodeInfo {
                node: Node {
                    node_data: NodeData {
                        id: GlobalNodeId::VNodeId(ElementId(1)),
                        parent: None,
                        node_type: NodeType::Element {
                            tag: "rect".to_string(),
                            namespace: None,
                            children: Vec::new(),
                            listeners: FxHashSet::default(),
                            attributes: FxHashMap::default(),
                        },
                        height: 0,
                    },
                    state: NodeState::default().set_size(Size {
                        width: SizeMode::Manual(170.0),
                        height: SizeMode::Manual(25.0),
                        ..Size::expanded()
                    }),
                },
            })
        },
        0,
        &mut FontCollection::new(),
    );

    assert_eq!(result.x, 15.0);
    assert_eq!(result.y, 25.0);
}
