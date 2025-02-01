use freya_common::ImagesCache;
use freya_engine::prelude::*;
use freya_native_core::real_dom::NodeImmutable;
use freya_node_state::{
    ImageCover,
    TransformState,
};

use super::utils::ElementUtils;
use crate::{
    dom::DioxusNode,
    render::{
        get_or_create_image,
        ImageData,
    },
};

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

        let Some(ImageData { image, size }) =
            get_or_create_image(node_ref, &area.size, images_cache)
        else {
            return;
        };

        let node_transform = node_ref.get::<TransformState>().unwrap();

        let mut rect = Rect::new(
            area.min_x(),
            area.min_y(),
            area.min_x() + size.width,
            area.min_y() + size.height,
        );

        let clip_rect = Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y());

        if node_transform.image_cover == ImageCover::Center {
            let width_offset = (size.width - area.width()) / 2.;
            let height_offset = (size.width - area.height()) / 2.;

            rect.left -= width_offset;
            rect.right -= width_offset;
            rect.top -= height_offset;
            rect.bottom -= height_offset;
        }

        canvas.save();
        canvas.clip_rect(clip_rect, ClipOp::Intersect, true);

        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        canvas.draw_image_rect(image, None, rect, &paint);

        canvas.restore();
    }
}
