use dioxus::prelude::*;
use dioxus_native_core::NodeId;
use freya_components::*;
use freya_core::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{theme_with, ScrollViewThemeWith};

use crate::{
    hooks::use_selected_node,
    property::{
        BorderProperty, ColorProperty, LinearGradientProperty, Property, ShadowProperty,
        TextShadowProperty,
    },
    NodeInspectorBar,
};

#[allow(non_snake_case)]
#[component]
pub fn NodeInspectorStyle(node_id: NodeId) -> Element {
    let node = use_selected_node(&node_id);

    if let Some(node) = node {
        rsx!(
            rect {
                overflow: "clip",
                width: "100%",
                height: "50%",
                NodeInspectorBar {
                    node_id
                }
                ScrollView {
                    show_scrollbar: true,
                    theme: theme_with!(
                        ScrollViewTheme {
                            height : "calc(100% - 35)".into(),
                            width: "100%".into(),
                        }
                    ),
                    {node.state.iter().enumerate().map(|(i, (name, attr))| {
                        match attr {
                            AttributeType::Measure(measure) => {
                                rsx!{
                                    Property {
                                        key: "{i}",
                                        name: "{name}",
                                        value: measure.to_string()
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
                            AttributeType::Color(fill) => {
                                rsx!{
                                    ColorProperty {
                                        key: "{i}",
                                        name: "{name}",
                                        fill: fill.clone()
                                    }
                                }
                            }
                            AttributeType::LinearGradient(fill) => {
                                rsx!{
                                    LinearGradientProperty {
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
                        }
                    })}
                }
            }
        )
    } else {
        None
    }
}
