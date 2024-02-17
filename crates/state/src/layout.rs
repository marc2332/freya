use std::sync::{Arc, Mutex};

use dioxus_native_core::{
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    NodeId, SendAnyMap,
};
use dioxus_native_core_macro::partial_derive_state;
use torin::prelude::*;

use crate::{CustomAttributeValues, NodeReference, Parse};

#[derive(Default, Clone, Debug, Component, PartialEq)]
pub struct LayoutState {
    pub width: Size,
    pub height: Size,
    pub minimum_width: Size,
    pub minimum_height: Size,
    pub maximum_height: Size,
    pub maximum_width: Size,
    pub padding: Gaps,
    pub margin: Gaps,
    pub direction: DirectionMode,
    pub node_id: NodeId,
    pub offset_y: Length,
    pub offset_x: Length,
    pub main_alignment: Alignment,
    pub cross_alignment: Alignment,
    pub position: Position,
    pub node_ref: Option<NodeReference>,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for LayoutState {
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> = NodeMaskBuilder::new()
        .with_attrs(AttributeMaskBuilder::Some(&[
            "width",
            "height",
            "min_height",
            "min_width",
            "max_height",
            "max_width",
            "padding",
            "direction",
            "offset_y",
            "offset_x",
            "main_align",
            "cross_align",
            "reference",
            "margin",
            "position",
            "position_top",
            "position_right",
            "position_bottom",
            "position_left",
        ]))
        .with_tag();

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let torin_layout = context.get::<Arc<Mutex<Torin<NodeId>>>>().unwrap();
        let scale_factor = context.get::<f32>().unwrap();

        let mut layout = LayoutState {
            direction: if let Some("label") = node_view.tag() {
                DirectionMode::Horizontal
            } else if let Some("paragraph") = node_view.tag() {
                DirectionMode::Horizontal
            } else if let Some("text") = node_view.tag() {
                DirectionMode::Horizontal
            } else {
                DirectionMode::Vertical
            },
            ..Default::default()
        };

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "width" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(mut width) = Size::parse(value) {
                                width.scale(*scale_factor);
                                layout.width = width;
                            }
                        }
                    }
                    "height" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(mut height) = Size::parse(value) {
                                height.scale(*scale_factor);
                                layout.height = height;
                            }
                        }
                    }
                    "min_height" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(mut min_height) = Size::parse(value) {
                                min_height.scale(*scale_factor);
                                layout.minimum_height = min_height;
                            }
                        }
                    }
                    "min_width" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(mut min_width) = Size::parse(value) {
                                min_width.scale(*scale_factor);
                                layout.minimum_width = min_width;
                            }
                        }
                    }
                    "max_height" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(mut max_height) = Size::parse(value) {
                                max_height.scale(*scale_factor);
                                layout.maximum_height = max_height;
                            }
                        }
                    }
                    "max_width" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(mut max_width) = Size::parse(value) {
                                max_width.scale(*scale_factor);
                                layout.maximum_width = max_width;
                            }
                        }
                    }
                    "padding" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(mut padding) = Gaps::parse(value) {
                                padding.scale(*scale_factor);
                                layout.padding = padding;
                            }
                        }
                    }
                    "margin" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(mut margin) = Gaps::parse(value) {
                                margin.scale(*scale_factor);
                                layout.margin = margin;
                            }
                        }
                    }
                    "direction" => {
                        if let Some(value) = attr.value.as_text() {
                            layout.direction = match value {
                                "horizontal" => DirectionMode::Horizontal,
                                _ => DirectionMode::Vertical,
                            }
                        }
                    }
                    "offset_y" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(scroll) = value.parse::<f32>() {
                                layout.offset_y = Length::new(scroll * scale_factor);
                            }
                        }
                    }
                    "offset_x" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(scroll) = value.parse::<f32>() {
                                layout.offset_x = Length::new(scroll * scale_factor);
                            }
                        }
                    }
                    "main_align" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(alignment) = Alignment::parse(value) {
                                layout.main_alignment = alignment;
                            }
                        }
                    }
                    "cross_align" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(alignment) = Alignment::parse(value) {
                                layout.cross_alignment = alignment;
                            }
                        }
                    }
                    "position" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(position) = Position::parse(value) {
                                if layout.position.is_empty() {
                                    layout.position = position;
                                }
                            }
                        }
                    }
                    "position_top" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(top) = value.parse::<f32>() {
                                layout.position.set_top(top * scale_factor);
                            }
                        }
                    }
                    "position_right" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(right) = value.parse::<f32>() {
                                layout.position.set_right(right * scale_factor);
                            }
                        }
                    }
                    "position_bottom" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(bottom) = value.parse::<f32>() {
                                layout.position.set_bottom(bottom * scale_factor);
                            }
                        }
                    }
                    "position_left" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(left) = value.parse::<f32>() {
                                layout.position.set_left(left * scale_factor);
                            }
                        }
                    }
                    "reference" => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Reference(
                            reference,
                        )) = attr.value
                        {
                            layout.node_ref = Some(reference.clone());
                        }
                    }
                    _ => {
                        panic!("Unsupported attribute <{}>, this should not be happening, please report it.", attr.attribute.name);
                    }
                }
            }
        }

        let changed = (layout.width != self.width)
            || (layout.height != self.height)
            || (layout.minimum_width != self.minimum_width)
            || (layout.minimum_height != self.minimum_height)
            || (layout.maximum_width != self.maximum_width)
            || (layout.maximum_height != self.maximum_height)
            || (layout.padding != self.padding)
            || (node_view.node_id() != self.node_id)
            || (layout.direction != self.direction)
            || (layout.offset_x != self.offset_x)
            || (layout.offset_y != self.offset_y)
            || (layout.main_alignment != self.main_alignment)
            || (layout.cross_alignment != self.cross_alignment)
            || (layout.position != self.position);

        if changed {
            torin_layout.lock().unwrap().invalidate(node_view.node_id());
        }

        *self = layout;
        changed
    }
}
