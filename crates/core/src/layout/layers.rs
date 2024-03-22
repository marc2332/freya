use dioxus_native_core::NodeId;
use freya_dom::dom::{DioxusDOM, FreyaDOM};
use rustc_hash::FxHashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use torin::torin::Torin;
use uuid::Uuid;

use super::measure_paragraph;

#[derive(Default, Clone)]
pub struct Layers {
    pub layers: Arc<Mutex<FxHashMap<i16, Vec<NodeId>>>>,
    pub paragraph_elements: Arc<Mutex<FxHashMap<Uuid, Vec<NodeId>>>>,
}

impl Layers {
    pub fn new() -> Self {
        let layers = Layers::default();

        // Register paragraph elements

        // Notify layout references

        // let size_state = &*node.get::<LayoutState>().unwrap();

        // if let Some(reference) = &size_state.node_ref {
        //     let mut node_layout = NodeReferenceLayout {
        //         area: layout_node.area,
        //         inner: layout_node.inner_sizes,
        //     };
        //     node_layout.div(scale_factor);
        //     reference.0.send(node_layout).ok();
        // }

        //     layers.measure_all_paragraph_elements(rdom, layout, scale_factor);

        layers
    }

    pub fn layers(&self) -> MutexGuard<FxHashMap<i16, Vec<NodeId>>> {
        self.layers.lock().unwrap()
    }

    pub fn len_paragraph_elements(&self) -> usize {
        self.paragraph_elements.lock().unwrap().len()
    }

    pub fn len_layers(&self) -> usize {
        self.layers.lock().unwrap().len()
    }

    /// Measure all the paragraphs
    pub fn measure_all_paragraph_elements(
        &self,
        rdom: &DioxusDOM,
        layout: &Torin<NodeId>,
        scale_factor: f32,
    ) {
        for group in self.paragraph_elements.lock().unwrap().values() {
            for node_id in group {
                let node = rdom.get(*node_id);
                let layout_node = layout.get(*node_id);
                if let Some((node, layout_node)) = node.zip(layout_node) {
                    measure_paragraph(&node, layout_node, true, scale_factor);
                }
            }
        }
    }

    /// Measure all the paragraphs registered under the given TextId
    pub fn measure_paragraph_elements(&self, text_id: &Uuid, fdom: &FreyaDOM, scale_factor: f32) {
        let paragraphs = self.paragraph_elements.lock().unwrap();
        let group = paragraphs.get(text_id);
        let layout = fdom.layout();
        if let Some(group) = group {
            for node_id in group {
                let node = fdom.rdom().get(*node_id);
                let layout_node = layout.get(*node_id);

                if let Some((node, layout_node)) = node.zip(layout_node) {
                    measure_paragraph(&node, layout_node, true, scale_factor);
                }
            }
        }
    }
}
