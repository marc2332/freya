use dioxus::prelude::*;
use freya_components::*;
use freya_core::node::{
    AttributeType,
    ExternalPretty,
};
use freya_elements::{
    self as dioxus_elements,
};
use freya_native_core::NodeId;

use crate::{
    hooks::use_node_info,
    property::{
        BorderProperty,
        ColorProperty,
        GradientProperty,
        Property,
        ShadowProperty,
        TextShadowProperty,
    },
};

#[allow(non_snake_case)]
#[component]
pub fn NodeInspectorStyle(node_id: String) -> Element {
    let node_id = NodeId::deserialize(&node_id);
    let Some(node) = use_node_info(node_id) else {
        return Ok(VNode::placeholder());
    };

    rsx!(
        ScrollView {
            show_scrollbar: true,
            height : "fill",
            width: "fill",
            {node.state.attributes().into_iter().enumerate().filter_map(|(i, (name, attr))| {
                let background = if i % 2 == 0 {
                    "rgb(255, 255, 255, 0.1)"
                } else {
                    "transparent"
                };

                let el = match attr {
                    AttributeType::Measure(measure) => {
                        rsx!{
                            Property {
                                key: "{i}",
                                name: "{name}",
                                value: measure.to_string()
                            }
                        }
                    }
                    AttributeType::OptionalMeasure(measure) => {
                        rsx!{
                            Property {
                                key: "{i}",
                                name: "{name}",
                                value: measure.map(|measure| measure.to_string()).unwrap_or_else(|| "inherit".to_string())
                            }
                        }
                    }
                    AttributeType::Measures(measures) => {
                        rsx!{
                            Property {
                                key: "{i}",
                                name: "{name}",
                                value: measures.pretty()
                            }
                        }
                    }
                    AttributeType::CornerRadius(radius) => {
                        rsx!{
                            Property {
                                key: "{i}",
                                name: "{name}",
                                value: radius.pretty()
                            }
                        }
                    }
                    AttributeType::Size(size) => {
                        rsx!{
                            Property {
                                key: "{i}",
                                name: "{name}",
                                value: size.pretty()
                            }
                        }
                    }
                    AttributeType::VisibleSize(visible_size) => {
                        rsx!{
                            Property {
                                key: "{i}",
                                name: "{name}",
                                value: visible_size.pretty()
                            }
                        }
                    }
                    AttributeType::Color(fill) => {
                        rsx!{
                            ColorProperty {
                                key: "{i}",
                                name: "{name}",
                                fill: fill.clone()
                            }
                        }
                    }
                    AttributeType::OptionalColor(fill) => {
                        if let Some(fill) = fill {
                            rsx!{
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
                        rsx!{
                            GradientProperty {
                                key: "{i}",
                                name: "{name}",
                                fill: fill.clone()
                            }
                        }
                    }
                    AttributeType::Border(border) => {
                        rsx!{
                            BorderProperty {
                                key: "{i}",
                                name: "{name}",
                                border: border.clone()
                            }
                        }
                    }
                    AttributeType::Text(text) => {
                        rsx!{
                            Property {
                                key: "{i}",
                                name: "{name}",
                                value: text.to_string()
                            }
                        }
                    }
                    AttributeType::Direction(direction) => {
                        rsx!{
                            Property {
                                key: "{i}",
                                name: "{name}",
                                value: direction.pretty()
                            }
                        }
                    }
                    AttributeType::Position(position) => {
                        rsx!{
                            Property {
                                key: "{i}",
                                name: "{name}",
                                value: position.pretty()
                            }
                        }
                    }
                    AttributeType::Content(content) => {
                        rsx!{
                            Property {
                                key: "{i}",
                                name: "{name}",
                                value: content.pretty()
                            }
                        }
                    }
                    AttributeType::Alignment(alignment) => {
                        rsx!{
                            Property {
                                key: "{i}",
                                name: "{name}",
                                value: alignment.pretty()
                            }
                        }
                    }
                    AttributeType::Shadow(shadow) => {
                        rsx!{
                            ShadowProperty {
                                key: "{i}",
                                name: "{name}",
                                shadow: shadow.clone()
                            }
                        }
                    }
                    AttributeType::TextShadow(text_shadow) => {
                        rsx!{
                            TextShadowProperty {
                                key: "{i}",
                                name: "{name}",
                                text_shadow: *text_shadow
                            }
                        }
                    }
                    AttributeType::TextAlignment(text_align) => {
                        rsx!{
                            Property {
                                key: "{i}",
                                name: "{name}",
                                value: text_align.pretty()
                            }
                        }
                    }
                    AttributeType::TextOverflow(text_overflow) => {
                        rsx!{
                            Property {
                                key: "{i}",
                                name: "{name}",
                                value: text_overflow.pretty()
                            }
                        }
                    }
                };



                Some(rsx!(
                    rect {
                        background,
                        padding: "5 16",
                        {el}
                    }
                ))
            })}
        }
    )
}
