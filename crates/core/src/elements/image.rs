use freya_engine::prelude::*;
use freya_native_core::real_dom::NodeImmutable;

use super::utils::ElementUtils;
use crate::{
    dom::{DioxusNode, ImagesCache},
    render::{get_or_create_image, ImageData},
    states::ImageState,
    values::{ImageCover, SamplingMode},
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
        _fallback_fonts: &[String],
        images_cache: &mut ImagesCache,
        _scale_factor: f32,
    ) {
        let area = layout_node.visible_area();

        let Some(ImageData { image, .. }) = get_or_create_image(node_ref, &area.size, images_cache)
        else {
            return;
        };

        let image_state = node_ref.get::<ImageState>().unwrap();

        let mut rect = Rect::new(
            area.min_x(),
            area.min_y(),
            area.min_x() + area.width(),
            area.min_y() + area.height(),
        );
        let clip_rect = Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y());

        // if image_state.image_cover == ImageCover::Center {
        //     let width_offset = (size.width - area.width()) / 2.;
        //     let height_offset = (size.height - area.height()) / 2.;

        //     rect.left -= width_offset;
        //     rect.right -= width_offset;
        //     rect.top -= height_offset;
        //     rect.bottom -= height_offset;
        // }

        canvas.save();
        canvas.clip_rect(clip_rect, ClipOp::Intersect, true);

        let mut paint = Paint::default();
        paint.set_anti_alias(true);

        let sampling = match image_state.image_sampling {
            SamplingMode::Nearest => SamplingOptions::new(FilterMode::Nearest, MipmapMode::None),
            SamplingMode::Bilinear => SamplingOptions::new(FilterMode::Linear, MipmapMode::None),
            SamplingMode::Trilinear => SamplingOptions::new(FilterMode::Linear, MipmapMode::Linear),
            SamplingMode::Mitchell => SamplingOptions::from(CubicResampler::mitchell()),
            SamplingMode::CatmullRom => SamplingOptions::from(CubicResampler::catmull_rom()),
        };

        canvas.draw_image_rect_with_sampling_options(image, None, rect, sampling, &paint);

        canvas.restore();
    }
    fn clip(
        &self,
        layout_node: &torin::prelude::LayoutNode,
        _node_ref: &DioxusNode,
        canvas: &Canvas,
        _scale_factor: f32,
    ) {
        canvas.clip_rect(
            Rect::new(
                layout_node.area.min_x(),
                layout_node.area.min_y(),
                layout_node.area.max_x(),
                layout_node.area.max_y(),
            ),
            ClipOp::Intersect,
            true,
        );
    }
}
