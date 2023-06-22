use dioxus::prelude::*;
use freya_components::*;
use freya_core::prelude::*;
use freya_elements::elements as dioxus_elements;

use crate::{
    property::{BorderProperty, ColorfulProperty, Property, ShadowProperty},
    NodeInspectorBar, TreeNode,
};

#[allow(non_snake_case)]
#[inline_props]
pub fn NodeInspectorStyle<'a>(cx: Scope<'a>, node: &'a TreeNode) -> Element<'a> {
    render!(
         rect {
    overflow: "clip",
             width: "100%",
             height: "50%",
             NodeInspectorBar { }
             ScrollView {
                 show_scrollbar: true,
                 height: "calc(100% - 35)",
                 width: "100%",
                 node.state.iter().enumerate().map(|(i, (name, attr))| {
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
                         AttributeType::Paddings(paddings) => {
                             rsx!{
                                 Property {
                                     key: "{i}",
                                     name: "{name}",
                                     value: paddings.pretty()
                                 }
                             }
                         }
                         AttributeType::Radius(radius) => {
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
                         AttributeType::Color(color) => {
                             rsx!{
                                 ColorfulProperty {
                                     key: "{i}",
                                     name: "{name}",
                                     color: color
                                 }
                             }
                         }
                         AttributeType::Border(border) => {
                             rsx!{
                                 BorderProperty {
                                     key: "{i}",
                                     name: "{name}",
                                     border: border
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
                         AttributeType::Display(display) => {
                             rsx!{
                                 Property {
                                     key: "{i}",
                                     name: "{name}",
                                     value: display.pretty()
                                 }
                             }
                         }
                         AttributeType::Shadow(shadow) => {
                             rsx!{
                                 ShadowProperty {
                                     key: "{i}",
                                     name: "{name}",
                                     shadow: shadow
                                 }
                             }
                         }
                     }
                 })
             }
         }
     )
}
