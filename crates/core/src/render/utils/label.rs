use freya_engine::prelude::*;
use freya_native_core::{
    prelude::NodeType,
    real_dom::NodeImmutable,
};
use freya_node_state::FontStyleState;
use torin::prelude::Size2D;

use super::ParagraphData;
use crate::dom::*;

pub fn create_label(
    node: &DioxusNode,
    area_size: &Size2D,
    font_collection: &FontCollection,
    default_font_family: &[String],
    scale_factor: f32,
) -> ParagraphData {
    let font_style = &*node.get::<FontStyleState>().unwrap();

    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.set_text_align(font_style.text_align);
    paragraph_style.set_max_lines(font_style.max_lines);
    paragraph_style.set_replace_tab_characters(true);
    paragraph_style.set_text_height_behavior(font_style.text_height);

    if let Some(ellipsis) = font_style.text_overflow.get_ellipsis() {
        paragraph_style.set_ellipsis(ellipsis);
    }

    let text_style =
        font_style.text_style(default_font_family, scale_factor, font_style.text_height);
    paragraph_style.set_text_style(&text_style);

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

    for child in node.children() {
        if let NodeType::Text(text) = &*child.node_type() {
            paragraph_builder.add_text(text);
        }
    }

    let mut paragraph = paragraph_builder.build();
    paragraph.layout(
        if font_style.max_lines == Some(1) && font_style.text_align == TextAlign::default() {
            f32::MAX
        } else {
            area_size.width + 1.0
        },
    );

    let width = match font_style.text_align {
        TextAlign::Start | TextAlign::Left => paragraph.longest_line(),
        _ => paragraph.max_width(),
    };

    ParagraphData {
        size: Size2D::new(width, paragraph.height()),
        paragraph,
    }
}
