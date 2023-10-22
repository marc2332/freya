use std::sync::{Arc, Mutex};

use dioxus_native_core::{
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    NodeId, SendAnyMap,
};
use dioxus_native_core_macro::partial_derive_state;
use freya_common::NodeReferenceLayout;
use tokio::sync::mpsc::UnboundedSender;
use torin::prelude::*;

use crate::{CustomAttributeValues, Parse};

#[derive(Default, Clone, Debug, Component)]
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
    pub offset_y: f32,
    pub offset_x: f32,
    pub main_alignment: Alignment,
    pub cross_alignment: Alignment,
    pub node_ref: Option<UnboundedSender<NodeReferenceLayout>>,
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
            "main_alignment",
            "cross_alignment",
            "reference",
            "margin",
        ]))
        .with_tag()
        .with_text();

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
            } else if node_view.text().is_some() {
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
                                layout.offset_y = scroll * scale_factor;
                            }
                        }
                    }
                    "offset_x" => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(scroll) = value.parse::<f32>() {
                                layout.offset_x = scroll * scale_factor;
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
                    "reference" => {
                        if let OwnedAttributeValue::Custom(CustomAttributeValues::Reference(
                            reference,
                        )) = attr.value
                        {
                            layout.node_ref = Some(reference.0.clone());
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
            || (layout.cross_alignment != self.cross_alignment);

        if changed {
            torin_layout.lock().unwrap().invalidate(node_view.node_id());
        }

        *self = layout;
        changed
    }
}
