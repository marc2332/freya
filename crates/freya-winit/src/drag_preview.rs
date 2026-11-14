use std::borrow::Cow;

use freya_core::{
    integration::*,
    prelude::Color,
};
use freya_engine::prelude::{
    AlphaType,
    ColorType,
    FontCollection,
    FontMgr,
    ImageInfo,
    raster_n32_premul,
};
use torin::prelude::{
    Size2DFixed,
    Vector2D,
};
use winit::{
    event_loop::DragIcon,
    icon::RgbaIcon,
};

use crate::window::AppWindow;

impl AppWindow {
    pub(crate) fn render_drag_preview(
        &self,
        preview: NativeDragPreview,
        scope_id: ScopeId,
        font_collection: &mut FontCollection,
        font_manager: &FontMgr,
        fallback_fonts: &[Cow<'static, str>],
    ) -> Result<DragIcon, DragError> {
        let scale_factor = self.effective_scale_factor();
        let (size, offset) = match &preview {
            NativeDragPreview::Snapshot { area, offset } => (area.size, *offset),
            NativeDragPreview::Element { size, offset, .. } => (*size, *offset),
        };
        if !size.is_finite() || !size.is_positive() || !offset.is_finite() {
            return Err(DragError::InvalidData(
                "Drag preview needs a finite offset and a finite, positive size".into(),
            ));
        }

        let physical_size = size.to_f64() * scale_factor;
        let thumbnail_scale = (512. / physical_size.width.max(physical_size.height)).min(1.);
        let render_scale = scale_factor * thumbnail_scale;
        let bitmap_size = (physical_size * thumbnail_scale).ceil().to_i32();

        let mut surface = raster_n32_premul(bitmap_size.to_tuple())
            .ok_or_else(|| DragError::Platform("Could not allocate drag preview".into()))?;

        match preview {
            NativeDragPreview::Snapshot { area, .. } => {
                surface
                    .canvas()
                    .scale((thumbnail_scale as f32, thumbnail_scale as f32));
                surface
                    .canvas()
                    .translate((-area.origin.to_vector() * scale_factor as f32).to_tuple());
                RenderPipeline {
                    font_collection,
                    font_manager,
                    tree: &self.tree,
                    canvas: surface.canvas(),
                    scale_factor,
                    background: Color::TRANSPARENT,
                }
                .render();
            }
            NativeDragPreview::Element { element, .. } => {
                let mut runner = Runner::new_from(&self.runner, scope_id, move || element.clone());

                let mut tree = Tree::default();
                runner.provide_root_context(|| tree.accessibility_generator.clone());
                runner.provide_root_context(|| font_collection.clone());
                let mutations = runner.sync_and_update();
                runner.run_in(|| tree.apply_mutations(mutations, render_scale as f32));
                let (sender, _receiver) = futures_channel::mpsc::unbounded();
                tree.measure_layout(
                    bitmap_size.to_f32(),
                    font_collection,
                    font_manager,
                    &sender,
                    &mut NodesState::default(),
                    render_scale,
                    fallback_fonts,
                );
                RenderPipeline {
                    font_collection,
                    font_manager,
                    tree: &tree,
                    canvas: surface.canvas(),
                    scale_factor: render_scale,
                    background: Color::TRANSPARENT,
                }
                .render();
            }
        }

        let image_info = ImageInfo::new(
            bitmap_size.to_tuple(),
            ColorType::RGBA8888,
            AlphaType::Unpremul,
            None,
        );
        let pixel_size = bitmap_size.cast::<usize>();
        let mut pixels = vec![0; pixel_size.area() * 4];
        if !surface.read_pixels(&image_info, &mut pixels, pixel_size.width * 4, (0, 0)) {
            return Err(DragError::Platform(
                "Could not read drag preview pixels".into(),
            ));
        }

        let icon_size = bitmap_size.to_u32();
        let icon = RgbaIcon::new(pixels, icon_size.width, icon_size.height)
            .map_err(|error| DragError::InvalidData(error.to_string()))?;
        let max_offset = (bitmap_size - Size2DFixed::new(1, 1)).to_f64().to_vector();
        let offset = -(offset.to_f64() * render_scale)
            .clamp(Vector2D::zero().to_f64(), max_offset)
            .to_i32();

        Ok(DragIcon {
            icon: icon.into(),
            offset_x: offset.x,
            offset_y: offset.y,
        })
    }
}
