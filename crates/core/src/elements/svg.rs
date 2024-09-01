use freya_engine::prelude::*;
use freya_native_core::real_dom::NodeImmutable;
use freya_node_state::StyleState;
use torin::prelude::LayoutNode;

use super::utils::ElementUtils;
use crate::dom::DioxusNode;

pub struct SvgElement;

impl ElementUtils for SvgElement {
    fn render(
        self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        canvas: &Canvas,
        _font_collection: &mut FontCollection,
        font_manager: &FontMgr,
        _default_fonts: &[String],
        _scale_factor: f32,
    ) {
        let node_style = &*node_ref.get::<StyleState>().unwrap();

        let mut area = layout_node.visible_area().round();

        if node_style.subpixel_rounding {
            area = area.round();
        }

        let x = area.min_x();
        let y = area.min_y();
        if let Some(svg_data) = &node_style.svg_data {
            let svg_dom = svg::Dom::from_bytes(svg_data.as_slice(), font_manager);
            if let Ok(mut svg_dom) = svg_dom {
                canvas.save();
                canvas.translate((x, y));
                svg_dom.set_container_size((area.width() as i32, area.height() as i32));
                svg_dom.render(canvas);
                canvas.restore();
            }
        }
    }
}
