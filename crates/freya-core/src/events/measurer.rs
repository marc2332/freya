use crate::{
    data::Interactive,
    element::EventMeasurementContext,
    events::{
        emittable::EmmitableEvent,
        name::EventName,
        platform::PlatformEvent,
    },
    node_id::NodeId,
    prelude::Color,
    style::fill::Fill,
    tree::Tree,
};

pub struct EventsMeasurerAdapter<'a> {
    pub tree: &'a mut Tree,
    pub scale_factor: f64,
}

impl ragnarok::EventsMeasurer for EventsMeasurerAdapter<'_> {
    type Key = NodeId;
    type Name = EventName;
    type Source = PlatformEvent;
    type Emmitable = EmmitableEvent;

    fn get_listeners_of(&self, name: &Self::Name) -> impl Iterator<Item = &Self::Key> {
        self.tree
            .listeners
            .get(name)
            .map(|l| l.iter())
            .unwrap_or_else(|| [].iter())
    }

    fn is_listening_to(&self, key: &Self::Key, name: &Self::Name) -> bool {
        self.tree
            .listeners
            .get(name)
            .map(|listeners| listeners.contains(key))
            .unwrap_or_default()
    }

    fn get_layers(&self) -> impl Iterator<Item = (&i16, impl Iterator<Item = &Self::Key>)> {
        self.tree
            .layers
            .iter()
            .map(|(layer, nodes)| (layer, nodes.iter()))
    }

    fn is_point_inside(&self, key: &Self::Key, cursor: ragnarok::CursorPoint) -> bool {
        let element = self.tree.elements.get(key).unwrap();
        let Some(layout_node) = self.tree.layout.get(key) else {
            return false;
        };

        // Make sure the cursor is inside the element
        if !element.is_point_inside(EventMeasurementContext {
            cursor,
            layout_node,
            scale_factor: self.scale_factor,
        }) {
            return false;
        }

        let effect_state = self.tree.effect_state.get(key);

        if let Some(effect_state) = effect_state {
            // Make sure the cursor is inside all the inherited clips of the element
            for node_id in effect_state.clips.iter() {
                let element = self.tree.elements.get(node_id).unwrap();
                let layout_node = self.tree.layout.get(node_id).unwrap();
                if !element.is_point_inside(EventMeasurementContext {
                    cursor,
                    layout_node,
                    scale_factor: self.scale_factor,
                }) {
                    return false;
                }
            }
        }

        true
    }

    fn is_node_parent_of(&self, key: &Self::Key, parent: Self::Key) -> bool {
        let mut head = Some(key);
        while let Some(id) = head.take() {
            if let Some(parent_id) = self.tree.parents.get(id) {
                if *parent_id == parent {
                    return true;
                }

                head = Some(parent_id)
            }
        }
        false
    }

    fn is_node_transparent(&self, key: &Self::Key) -> bool {
        let element = self.tree.elements.get(key).unwrap();
        if element.style().background == Fill::Color(Color::TRANSPARENT) {
            return true;
        }
        if let Some(effect_state) = self.tree.effect_state.get(key) {
            return effect_state.interactive == Interactive::No;
        }
        false
    }

    fn try_area_of(&self, key: &Self::Key) -> Option<ragnarok::Area> {
        self.tree
            .layout
            .get(key)
            .map(|layout| layout.visible_area())
    }

    fn new_emmitable_event(
        &self,
        key: Self::Key,
        name: Self::Name,
        source: Self::Source,
        area: Option<ragnarok::Area>,
    ) -> Self::Emmitable {
        EmmitableEvent::new(key, name, source, area, self.scale_factor)
    }
}
