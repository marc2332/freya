use std::sync::{
    Arc,
    Mutex,
};

use freya_engine::prelude::BlendMode;
use freya_native_core::{
    exports::shipyard::Component,
    node_ref::NodeView,
    prelude::{
        AttributeMaskBuilder,
        AttributeName,
        Dependancy,
        NodeMaskBuilder,
        State,
    },
    NodeId,
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;

use crate::{
    custom_attributes::CustomAttributeValues,
    dom::CompositorDirtyNodes,
    parsing::{
        Parse,
        ParseAttribute,
        ParseError,
    },
};

#[derive(Default, Clone, Debug, Component, PartialEq)]
pub struct TransformState {
    pub node_id: NodeId,
    pub opacities: Vec<f32>,
    pub rotations: Vec<(NodeId, f32)>,
    pub scales: Vec<(NodeId, f32, f32)>,
    pub blend_mode: Option<BlendMode>,
    pub backdrop_blur: f32,
}

impl ParseAttribute for TransformState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), ParseError> {
        match attr.attribute {
            AttributeName::Rotate => {
                let value = attr.value.as_text().ok_or(ParseError)?;
                if value.ends_with("deg") {
                    let rotation = value
                        .trim_end_matches("deg")
                        .parse::<f32>()
                        .map_err(|_| ParseError)?;
                    self.rotations.push((self.node_id, rotation));
                }
            }
            AttributeName::Opacity => {
                let value = attr.value.as_text().ok_or(ParseError)?;
                let opacity = value.parse::<f32>().map_err(|_| ParseError)?;
                self.opacities.push(opacity);
            }
            AttributeName::Scale => {
                let value = attr.value.as_text().ok_or(ParseError)?;
                let (scale_x, scale_y) = if !value.trim().contains(' ') {
                    let scale = value.parse::<f32>().map_err(|_| ParseError)?;
                    (scale, scale)
                } else {
                    let (x, y) = value.split_once(' ').ok_or(ParseError)?;
                    let scale_x = x.parse::<f32>().map_err(|_| ParseError)?;
                    let scale_y = y.parse::<f32>().map_err(|_| ParseError)?;
                    (scale_x, scale_y)
                };
                self.scales
                    .push((self.node_id, scale_x.max(0.0), scale_y.max(0.0)));
            }

            AttributeName::BlendMode => {
                self.blend_mode = Some(BlendMode::parse(attr.value.as_text().ok_or(ParseError)?)?);
            }
            AttributeName::BackdropBlur => {
                self.backdrop_blur = attr
                    .value
                    .as_text()
                    .ok_or(ParseError)?
                    .parse::<f32>()
                    .map_err(|_| ParseError)?;
            }
            _ => {}
        }

        Ok(())
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for TransformState {
    type ParentDependencies = (Self,);

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::Rotate,
            AttributeName::Opacity,
            AttributeName::Scale,
            AttributeName::AspectRatio,
            AttributeName::ImageCover,
            AttributeName::BlendMode,
            AttributeName::BackdropBlur,
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let root_id = context.get::<NodeId>().unwrap();
        let compositor_dirty_nodes = context.get::<Arc<Mutex<CompositorDirtyNodes>>>().unwrap();
        let inherited_transform = parent.map(|(p,)| p.clone()).unwrap_or_default();

        let mut transform_state = TransformState {
            node_id: node_view.node_id(),
            blend_mode: None,
            backdrop_blur: 0.,
            ..inherited_transform
        };

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                transform_state.parse_safe(attr);
            }
        }

        let changed = transform_state != *self;

        let is_orphan = node_view.height() == 0 && node_view.node_id() != *root_id;

        if changed && !is_orphan {
            compositor_dirty_nodes
                .lock()
                .unwrap()
                .invalidate(node_view.node_id());
        }

        *self = transform_state;
        changed
    }
}
