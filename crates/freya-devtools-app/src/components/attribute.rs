use freya::prelude::*;
use freya_devtools::AttributeType;

use crate::property::{
    BorderProperty,
    ColorProperty,
    GradientProperty,
    Property,
    ShadowProperty,
    TextShadowProperty,
};

pub fn attribute_element(name: &str, attribute: AttributeType<'_>) -> Option<Element> {
    match attribute {
        AttributeType::Measure(measure) => Some(Property::new(name, measure.to_string()).into()),
        AttributeType::OptionalMeasure(measure) => Some(
            Property::new(
                name,
                measure
                    .map(|measure| measure.to_string())
                    .unwrap_or_else(|| "inherit".to_string()),
            )
            .into(),
        ),
        AttributeType::Measures(measures) => Some(Property::new(name, measures.pretty()).into()),
        AttributeType::CornerRadius(radius) => Some(Property::new(name, radius.pretty()).into()),
        AttributeType::Size(size) => Some(Property::new(name, size.pretty()).into()),
        AttributeType::VisibleSize(visible_size) => {
            Some(Property::new(name, visible_size.pretty()).into())
        }
        AttributeType::Color(color) => Some(ColorProperty::new(name, color).into()),
        AttributeType::OptionalColor(fill) => {
            fill.map(|color| ColorProperty::new(name, color).into())
        }
        AttributeType::Gradient(fill) => Some(GradientProperty::new(name, fill).into()),
        AttributeType::Border(border) => Some(BorderProperty::new(name, border.clone()).into()),
        AttributeType::Text(text) => Some(Property::new(name, text).into()),
        AttributeType::Direction(direction) => Some(Property::new(name, direction.pretty()).into()),
        AttributeType::Position(position) => Some(Property::new(name, position.pretty()).into()),
        AttributeType::Content(content) => Some(Property::new(name, content.pretty()).into()),
        AttributeType::Alignment(alignment) => Some(Property::new(name, alignment.pretty()).into()),
        AttributeType::Shadow(shadow) => Some(ShadowProperty::new(name, shadow.clone()).into()),
        AttributeType::TextShadow(text_shadow) => {
            Some(TextShadowProperty::new(name, *text_shadow).into())
        }
        AttributeType::TextAlignment(text_align) => {
            Some(Property::new(name, text_align.pretty()).into())
        }
        AttributeType::TextOverflow(text_overflow) => {
            Some(Property::new(name, text_overflow.pretty()).into())
        }
        AttributeType::Length(length) => Some(Property::new(name, length.get().to_string()).into()),
        AttributeType::TextHeightBehavior(text_height) => {
            Some(Property::new(name, text_height.pretty()).into())
        }
        AttributeType::FontSlant(font_slant) => {
            Some(Property::new(name, font_slant.pretty()).into())
        }
        AttributeType::Layer(layer) => Some(Property::new(name, layer.to_string()).into()),
    }
}
