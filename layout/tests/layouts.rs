use std::collections::{HashMap, HashSet};

use dioxus_native_core::real_dom::NodeMut;
use dioxus_native_core::{node::NodeType, prelude::ElementNode, real_dom::NodeImmutable};
use freya_common::Area;
use freya_dom::FreyaDOM;
use freya_layout::Layers;
use freya_layout::NodeLayoutMeasurer;
use freya_node_state::{
    CursorSettings, CustomAttributeValues, DirectionMode, FontStyle, References, Scroll, Size,
    SizeMode, Style, Transform,
};
use skia_safe::textlayout::FontCollection;

const SCALE_FACTOR: f32 = 1.0;

fn default_node_state(node: &mut NodeMut<CustomAttributeValues>) {
    node.insert(CursorSettings::default());
    node.insert(FontStyle::default());
    node.insert(References::default());
    node.insert(Scroll::default());
    node.insert(Size::default());
    node.insert(Style::default());
    node.insert(Transform::default());
}

#[test]
fn percentage() {
    let mut dom = FreyaDOM::default();

    let node_id = {
        let mut node = dom.dom_mut().create_node(NodeType::Element(ElementNode {
            tag: "rect".to_owned(),
            namespace: Some("rect".to_owned()),
            attributes: HashMap::default(),
            listeners: HashSet::default(),
        }));

        default_node_state(&mut node);

        node.insert(Size {
            width: SizeMode::Percentage(50.0),
            height: SizeMode::Percentage(25.0),
            ..expanded_size()
        });

        node.id()
    };

    let node = dom.dom().get(node_id).unwrap();

    let mut remaining_area = Area {
        origin: (0.0, 0.0).into(),
        size: (200.0, 300.0).into(),
    };
    let mut layers = Layers::default();
    let mut fonts = FontCollection::new();
    let mut measurer = NodeLayoutMeasurer::new(
        node,
        &mut remaining_area,
        Area {
            origin: (0.0, 0.0).into(),
            size: (200.0, 300.0).into(),
        },
        &dom,
        &mut layers,
        0,
        &mut fonts,
    );
    let result = measurer.measure_area(true, SCALE_FACTOR);

    assert_eq!(result.height(), 75.0);
    assert_eq!(result.width(), 100.0);
}

#[test]
fn manual() {
    let mut dom = FreyaDOM::default();

    let node_id = {
        let mut node = dom.dom_mut().create_node(NodeType::Element(ElementNode {
            tag: "rect".to_owned(),
            namespace: Some("rect".to_owned()),
            attributes: HashMap::default(),
            listeners: HashSet::default(),
        }));

        default_node_state(&mut node);

        node.insert(Size {
            width: SizeMode::Manual(250.0),
            height: SizeMode::Manual(150.0),
            ..expanded_size()
        });

        node.id()
    };

    let node = dom.dom().get(node_id).unwrap();

    let mut remaining_area = Area {
        origin: (0.0, 0.0).into(),
        size: (200.0, 300.0).into(),
    };
    let mut layers = Layers::default();
    let mut fonts = FontCollection::new();
    let mut measurer = NodeLayoutMeasurer::new(
        node,
        &mut remaining_area,
        Area {
            origin: (0.0, 0.0).into(),
            size: (200.0, 300.0).into(),
        },
        &dom,
        &mut layers,
        0,
        &mut fonts,
    );
    let result = measurer.measure_area(true, SCALE_FACTOR);

    assert_eq!(result.height(), 150.0);
    assert_eq!(result.width(), 250.0);
}

#[test]
fn auto() {
    let mut dom = FreyaDOM::default();

    let child_node_id = {
        let mut child_node = dom.dom_mut().create_node(NodeType::Element(ElementNode {
            tag: "rect".to_owned(),
            namespace: Some("rect".to_owned()),
            attributes: HashMap::default(),
            listeners: HashSet::default(),
        }));

        default_node_state(&mut child_node);

        child_node.insert(Size {
            width: SizeMode::Manual(170.0),
            height: SizeMode::Manual(25.0),
            direction: DirectionMode::Both,
            ..expanded_size()
        });

        child_node.id()
    };

    let node_id = {
        let mut node = dom.dom_mut().create_node(NodeType::Element(ElementNode {
            tag: "rect".to_owned(),
            namespace: Some("rect".to_owned()),
            attributes: HashMap::default(),
            listeners: HashSet::default(),
        }));

        default_node_state(&mut node);

        node.insert(Size {
            width: SizeMode::Auto,
            height: SizeMode::Auto,
            direction: DirectionMode::Both,
            ..expanded_size()
        });

        node.add_child(child_node_id);

        node.id()
    };

    let node = dom.dom().get(node_id).unwrap();

    let mut remaining_area = Area {
        origin: (0.0, 0.0).into(),
        size: (200.0, 300.0).into(),
    };
    let mut layers = Layers::default();
    let mut fonts = FontCollection::new();
    let mut measurer = NodeLayoutMeasurer::new(
        node,
        &mut remaining_area,
        Area {
            origin: (0.0, 0.0).into(),
            size: (200.0, 300.0).into(),
        },
        &dom,
        &mut layers,
        0,
        &mut fonts,
    );
    let result = measurer.measure_area(true, SCALE_FACTOR);

    assert_eq!(result.height(), 25.0);
    assert_eq!(result.width(), 170.0);
}

#[test]
fn x_y() {
    let mut fdom = FreyaDOM::default();

    let node_id = {
        let dom = fdom.dom_mut();

        let child_node_id = {
            let mut child_node = dom.create_node(NodeType::Element(ElementNode {
                tag: "rect".to_owned(),
                namespace: Some("rect".to_owned()),
                attributes: HashMap::default(),
                listeners: HashSet::default(),
            }));

            default_node_state(&mut child_node);

            child_node.insert(Size {
                width: SizeMode::Manual(170.0),
                height: SizeMode::Manual(25.0),
                direction: DirectionMode::Both,
                ..expanded_size()
            });

            child_node.id()
        };

        let mut node = dom.create_node(NodeType::Element(ElementNode {
            tag: "rect".to_owned(),
            namespace: Some("rect".to_owned()),
            attributes: HashMap::default(),
            listeners: HashSet::default(),
        }));

        default_node_state(&mut node);

        node.insert(Size {
            width: SizeMode::Auto,
            height: SizeMode::Auto,
            direction: DirectionMode::Both,
            ..expanded_size()
        });

        node.add_child(child_node_id);

        node.id()
    };

    let node = fdom.dom().get(node_id).unwrap();

    let mut remaining_area = Area {
        origin: (15.0, 25.0).into(),
        size: (200.0, 300.0).into(),
    };

    let mut layers = Layers::default();
    let mut fonts = FontCollection::new();
    let mut measurer = NodeLayoutMeasurer::new(
        node,
        &mut remaining_area,
        Area {
            origin: (15.0, 25.0).into(),
            size: (200.0, 300.0).into(),
        },
        &fdom,
        &mut layers,
        0,
        &mut fonts,
    );

    let result = measurer.measure_area(true, SCALE_FACTOR);

    assert_eq!(result.min_x(), 15.0);
    assert_eq!(result.min_y(), 25.0);
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
