use freya_engine::prelude::*;
use freya_native_core::real_dom::NodeImmutable;
use freya_node_state::{
    Fill,
    StyleState,
};
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
        let area = layout_node.visible_area();
        let node_style = &*node_ref.get::<StyleState>().unwrap();

        let x = area.min_x();
        let y = area.min_y();
        if let Some(svg_data) = &node_style.svg_data {
            let svg_dom = svg::Dom::from_bytes(svg_data.as_slice(), font_manager);
            if let Ok(svg_dom) = svg_dom {
                if node_style.fill.is_some() {
                    canvas.save_layer(&SaveLayerRec::default());
                } else {
                    canvas.save();
                }

                canvas.translate((x, y));
                canvas.scale((
                    area.width() / svg_dom.inner().fContainerSize.fWidth,
                    area.height() / svg_dom.inner().fContainerSize.fHeight,
                ));
                svg_dom.render(canvas);

                if let Some(fill) = node_style.fill.as_ref() {
                    let mut paint = Paint::default();

                    paint.set_anti_alias(true);
                    paint.set_blend_mode(BlendMode::SrcIn);

                    match fill {
                        Fill::Color(color) => {
                            paint.set_color(*color);
                        }
                        Fill::LinearGradient(gradient) => {
                            paint.set_shader(gradient.into_shader(area));
                        }
                    }

                    canvas.draw_paint(&paint);
                }

                canvas.restore();
            }
        }
    }
}
