use freya::prelude::{
    Element,
    rsx,
    *,
};
use freya_devtools::AttributeType;

use crate::property::{
    BorderProperty,
    ColorProperty,
    GradientProperty,
    Property,
    ShadowProperty,
    TextShadowProperty,
};

pub fn attribute_element(i: usize, name: &str, attribute: AttributeType<'_>) -> Option<Element> {
    let element = match attribute {
        AttributeType::Measure(measure) => {
            rsx! {
                Property {
                    key: "{i}",
                    name: "{name}",
                    value: measure.to_string()
                }
            }
        }
        AttributeType::OptionalMeasure(measure) => {
            rsx! {
                Property {
                    key: "{i}",
                    name: "{name}",
                    value: measure.map(|measure| measure.to_string()).unwrap_or_else(|| "inherit".to_string())
                }
            }
        }
        AttributeType::Measures(measures) => {
            rsx! {
                Property {
                    key: "{i}",
                    name: "{name}",
                    value: measures.pretty()
                }
            }
        }
        AttributeType::CornerRadius(radius) => {
            rsx! {
                Property {
                    key: "{i}",
                    name: "{name}",
                    value: radius.pretty()
                }
            }
        }
        AttributeType::Size(size) => {
            rsx! {
                Property {
                    key: "{i}",
                    name: "{name}",
                    value: size.pretty()
                }
            }
        }
        AttributeType::VisibleSize(visible_size) => {
            rsx! {
                Property {
                    key: "{i}",
                    name: "{name}",
                    value: visible_size.pretty()
                }
            }
        }
        AttributeType::Color(fill) => {
            rsx! {
                ColorProperty {
                    key: "{i}",
                    name: "{name}",
                    fill: fill.clone()
                }
            }
        }
        AttributeType::OptionalColor(fill) => {
            if let Some(fill) = fill {
                rsx! {
                    ColorProperty {
                        key: "{i}",
                        name: "{name}",
                        fill: fill.clone()
                    }
                }
            } else {
                return None;
            }
        }
        AttributeType::Gradient(fill) => {
            rsx! {
                GradientProperty {
                    key: "{i}",
                    name: "{name}",
                    fill: fill.clone()
                }
            }
        }
        AttributeType::Border(border) => {
            rsx! {
                BorderProperty {
                    key: "{i}",
                    name: "{name}",
                    border: border.clone()
                }
            }
        }
        AttributeType::Text(text) => {
            rsx! {
                Property {
                    key: "{i}",
                    name: "{name}",
                    value: text.to_string()
                }
            }
        }
        AttributeType::Direction(direction) => {
            rsx! {
                Property {
                    key: "{i}",
                    name: "{name}",
                    value: direction.pretty()
                }
            }
        }
        AttributeType::Position(position) => {
            rsx! {
                Property {
                    key: "{i}",
                    name: "{name}",
                    value: position.pretty()
                }
            }
        }
        AttributeType::Content(content) => {
            rsx! {
                Property {
                    key: "{i}",
                    name: "{name}",
                    value: content.pretty()
                }
            }
        }
        AttributeType::Alignment(alignment) => {
            rsx! {
                Property {
                    key: "{i}",
                    name: "{name}",
                    value: alignment.pretty()
                }
            }
        }
        AttributeType::Shadow(shadow) => {
            rsx! {
                ShadowProperty {
                    key: "{i}",
                    name: "{name}",
                    shadow: shadow.clone()
                }
            }
        }
        AttributeType::TextShadow(text_shadow) => {
            rsx! {
                TextShadowProperty {
                    key: "{i}",
                    name: "{name}",
                    text_shadow: *text_shadow
                }
            }
        }
        AttributeType::TextAlignment(text_align) => {
            rsx! {
                Property {
                    key: "{i}",
                    name: "{name}",
                    value: text_align.pretty()
                }
            }
        }
        AttributeType::TextOverflow(text_overflow) => {
            rsx! {
                Property {
                    key: "{i}",
                    name: "{name}",
                    value: text_overflow.pretty()
                }
            }
        }
    };

    Some(element)
}
