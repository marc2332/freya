use freya::prelude::*;
use freya_devtools::AttributeType;

use crate::property::{
    FillProperty,
    Property,
};

pub fn attributes_list(attributes: Vec<(&str, AttributeType)>) -> Element {
    ScrollView::new()
        .children(
            attributes
                .into_iter()
                .enumerate()
                .map(|(i, (name, attribute))| {
                    let background = if i % 2 == 0 {
                        Color::from_af32rgb(0.1, 255, 255, 255)
                    } else {
                        Color::TRANSPARENT
                    };
                    rect()
                        .key(i)
                        .background(background)
                        .padding((5., 16.))
                        .child(attribute_element(name, attribute))
                        .into()
                }),
        )
        .into()
}

pub fn attribute_element(name: &str, attribute: AttributeType<'_>) -> Element {
    match attribute {
        AttributeType::Measure(measure) => Property::new(name, measure.to_string()).into(),
        AttributeType::Measures(measures) => Property::new(name, measures.pretty()).into(),
        AttributeType::CornerRadius(radius) => Property::new(name, radius.pretty()).into(),
        AttributeType::Size(size) => Property::new(name, size.pretty()).into(),
        AttributeType::VisibleSize(visible_size) => {
            Property::new(name, visible_size.pretty()).into()
        }
        AttributeType::Color(color) => Property::new(name, "").swatch(color, color.pretty()).into(),
        AttributeType::Fill(fill) => FillProperty::new(name, fill).into(),
        AttributeType::Border(border) => Property::new(name, border.pretty())
            .swatch(border.fill, border.fill.pretty())
            .into(),
        AttributeType::Text(text) => Property::new(name, text).into(),
        AttributeType::Direction(direction) => Property::new(name, direction.pretty()).into(),
        AttributeType::Position(position) => Property::new(name, position.pretty()).into(),
        AttributeType::Content(content) => Property::new(name, content.pretty()).into(),
        AttributeType::Alignment(alignment) => Property::new(name, alignment.pretty()).into(),
        AttributeType::Shadow(shadow) => Property::new(name, shadow.to_string())
            .swatch(shadow.color, format!("{:?}", shadow.color))
            .into(),
        AttributeType::TextShadow(text_shadow) => Property::new(
            name,
            format!(
                "{} {} {}",
                text_shadow.offset.0, text_shadow.offset.1, text_shadow.blur_sigma
            ),
        )
        .swatch(text_shadow.color, format!("{:?}", text_shadow.color))
        .into(),
        AttributeType::TextAlignment(text_align) => Property::new(name, text_align.pretty()).into(),
        AttributeType::TextOverflow(text_overflow) => {
            Property::new(name, text_overflow.pretty()).into()
        }
        AttributeType::Length(length) => Property::new(name, length.get().to_string()).into(),
        AttributeType::TextHeightBehavior(text_height) => {
            Property::new(name, text_height.pretty()).into()
        }
        AttributeType::FontSlant(font_slant) => Property::new(name, font_slant.pretty()).into(),
        AttributeType::TextDecoration(text_decoration) => {
            Property::new(name, text_decoration.pretty()).into()
        }
    }
}
