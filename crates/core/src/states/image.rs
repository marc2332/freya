use std::sync::{
    Arc,
    Mutex,
};

use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node::{
        NodeType,
        OwnedAttributeValue,
    },
    node_ref::NodeView,
    prelude::{
        AttributeMaskBuilder,
        Dependancy,
        NodeMaskBuilder,
        State,
    },
    tags::TagName,
    NodeId,
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;
use torin::torin::Torin;

use crate::{
    custom_attributes::{
        AttributesBytes,
        CustomAttributeValues,
        ImageReference,
    },
    dom::{
        CompositorDirtyNodes,
        ImageCacheKey,
        ImagesCache,
    },
    parsing::{
        Parse,
        ParseAttribute,
        ParseError,
    },
    values::{
        ImageCover,
        SamplingMode,
    },
};

#[derive(Default, Debug, Clone, PartialEq, Component)]
pub struct ImageState {
    pub image_sampling: SamplingMode,
    pub image_data: Option<AttributesBytes>,
    pub image_cache_key: Option<ImageCacheKey>,
    pub image_cover: ImageCover,
    pub image_ref: Option<ImageReference>,
}

impl ParseAttribute for ImageState {
    fn parse_attribute(
        &mut self,
        attr: freya_native_core::prelude::OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), ParseError> {
        match attr.attribute {
            AttributeName::Sampling => {
                self.image_sampling = SamplingMode::parse(attr.value.as_text().ok_or(ParseError)?)?;
            }
            AttributeName::ImageData => {
                if let OwnedAttributeValue::Custom(CustomAttributeValues::Bytes(bytes)) = attr.value
                {
                    self.image_data = Some(bytes.clone());
                }
            }
            AttributeName::ImageCacheKey => {
                if let OwnedAttributeValue::Text(key) = attr.value {
                    self.image_cache_key = Some(ImageCacheKey(key.clone()));
                }
            }
            AttributeName::ImageCover => {
                self.image_cover = ImageCover::parse(attr.value.as_text().ok_or(ParseError)?)
                    .map_err(|_| ParseError)?;
            }
            AttributeName::ImageReference => {
                if let OwnedAttributeValue::Custom(CustomAttributeValues::ImageReference(
                    reference,
                )) = attr.value
                {
                    self.image_ref = Some(reference.clone());
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for ImageState {
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::Sampling,
            AttributeName::ImageData,
            AttributeName::ImageCacheKey,
            AttributeName::ImageCover,
            AttributeName::ImageReference,
        ]));

    fn allow_node(node_type: &NodeType<CustomAttributeValues>) -> bool {
        node_type.tag() == Some(&TagName::Image)
    }

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let mut image = ImageState::default();

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                image.parse_safe(attr)
            }
        }

        let changed = &image != self;

        if changed {
            let compositor_dirty_nodes = context.get::<Arc<Mutex<CompositorDirtyNodes>>>().unwrap();
            compositor_dirty_nodes
                .lock()
                .unwrap()
                .invalidate(node_view.node_id());

            if let Some(image_cache_key) = &self.image_cache_key {
                let images_cache = context.get::<Arc<Mutex<ImagesCache>>>().unwrap();
                images_cache.lock().unwrap().remove(image_cache_key);
            }

            let torin_layout = context.get::<Arc<Mutex<Torin<NodeId>>>>().unwrap();
            torin_layout.lock().unwrap().invalidate(node_view.node_id());
        }

        *self = image;
        changed
    }
}
