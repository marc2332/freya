use freya_engine::prelude::*;
use freya_native_core::real_dom::NodeImmutable;
use freya_node_state::{
    ReferencesState,
    StyleState,
};

use super::utils::ElementUtils;
use crate::{
    dom::DioxusNode,
    render::ParagraphCache,
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
        _scale_factor: f32,
        _paragraph_cache: &mut ParagraphCache,
    ) {
        let area = layout_node.visible_area();
        let node_style = node_ref.get::<StyleState>().unwrap();
        let node_references = node_ref.get::<ReferencesState>().unwrap();

        let draw_img = |bytes: &[u8]| {
            let pic = Image::from_encoded(unsafe { Data::new_bytes(bytes) });
            if let Some(pic) = pic {
                let mut paint = Paint::default();
                paint.set_anti_alias(true);
                canvas.draw_image_nine(
                    pic,
                    IRect::new(0, 0, 0, 0),
                    Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
                    FilterMode::Last,
                    Some(&paint),
                );
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
