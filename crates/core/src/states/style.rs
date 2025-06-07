use std::sync::{
    Arc,
    Mutex,
};

use freya_engine::prelude::BlendMode;
use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node_ref::NodeView,
    prelude::{
        AttributeMaskBuilder,
        Dependancy,
        NodeMaskBuilder,
        State,
    },
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;

use crate::{
    custom_attributes::CustomAttributeValues,
    dom::CompositorDirtyNodes,
    parsing::{
        ExtSplit,
        Parse,
        ParseAttribute,
        ParseError,
    },
    values::{
        parse_alpha,
        Border,
        CornerRadius,
        Fill,
        OverflowMode,
        Shadow,
    },
};

#[derive(Default, Debug, Clone, PartialEq, Component)]
pub struct StyleState {
    pub background: Fill,
    pub background_opacity: Option<u8>,
    pub borders: Arc<[Border]>,
    pub shadows: Arc<[Shadow]>,
    pub corner_radius: CornerRadius,
    pub overflow: OverflowMode,
    pub blend_mode: Option<BlendMode>,
    pub backdrop_blur: f32,
}

impl ParseAttribute for StyleState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), ParseError> {
        match attr.attribute {
            AttributeName::Background => {
                let value = attr.value.as_text().ok_or(ParseError)?;
                if value == "none" {
                    return Ok(());
                }
                self.background = Fill::parse(value)?;
            }
            AttributeName::BackgroundOpacity => {
                let value = attr.value.as_text().ok_or(ParseError)?;
                if value == "none" {
                    return Ok(());
                }
                self.background_opacity = Some(parse_alpha(value)?);
            }
            AttributeName::Border => {
                self.borders = attr
                    .value
                    .as_text()
                    .ok_or(ParseError)?
                    .split_excluding_group(',', '(', ')')
                    .map(|chunk| Border::parse(chunk).unwrap_or_default())
                    .collect();
            }
            AttributeName::Shadow => {
                self.shadows = attr
                    .value
                    .as_text()
                    .ok_or(ParseError)?
                    .split_excluding_group(',', '(', ')')
                    .map(|chunk| Shadow::parse(chunk).unwrap_or_default())
                    .collect();
            }
            AttributeName::CornerRadius => {
                let mut radius = CornerRadius::parse(attr.value.as_text().ok_or(ParseError)?)?;
                radius.smoothing = self.corner_radius.smoothing;
                self.corner_radius = radius;
            }
            AttributeName::CornerSmoothing => {
                let value = attr.value.as_text().ok_or(ParseError)?;
                if value.ends_with('%') {
                    let smoothing = value
                        .trim_end_matches('%')
                        .parse::<f32>()
                        .map_err(|_| ParseError)?;
                    self.corner_radius.smoothing = (smoothing / 100.0).clamp(0.0, 1.0);
                }
            }
            AttributeName::Overflow => {
                self.overflow = OverflowMode::parse(attr.value.as_text().ok_or(ParseError)?)?;
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
impl State<CustomAttributeValues> for StyleState {
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::Background,
            AttributeName::BackgroundOpacity,
            AttributeName::Layer,
            AttributeName::Border,
            AttributeName::Shadow,
            AttributeName::CornerRadius,
            AttributeName::CornerSmoothing,
            AttributeName::Sampling,
            AttributeName::ImageData,
            AttributeName::Overflow,
            AttributeName::ImageCacheKey,
            AttributeName::BlendMode,
            AttributeName::BackdropBlur,
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

        if let Some(background_opacity) = style.background_opacity {
            style.background.set_a(background_opacity);
        }

        let changed = &style != self;

        if changed {
            let compositor_dirty_nodes = context.get::<Arc<Mutex<CompositorDirtyNodes>>>().unwrap();
            compositor_dirty_nodes
                .lock()
                .unwrap()
                .invalidate(node_view.node_id());
        }

        *self = style;
        changed
    }
}
