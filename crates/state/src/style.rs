use std::sync::{
    Arc,
    Mutex,
};

use freya_common::{
    CompositorDirtyNodes,
    ImageCacheKey,
    ImagesCache,
};
use freya_engine::prelude::Color;
use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{
        AttributeMaskBuilder,
        Dependancy,
        NodeMaskBuilder,
        State,
    },
    NodeId,
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;
use torin::torin::Torin;

use crate::{
    parsing::ExtSplit,
    AttributesBytes,
    Border,
    CornerRadius,
    CustomAttributeValues,
    Fill,
    OverflowMode,
    Parse,
    ParseAttribute,
    ParseError,
    Shadow,
};

#[derive(Default, Debug, Clone, PartialEq, Component)]
pub struct StyleState {
    pub background: Fill,
    pub svg_fill: Option<Color>,
    pub svg_stroke: Option<Color>,
    pub borders: Vec<Border>,
    pub shadows: Vec<Shadow>,
    pub corner_radius: CornerRadius,
    pub image_data: Option<AttributesBytes>,
    pub svg_data: Option<AttributesBytes>,
    pub overflow: OverflowMode,
    pub image_cache_key: Option<ImageCacheKey>,
}

impl ParseAttribute for StyleState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), crate::ParseError> {
        match attr.attribute {
            AttributeName::Background => {
                if let Some(value) = attr.value.as_text() {
                    if value == "none" {
                        return Ok(());
                    }
                    self.background = Fill::parse(value)?;
                }
            }
            AttributeName::Fill => {
                if let Some(value) = attr.value.as_text() {
                    if value == "none" {
                        return Ok(());
                    }
                    self.svg_stroke = Some(Color::parse(value)?);
                }
            }
            AttributeName::Stroke => {
                if let Some(value) = attr.value.as_text() {
                    if value == "none" {
                        return Ok(());
                    }
                    self.svg_fill = Some(Color::parse(value)?);
                }
            }
            AttributeName::Border => {
                if let Some(value) = attr.value.as_text() {
                    self.borders = value
                        .split_excluding_group(',', '(', ')')
                        .map(|chunk| Border::parse(chunk).unwrap_or_default())
                        .collect();
                }
            }
            AttributeName::Shadow => {
                if let Some(value) = attr.value.as_text() {
                    self.shadows = value
                        .split_excluding_group(',', '(', ')')
                        .map(|chunk| Shadow::parse(chunk).unwrap_or_default())
                        .collect();
                }
            }
            AttributeName::CornerRadius => {
                if let Some(value) = attr.value.as_text() {
                    let mut radius = CornerRadius::parse(value)?;
                    radius.smoothing = self.corner_radius.smoothing;
                    self.corner_radius = radius;
                }
            }
            AttributeName::CornerSmoothing => {
                if let Some(value) = attr.value.as_text() {
                    if value.ends_with('%') {
                        let smoothing = value
                            .replacen('%', "", 1)
                            .parse::<f32>()
                            .map_err(|_| ParseError)?;
                        self.corner_radius.smoothing = (smoothing / 100.0).clamp(0.0, 1.0);
                    }
                }
            }
            AttributeName::ImageData => {
                if let OwnedAttributeValue::Custom(CustomAttributeValues::Bytes(bytes)) = attr.value
                {
                    self.image_data = Some(bytes.clone());
                }
            }
            AttributeName::SvgData => {
                if let OwnedAttributeValue::Custom(CustomAttributeValues::Bytes(bytes)) = attr.value
                {
                    self.svg_data = Some(bytes.clone());
                }
            }
            AttributeName::SvgContent => {
                let text = attr.value.as_text();
                self.svg_data =
                    text.map(|v| AttributesBytes::Dynamic(v.as_bytes().to_vec().into()));
            }
            AttributeName::Overflow => {
                if let Some(value) = attr.value.as_text() {
                    self.overflow = OverflowMode::parse(value)?;
                }
            }
            AttributeName::ImageCacheKey => {
                if let OwnedAttributeValue::Text(key) = attr.value {
                    self.image_cache_key = Some(ImageCacheKey(key.clone()));
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for StyleState {
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::Background,
            AttributeName::Fill,
            AttributeName::Stroke,
            AttributeName::Layer,
            AttributeName::Border,
            AttributeName::Shadow,
            AttributeName::CornerRadius,
            AttributeName::CornerSmoothing,
            AttributeName::ImageData,
            AttributeName::SvgData,
            AttributeName::SvgContent,
            AttributeName::Overflow,
            AttributeName::ImageCacheKey,
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let mut style = StyleState::default();

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                style.parse_safe(attr)
            }
        }

        let changed = &style != self;
        let changed_image_cache_key = style.image_cache_key != self.image_cache_key;

        if changed {
            let compositor_dirty_nodes = context.get::<Arc<Mutex<CompositorDirtyNodes>>>().unwrap();
            compositor_dirty_nodes
                .lock()
                .unwrap()
                .invalidate(node_view.node_id());
        }

        if changed_image_cache_key {
            if let Some(image_cache_key) = &self.image_cache_key {
                let images_cache = context.get::<Arc<Mutex<ImagesCache>>>().unwrap();
                images_cache.lock().unwrap().remove(image_cache_key);
            }

            let torin_layout = context.get::<Arc<Mutex<Torin<NodeId>>>>().unwrap();
            torin_layout.lock().unwrap().invalidate(node_view.node_id());
        }

        *self = style;
        changed
    }
}
