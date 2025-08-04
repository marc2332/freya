use freya_engine::prelude::*;
use freya_native_core::real_dom::NodeImmutable;
use torin::prelude::LayoutNode;

use super::utils::ElementUtils;
use crate::{
    dom::{
        DioxusNode,
        ImagesCache,
    },
    states::{
        FontStyleState,
        SvgState,
    },
};

pub struct SvgElement;

impl ElementUtils for SvgElement {
    fn render(
        self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        canvas: &Canvas,
        _font_collection: &mut FontCollection,
        font_manager: &FontMgr,
        _fallback_fonts: &[String],
        _images_cache: &mut ImagesCache,
        _scale_factor: f32,
    ) {
        let area = layout_node.visible_area();
        let svg_state = &*node_ref.get::<SvgState>().unwrap();
        let font_style = &*node_ref.get::<FontStyleState>().unwrap();

        let x = area.min_x();
        let y = area.min_y();
        if let Some(svg_data) = &svg_state.svg_data {
            let resource_provider = LocalResourceProvider::new(font_manager);
            let svg_dom = svg::Dom::from_bytes(svg_data.as_slice(), resource_provider);
            if let Ok(mut svg_dom) = svg_dom {
                canvas.save();
                canvas.translate((x, y));
                svg_dom.set_container_size((area.width() as i32, area.height() as i32));
                let mut root = svg_dom.root();
                root.set_width(svg::Length::new(100.0, svg::LengthUnit::Percentage));
                root.set_height(svg::Length::new(100.0, svg::LengthUnit::Percentage));
                root.set_color(font_style.color.into());
                if let Some(paint) = svg_state.svg_fill.as_ref() {
                    root.set_fill((*paint).into());
                }
                if let Some(paint) = svg_state.svg_stroke.as_ref() {
                    root.set_stroke((*paint).into());
                }
                svg_dom.render(canvas);
                canvas.restore();
            }
        }
    }
}
