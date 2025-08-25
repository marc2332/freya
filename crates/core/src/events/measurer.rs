use dioxus_core::VirtualDom;
use freya_native_core::{
    events::EventName,
    prelude::NodeImmutable,
    tree::TreeRef,
    NodeId,
};
use torin::torin::Torin;

use crate::{
    dom::DioxusDOM,
    elements::{
        ElementUtils,
        ElementUtilsResolver,
    },
    events::{
        DomEvent,
        PlatformEvent,
    },
    layers::Layers,
    states::{
        StyleState,
        ViewportState,
    },
    values::{
        Color,
        Fill,
    },
};

pub struct EventsMeasurerAdapter<'a> {
    pub rdom: &'a DioxusDOM,
    pub vdom: &'a mut VirtualDom,
    pub layers: &'a Layers,
    pub layout: &'a Torin<NodeId>,
    pub scale_factor: f64,
}

impl ragnarok::EventsMeasurer for EventsMeasurerAdapter<'_> {
    type Key = NodeId;
    type Name = EventName;
    type Source = PlatformEvent;
    type Emmitable = DomEvent;

    fn get_listeners_of(&self, name: &Self::Name) -> Vec<Self::Key> {
        self.rdom
            .get_listeners(name)
            .into_iter()
            .map(|n| n.id())
            .collect::<Vec<_>>()
    }

    fn is_listening_to(&self, key: Self::Key, name: &Self::Name) -> bool {
        self.rdom.is_node_listening(&key, name)
    }

    fn get_layers(&self) -> impl Iterator<Item = (&i16, impl Iterator<Item = &Self::Key>)> {
        self.layers
            .iter()
            .map(|(layer, nodes)| (layer, nodes.iter()))
    }

    fn is_point_inside(&self, key: Self::Key, cursor: ragnarok::CursorPoint) -> bool {
        let Some(node_ref) = self.rdom.get(key) else {
            return false;
        };
        let node_type = node_ref.node_type();

        let Some(element_utils) = node_type.tag().and_then(|tag| tag.utils()) else {
            return false;
        };

        let Some(layout_node) = self.layout.get(key) else {
            return false;
        };

        // Make sure the cursor is inside the node area
        if !element_utils.is_point_inside_area(
            &cursor,
            &node_ref,
            layout_node,
            self.scale_factor as f32,
        ) {
            return false;
        }

        let node_viewports = node_ref.get::<ViewportState>().unwrap();

        // Make sure the cursor is inside all the inherited viewports of the node
        for node_id in &node_viewports.viewports {
            let node_ref = self.rdom.get(*node_id).unwrap();
            let node_type = node_ref.node_type();
            let Some(element_utils) = node_type.tag().and_then(|tag| tag.utils()) else {
                continue;
            };
            let layout_node = self.layout.get(*node_id).unwrap();
            if !element_utils.is_point_inside_area(
                &cursor,
                &node_ref,
                layout_node,
                self.scale_factor as f32,
            ) {
                return false;
            }
        }

        true
    }

    fn is_node_parent_of(&self, key: Self::Key, parent: Self::Key) -> bool {
        let mut head = Some(key);
        while let Some(id) = head.take() {
            let tree = self.rdom.tree_ref();
            if let Some(parent_id) = tree.parent_id(id) {
                if parent_id == parent {
                    return true;
                }

                head = Some(parent_id)
            }
        }
        false
    }

    fn is_node_transparent(&self, key: Self::Key) -> bool {
        let Some(node_ref) = self.rdom.get(key) else {
            return false;
        };
        let StyleState { background, .. } = &*node_ref.get::<StyleState>().unwrap();

        background == &Fill::Color(Color::TRANSPARENT)
    }

    fn try_area_of(&self, key: Self::Key) -> Option<ragnarok::Area> {
        self.layout.get(key).map(|layout| layout.visible_area())
    }

    fn new_emmitable_event(
        &self,
        key: Self::Key,
        name: Self::Name,
        source: Self::Source,
        area: Option<ragnarok::Area>,
    ) -> Self::Emmitable {
        DomEvent::new(key, name, source, area, self.scale_factor)
    }
}
