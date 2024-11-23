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
            let resource_provider = LocalResourceProvider::new(font_manager);
            let svg_dom = svg::Dom::from_bytes(svg_data.as_slice(), resource_provider);
            if let Ok(mut svg_dom) = svg_dom {
                let (scale_x, scale_y) = (
                    area.width() / svg_dom.inner().fContainerSize.fWidth,
                    area.height() / svg_dom.inner().fContainerSize.fHeight,
                );

                canvas.save_layer(&SaveLayerRec::default());
                canvas.translate((x, y));
                svg_dom.set_container_size((area.width() as i32, area.height() as i32));
                let mut root = svg_dom.root();
                root.set_width(svg::Length::new(100.0, svg::LengthUnit::Percentage));
                root.set_height(svg::Length::new(100.0, svg::LengthUnit::Percentage));
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
                        Fill::RadialGradient(gradient) => {
                            paint.set_shader(gradient.into_shader(area));
                        }
                        Fill::ConicGradient(gradient) => {
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
