use dioxus_native_core::prelude::ElementNode;
use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::{node::NodeType, NodeId};
use freya_common::NodeReferenceLayout;
use freya_dom::prelude::DioxusDOM;
use freya_layout::Layers;

use freya_node_state::{CursorMode, CursorSettings, References, SizeState, Style};
use rustc_hash::FxHashMap;
use skia_safe::textlayout::FontCollection;
use torin::torin::Torin;

pub fn process_layers(
    layers: &mut Layers,
    rdom: &DioxusDOM,
    layout: &Torin<NodeId>,
    font_collection: &FontCollection,
    scale_factor: f32,
) {
    let mut inherit_layers = FxHashMap::default();

    rdom.traverse_depth_first(|node| {
        let areas = layout.get(node.id());

        if let Some(areas) = areas {
            // Add the Node to a Layer
            let node_style = node.get::<Style>().unwrap();

            let inherited_relative_layer = node
                .parent_id()
                .map(|p| *inherit_layers.get(&p).unwrap())
                .unwrap_or(0);

            let (node_layer, node_relative_layer) = Layers::calculate_layer(
                node_style.relative_layer,
                node.height() as i16,
                inherited_relative_layer,
            );

            inherit_layers.insert(node.id(), node_relative_layer);
            layers.add_element(node.id(), node_layer);

            // Register paragraph elements

            if let NodeType::Element(ElementNode { tag, .. }) = &*node.node_type() {
                if tag == "paragraph" {
                    let cursor_settings = node.get::<CursorSettings>().unwrap();
                    let is_editable = CursorMode::Editable == cursor_settings.mode;

                    let references = node.get::<References>().unwrap();
                    if is_editable {
                        if let Some(cursor_ref) = &references.cursor_ref {
                            let text_group = layers
                                .paragraph_elements
                                .entry(cursor_ref.text_id)
                                .or_insert_with(Vec::default);

                            text_group.push(node.id());
                        }
                    }
                }
            }

            // Notify layout references

            let size_state = &*node.get::<SizeState>().unwrap();

            if let Some(reference) = &size_state.node_ref {
                let mut node_layout = NodeReferenceLayout {
                    area: areas.area,
                    inner: areas.inner_sizes,
                };
                node_layout.div(scale_factor);
                reference.send(node_layout).ok();
            }
        }
    });

    layers.measure_all_paragraph_elements(rdom, layout, font_collection);
}
