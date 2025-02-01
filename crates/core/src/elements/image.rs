use freya_common::ImagesCache;
use freya_engine::prelude::*;
use freya_native_core::real_dom::NodeImmutable;
use freya_node_state::{
    AspectRatio,
    ImageCover,
    ReferencesState,
    StyleState,
    TransformState,
};

use super::utils::ElementUtils;
use crate::dom::DioxusNode;

pub struct ImageElement;

impl ElementUtils for ImageElement {
    fn render(
        self,
        layout_node: &torin::prelude::LayoutNode,
        node_ref: &DioxusNode,
        canvas: &Canvas,
        _font_collection: &mut FontCollection,
        _font_manager: &FontMgr,
        _default_fonts: &[String],
        images_cache: &mut ImagesCache,
        _scale_factor: f32,
    ) {
        let area = layout_node.visible_area();
        let node_style = node_ref.get::<StyleState>().unwrap();
        let node_references = node_ref.get::<ReferencesState>().unwrap();
        let node_transform = node_ref.get::<TransformState>().unwrap();

        let mut draw_img = |bytes: &[u8]| {
            let image = if let Some(image_cache_key) = &node_style.image_cache_key {
                images_cache.get(image_cache_key).cloned().or_else(|| {
                    Image::from_encoded(unsafe { Data::new_bytes(bytes) }).inspect(|image| {
                        images_cache.insert(image_cache_key.clone(), image.clone());
                    })
                })
            } else {
                Image::from_encoded(unsafe { Data::new_bytes(bytes) })
            };

            let Some(image) = image else {
                return;
            };

            let mut paint = Paint::default();
            paint.set_anti_alias(true);

            let width_ratio = area.width() / image.width() as f32;
            let height_ratio = area.height() / image.height() as f32;

            let (width, height) = match node_transform.aspect_ratio {
                AspectRatio::Max => {
                    let ratio = width_ratio.max(height_ratio);

                    (image.width() as f32 * ratio, image.height() as f32 * ratio)
                }
                AspectRatio::Min => {
                    let ratio = width_ratio.min(height_ratio);

                    (image.width() as f32 * ratio, image.height() as f32 * ratio)
                }
                AspectRatio::None => (area.width(), area.height()),
            };

            let mut rect = Rect::new(
                area.min_x(),
                area.min_y(),
                area.min_x() + width,
                area.min_y() + height,
            );

            if node_transform.image_cover == ImageCover::Center {
                let width_offset = (width - area.width()) / 2.;
                let height_offset = (height - area.height()) / 2.;

                let clip_rect = Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y());

                rect.left -= width_offset;
                rect.right -= width_offset;
                rect.top -= height_offset;
                rect.bottom -= height_offset;

                canvas.save();
                canvas.clip_rect(clip_rect, ClipOp::Intersect, true);
            }

            canvas.draw_image_rect(image, None, rect, &paint);

            if node_transform.image_cover == ImageCover::Center {
                canvas.restore();
            }
        };

        if let Some(image_ref) = &node_references.image_ref {
            let image_data = image_ref.0.lock().unwrap();
            if let Some(image_data) = image_data.as_ref() {
                draw_img(image_data)
            }
        } else if let Some(image_data) = &node_style.image_data {
            draw_img(image_data.as_slice())
        }
    }
}
