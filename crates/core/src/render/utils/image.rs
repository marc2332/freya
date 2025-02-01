use freya_common::ImagesCache;
use freya_engine::prelude::{
    Data,
    Image,
};
use freya_native_core::prelude::NodeImmutable;
use freya_node_state::{
    AspectRatio,
    ReferencesState,
    StyleState,
    TransformState,
};
use torin::prelude::Size2D;

use crate::dom::DioxusNode;

pub struct ImageData {
    pub image: Image,
    pub size: Size2D,
}

pub fn get_or_create_image(
    node_ref: &DioxusNode,
    area_size: &Size2D,
    images_cache: &mut ImagesCache,
) -> Option<ImageData> {
    let node_style = node_ref.get::<StyleState>().unwrap();
    let node_references = node_ref.get::<ReferencesState>().unwrap();

    let mut get_or_create_image = |bytes: &[u8]| -> Option<Image> {
        if let Some(image_cache_key) = &node_style.image_cache_key {
            images_cache.get(image_cache_key).cloned().or_else(|| {
                Image::from_encoded(unsafe { Data::new_bytes(bytes) }).inspect(|image| {
                    images_cache.insert(image_cache_key.clone(), image.clone());
                })
            })
        } else {
            Image::from_encoded(unsafe { Data::new_bytes(bytes) })
        }
    };

    let image = if let Some(image_ref) = &node_references.image_ref {
        let image_data = image_ref.0.lock().unwrap();
        if let Some(bytes) = image_data.as_ref() {
            get_or_create_image(bytes)
        } else {
            None
        }
    } else if let Some(image_data) = &node_style.image_data {
        get_or_create_image(image_data.as_slice())
    } else {
        None
    }?;

    let node_transform = node_ref.get::<TransformState>().unwrap();

    let image_width = image.width() as f32;
    let image_height = image.height() as f32;

    let width_ratio = area_size.width / image.width() as f32;
    let height_ratio = area_size.height / image.height() as f32;

    let size = match node_transform.aspect_ratio {
        AspectRatio::Max => {
            let ratio = width_ratio.max(height_ratio);

            Size2D::new(image_width * ratio, image_height * ratio)
        }
        AspectRatio::Min => {
            let ratio = width_ratio.min(height_ratio);

            Size2D::new(image_width * ratio, image_height * ratio)
        }
        AspectRatio::Auto => Size2D::new(image_width, image_height),
        AspectRatio::None => *area_size,
    };

    Some(ImageData { image, size })
}
