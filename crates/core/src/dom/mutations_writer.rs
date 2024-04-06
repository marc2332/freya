use dioxus_core::{ElementId, WriteMutations};
use freya_common::{Layers, ParagraphElements};
use freya_native_core::{
    dioxus::DioxusNativeCoreMutationWriter, prelude::NodeImmutable, tree::TreeRef, NodeId,
};
use freya_node_state::{CursorSettings, CustomAttributeValues, LayerState};
use torin::torin::Torin;

use crate::prelude::DioxusDOMAdapter;

pub struct MutationsWriter<'a> {
    pub native_writer: DioxusNativeCoreMutationWriter<'a, CustomAttributeValues>,
    pub layout: &'a mut Torin<NodeId>,
    pub layers: &'a Layers,
    pub paragraphs: &'a ParagraphElements,
}

impl<'a> MutationsWriter<'a> {
    pub fn remove(&mut self, id: ElementId) {
        let node_id = self.native_writer.state.element_to_node_id(id);
        let mut dom_adapter = DioxusDOMAdapter::new_with_cache(self.native_writer.rdom);

        // Remove from layout
        self.layout.remove(node_id, &mut dom_adapter, true);

        // Remove from layers and paragraph elements
        let mut stack = vec![node_id];
        let tree = self.native_writer.rdom.tree_ref();
        while let Some(node_id) = stack.pop() {
            if let Some(node) = self.native_writer.rdom.get(node_id) {
                if !node.node_type().is_visible_element() {
                    continue;
                }

                let traverse_children = node
                    .node_type()
                    .tag()
                    .map(|tag| tag.has_children_with_intrinsic_layout())
                    .unwrap_or_default();
                if traverse_children {
                    let children = tree.children_ids_advanced(node_id, false);
                    stack.extend(children.iter().copied().rev());
                }

                // Remove from layers
                let layer_state = node.get::<LayerState>().unwrap();
                self.layers
                    .remove_node_from_layer(node_id, layer_state.layer);

                // Remove from paragraph elements
                let cursor_settings = node.get::<CursorSettings>().unwrap();
                if let Some(cursor_ref) = cursor_settings.cursor_ref.as_ref() {
                    self.paragraphs
                        .remove_paragraph(node_id, &cursor_ref.text_id);
                }
            }
        }
    }
}

impl<'a> WriteMutations for MutationsWriter<'a> {
    fn register_template(&mut self, template: dioxus_core::prelude::Template) {
        self.native_writer.register_template(template);
    }

    fn append_children(&mut self, id: dioxus_core::ElementId, m: usize) {
        self.native_writer.append_children(id, m);
    }

    fn assign_node_id(&mut self, path: &'static [u8], id: dioxus_core::ElementId) {
        self.native_writer.assign_node_id(path, id);
    }

    fn create_placeholder(&mut self, id: dioxus_core::ElementId) {
        self.native_writer.create_placeholder(id);
    }

    fn create_text_node(&mut self, value: &str, id: dioxus_core::ElementId) {
        self.native_writer.create_text_node(value, id);
    }

    fn hydrate_text_node(&mut self, path: &'static [u8], value: &str, id: dioxus_core::ElementId) {
        self.native_writer.hydrate_text_node(path, value, id);
    }

    fn load_template(&mut self, name: &'static str, index: usize, id: dioxus_core::ElementId) {
        self.native_writer.load_template(name, index, id);
    }

    fn replace_node_with(&mut self, id: dioxus_core::ElementId, m: usize) {
        if m > 0 {
            self.remove(id);
        }

        self.native_writer.replace_node_with(id, m);
    }

    fn replace_placeholder_with_nodes(&mut self, path: &'static [u8], m: usize) {
        self.native_writer.replace_placeholder_with_nodes(path, m);
    }

    fn insert_nodes_after(&mut self, id: dioxus_core::ElementId, m: usize) {
        self.native_writer.insert_nodes_after(id, m);
    }

    fn insert_nodes_before(&mut self, id: dioxus_core::ElementId, m: usize) {
        self.native_writer.insert_nodes_before(id, m);
    }

    fn set_attribute(
        &mut self,
        name: &'static str,
        ns: Option<&'static str>,
        value: &dioxus_core::AttributeValue,
        id: dioxus_core::ElementId,
    ) {
        self.native_writer.set_attribute(name, ns, value, id);
    }

    fn set_node_text(&mut self, value: &str, id: dioxus_core::ElementId) {
        self.layout
            .invalidate(self.native_writer.state.element_to_node_id(id));
        self.native_writer.set_node_text(value, id);
    }

    fn create_event_listener(&mut self, name: &'static str, id: dioxus_core::ElementId) {
        self.native_writer.create_event_listener(name, id);
    }

    fn remove_event_listener(&mut self, name: &'static str, id: dioxus_core::ElementId) {
        self.native_writer.remove_event_listener(name, id);
    }

    fn remove_node(&mut self, id: dioxus_core::ElementId) {
        self.remove(id);
        self.native_writer.remove_node(id);
    }

    fn push_root(&mut self, id: dioxus_core::ElementId) {
        self.native_writer.push_root(id);
    }
}
