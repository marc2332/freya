use freya_engine::prelude::*;
use freya_native_core::{
    prelude::NodeType,
    real_dom::NodeImmutable,
};
use torin::{
    node::Node,
    prelude::{
        Phase,
        Size2D,
    },
};

use super::ParagraphData;
use crate::{
    dom::*,
    states::FontStyleState,
    values::TextAlign,
};

pub fn create_label(
    node: &DioxusNode,
    torin_node: &Node,
    area_size: &Size2D,
    font_collection: &FontCollection,
    fallback_fonts: &[String],
    scale_factor: f32,
) -> ParagraphData {
    let font_style = &*node.get::<FontStyleState>().unwrap();

    let mut paragraph_style = ParagraphStyle::default();
    paragraph_style.set_text_align(font_style.text_align.into());
    paragraph_style.set_max_lines(font_style.max_lines);
    paragraph_style.set_replace_tab_characters(true);
    paragraph_style.set_text_height_behavior(font_style.text_height.into());

    if let Some(ellipsis) = font_style.text_overflow.get_ellipsis() {
        paragraph_style.set_ellipsis(ellipsis);
    }

    let text_style = font_style.text_style(fallback_fonts, scale_factor, font_style.text_height);
    paragraph_style.set_text_style(&text_style);

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

    for child in node.children() {
        if let NodeType::Text(text) = &*child.node_type() {
            paragraph_builder.add_text(text);
        }
    }

    let mut paragraph = paragraph_builder.build();
    paragraph.layout(
        if font_style.max_lines == Some(1)
            && font_style.text_align == TextAlign::default()
            && !paragraph_style.ellipsized()
        {
            f32::MAX
        } else {
            area_size.width + 1.0
        },
    );

    // Relayout the paragraph so that its aligned based on its longest width
    match font_style.text_align {
        TextAlign::Center | TextAlign::Justify | TextAlign::Right | TextAlign::End
            if torin_node.width.inner_sized(Phase::Initial) =>
        {
            paragraph.layout(paragraph.longest_line() + 1.);
        }
        _ => {}
    }

    ParagraphData {
        size: Size2D::new(paragraph.longest_line(), paragraph.height()),
        paragraph,
    }
}
