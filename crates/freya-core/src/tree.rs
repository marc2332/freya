use std::{
    any::Any,
    borrow::Cow,
    collections::{
        VecDeque,
        hash_map::Entry,
    },
    fmt::Debug,
    rc::Rc,
};

use bitflags::bitflags;
use freya_engine::prelude::{
    FontCollection,
    FontMgr,
};
use futures_channel::mpsc::UnboundedSender;
use itertools::Itertools;
use rustc_hash::{
    FxHashMap,
    FxHashSet,
};
use torin::{
    prelude::{
        Area,
        LayoutMeasurer,
        Size2D,
    },
    torin::{
        DirtyReason,
        Torin,
    },
};

use crate::{
    accessibility::groups::AccessibilityGroups,
    data::{
        AccessibilityState,
        EffectState,
        LayerState,
        TextStyleState,
    },
    element::{
        ElementExt,
        LayoutContext,
    },
    elements::rect::RectElement,
    events::{
        data::{
            EventType,
            SizedEventData,
        },
        emittable::EmmitableEvent,
        name::EventName,
    },
    extended_hashmap::ExtendedHashMap,
    integration::{
        AccessibilityDirtyNodes,
        AccessibilityGenerator,
        EventsChunk,
    },
    layers::Layers,
    node_id::NodeId,
    runner::{
        MutationRemove,
        Mutations,
    },
    text_cache::TextCache,
    tree_layout_adapter::TreeAdapterFreya,
};

#[derive(Default)]
pub struct Tree {
    pub parents: FxHashMap<NodeId, NodeId>,
    pub children: FxHashMap<NodeId, Vec<NodeId>>,
    pub heights: FxHashMap<NodeId, u16>,

    pub elements: FxHashMap<NodeId, Rc<dyn ElementExt>>,

    // Event listeners
    pub listeners: FxHashMap<EventName, Vec<NodeId>>,

    // Derived states
    pub layer_state: FxHashMap<NodeId, LayerState>,
    pub accessibility_state: FxHashMap<NodeId, AccessibilityState>,
    pub effect_state: FxHashMap<NodeId, EffectState>,
    pub text_style_state: FxHashMap<NodeId, TextStyleState>,

    // Other
    pub layout: Torin<NodeId>,
    pub layers: Layers,
    pub text_cache: TextCache,

    // Accessibility
    pub accessibility_groups: AccessibilityGroups,
    pub accessibility_diff: AccessibilityDirtyNodes,
    pub accessibility_generator: AccessibilityGenerator,
}

impl Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Parents: {:#?}\nChildren: {:#?}\nHeights: {:#?}",
            self.parents, self.children, self.heights,
        ))
    }
}

impl Tree {
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn traverse_depth(&self, mut then: impl FnMut(NodeId)) {
        let mut buffer = vec![NodeId::ROOT];
        while let Some(node_id) = buffer.pop() {
            if let Some(children) = self.children.get(&node_id) {
                buffer.extend(children.iter().rev());
            }
            then(node_id);
        }
    }

    pub fn traverse_depth_cancel(&self, mut then: impl FnMut(NodeId) -> bool) {
        let mut buffer = vec![NodeId::ROOT];
        while let Some(node_id) = buffer.pop() {
            if let Some(children) = self.children.get(&node_id) {
                buffer.extend(children.iter().rev());
            }
            if then(node_id) {
                break;
            }
        }
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn apply_mutations(&mut self, mutations: Mutations) -> MutationsApplyResult {
        let mut needs_render = !mutations.removed.is_empty();
        let mut dirty = Vec::<(NodeId, DiffModifies)>::default();

        #[cfg(debug_assertions)]
        tracing::info!("{mutations:?}");

        if let Entry::Vacant(e) = self.elements.entry(NodeId::ROOT) {
            e.insert(Rc::new(RectElement::default()));
            self.heights.insert(NodeId::ROOT, 0);
            dirty.push((NodeId::ROOT, DiffModifies::all()));
        }

        hotpath::measure_block!("mutations run", {
            for remove in mutations.removed {
                let node_id = remove.node_id();
                let mut buff = vec![remove];
                let Some(parent_id) = self.parents.get(&node_id).copied() else {
                    continue;
                };
                self.layout.invalidate(parent_id);
                needs_render = true;

                while let Some(remove) = buff.pop() {
                    let node_id = remove.node_id();
                    self.layout.raw_remove(node_id);

                    let parent_id = self.parents.remove(&node_id).unwrap();

                    // Remove element
                    let old_element = self.elements.remove(&node_id).unwrap();

                    if let Some(children) = self.children.get_mut(&parent_id) {
                        match remove {
                            MutationRemove::Element { index, .. } => {
                                children.remove(index as usize);
                            }
                            MutationRemove::Scope { .. } => {
                                children.retain(|id| *id != node_id);
                            }
                        }
                    }

                    // Remove its children too
                    if let Some(children) = self.children.remove(&node_id) {
                        buff.extend(children.into_iter().enumerate().map(|(i, e)| {
                            MutationRemove::Element {
                                id: e,
                                index: i as u32,
                            }
                        }));
                    }

                    // Remove old events
                    if let Some(events) = old_element.events_handlers() {
                        for event in events.keys() {
                            self.listeners
                                .entry(*event)
                                .or_default()
                                .retain(|id| *id != node_id);
                        }
                    }

                    // Remove from the layers
                    let layer_state = self.layer_state.remove(&node_id).unwrap();
                    layer_state.remove(node_id, &mut self.layers);

                    // Remove from the accessibility
                    let accessibility_state = self.accessibility_state.remove(&node_id).unwrap();
                    accessibility_state.remove(
                        node_id,
                        parent_id,
                        &mut self.accessibility_diff,
                        &mut self.accessibility_groups,
                    );

                    // Remove from other states
                    self.heights.remove(&node_id);
                    self.effect_state.remove(&node_id);
                    self.text_style_state.remove(&node_id);
                    self.text_cache.remove(&node_id);
                }
            }

            for (node_id, parent_node_id, index_inside_parent, element) in mutations
                .added
                .into_iter()
                .sorted_by_key(|(_, parent_node_id, index_inside_parent, _)| {
                    (*parent_node_id, *index_inside_parent)
                })
            {
                let parent_height = *self.heights.entry(parent_node_id).or_default();

                self.parents.insert(node_id, parent_node_id);
                self.heights.insert(node_id, parent_height + 1);

                let parent = self.children.entry(parent_node_id).or_default();

                // TODO: Improve this
                if parent.len() < index_inside_parent as usize + 1 {
                    parent.resize(index_inside_parent as usize + 1, NodeId::PLACEHOLDER);

                    parent[index_inside_parent as usize] = node_id;
                } else if parent.get(index_inside_parent as usize) == Some(&NodeId::PLACEHOLDER) {
                    parent[index_inside_parent as usize] = node_id;
                } else {
                    parent.insert(index_inside_parent as usize, node_id);
                }

                // Add events
                if let Some(events) = element.events_handlers() {
                    for event in events.keys() {
                        self.listeners.entry(*event).or_default().push(node_id);
                    }
                }

                self.elements.insert(node_id, element);
                dirty.push((node_id, DiffModifies::all()));
            }

            for (parent_node_id, movements) in mutations.moved {
                let parent = self.children.get_mut(&parent_node_id).unwrap();
                for (to, node_id) in movements.iter() {
                    let from = parent.iter().position(|id| id == node_id).unwrap();

                    if from < *to as usize {
                        parent.insert(*to as usize, *node_id);
                        parent.remove(from);
                    } else {
                        parent.remove(from);
                        parent.insert(*to as usize, *node_id);
                    }
                }
                let mut diff = DiffModifies::empty();
                diff.insert(DiffModifies::REORDER_LAYOUT);
                diff.insert(DiffModifies::ACCESSIBILITY);
                diff.insert(DiffModifies::STYLE);
                dirty.push((parent_node_id, diff));
            }

            for (node_id, element, flags) in mutations.modified {
                dirty.push((node_id, flags));

                let old_element = self.elements.remove(&node_id).unwrap();

                if flags.contains(DiffModifies::EVENT_HANDLERS) {
                    // Remove old events
                    if let Some(events) = old_element.events_handlers() {
                        for event in events.keys() {
                            self.listeners
                                .entry(*event)
                                .or_default()
                                .retain(|id| *id != node_id);
                        }
                    }

                    // Add new events
                    if let Some(events) = element.events_handlers() {
                        for event in events.keys() {
                            self.listeners.entry(*event).or_default().push(node_id);
                        }
                    }
                }

                self.elements.insert(node_id, element);
            }
        });

        // Run states cascades
        let mut layer_cascades: Vec<NodeId> = Vec::new();
        let mut effects_cascades: Vec<NodeId> = Vec::new();
        let mut text_style_cascades: Vec<NodeId> = Vec::new();

        assert_eq!(dirty.len(), FxHashSet::from_iter(&dirty).len());

        hotpath::measure_block!("dirty run", {
            for (node_id, flags) in dirty {
                let element = self.elements.get(&node_id).unwrap();
                let height_b = self.heights.get(&node_id).unwrap();

                if flags.contains(DiffModifies::REORDER_LAYOUT) {
                    self.layout
                        .invalidate_with_reason(node_id, DirtyReason::Reorder);
                }

                if flags.contains(DiffModifies::INNER_LAYOUT) {
                    self.layout
                        .invalidate_with_reason(node_id, DirtyReason::InnerLayout);
                }

                if flags.contains(DiffModifies::LAYOUT) {
                    self.layout.invalidate(node_id);
                }

                if !needs_render
                    && (flags.intersects(
                        DiffModifies::STYLE
                            | DiffModifies::LAYER
                            | DiffModifies::EFFECT
                            | DiffModifies::TEXT_STYLE,
                    ))
                {
                    needs_render = true;
                }

                if flags.contains(DiffModifies::ACCESSIBILITY) {
                    match self.accessibility_state.get_mut(&node_id) {
                        Some(accessibility_state) => accessibility_state.update(
                            node_id,
                            element,
                            &mut self.accessibility_diff,
                            &mut self.accessibility_groups,
                        ),
                        None => {
                            self.accessibility_state.insert(
                                node_id,
                                AccessibilityState::create(
                                    node_id,
                                    element,
                                    &mut self.accessibility_diff,
                                    &self.accessibility_generator,
                                    &mut self.accessibility_groups,
                                ),
                            );
                        }
                    }
                }

                let handle_cascade = |cascades: &mut Vec<NodeId>| {
                    // Skip scanning if we already know this node is the a root
                    if cascades.iter_mut().any(|root| {
                        let height_a = self.heights.get(root).unwrap();

                        match height_a.cmp(height_b) {
                            std::cmp::Ordering::Less => {
                                self.balance_heights(&node_id, root) == Some(*root)
                            }
                            std::cmp::Ordering::Greater => {
                                let balanced_root = self.balance_heights(root, &node_id);
                                match balanced_root {
                                    Some(r) if r == node_id => {
                                        // If this node is ascendant than the
                                        // current root we set it as the new root
                                        *root = node_id;
                                        true
                                    }
                                    _ => false,
                                }
                            }
                            std::cmp::Ordering::Equal => false,
                        }
                    }) {
                        return;
                    }
                    cascades.push(node_id);
                };

                if flags.intersects(DiffModifies::LAYER) {
                    handle_cascade(&mut layer_cascades);
                }
                if flags.intersects(DiffModifies::EFFECT) {
                    let element = self.elements.get(&node_id).unwrap();
                    if element.effect().is_some() {
                        handle_cascade(&mut effects_cascades);
                    }
                }
                if flags.intersects(DiffModifies::TEXT_STYLE) {
                    handle_cascade(&mut text_style_cascades);
                }
            }
        });

        hotpath::measure_block!("layer cascade", {
            // Run the layer state
            for layer_root in layer_cascades {
                let mut buffer = VecDeque::new();
                buffer.push_front(&layer_root);

                while let Some(node_id) = buffer.pop_front() {
                    let element = self.elements.get(node_id).unwrap();
                    if let Some(parent_node_id) = self.parents.get(node_id) {
                        let entries = self
                            .layer_state
                            .get_disjoint_entries([node_id, parent_node_id], |_id| {
                                LayerState::default()
                            });
                        if let Some([layer_state, parent_layer_state]) = entries {
                            layer_state.update(
                                parent_layer_state,
                                *node_id,
                                element,
                                &mut self.layers,
                            );
                        }
                    } else {
                        assert_eq!(*node_id, NodeId::ROOT);
                        self.layer_state.insert(
                            NodeId::ROOT,
                            LayerState::create_for_root(*node_id, &mut self.layers),
                        );
                    }
                    if let Some(children) = self.children.get(node_id) {
                        buffer.extend(children);
                    }
                }
            }
        });

        hotpath::measure_block!("effect cascade", {
            // Run the effect state
            for effect_root in effects_cascades {
                let mut buffer = VecDeque::new();
                buffer.push_front(&effect_root);

                while let Some(node_id) = buffer.pop_front() {
                    let element = self.elements.get(node_id).unwrap();
                    if let Some(parent_node_id) = self.parents.get(node_id) {
                        let entries = self.effect_state.get_disjoint_two_entries(
                            parent_node_id,
                            node_id,
                            |_id| EffectState::default(),
                            |left, _id| left.clone(),
                        );
                        if let [Some(parent_effect_state), Some(effect_state)] = entries {
                            let effect_data = element.effect();
                            effect_state.update(
                                *parent_node_id,
                                parent_effect_state,
                                *node_id,
                                effect_data,
                            );
                        }
                    } else {
                        assert_eq!(*node_id, NodeId::ROOT);
                    }
                    if let Some(children) = self.children.get(node_id) {
                        buffer.extend(children);
                    }
                }
            }
        });

        hotpath::measure_block!("text style cascade", {
            // Run the text style state
            for text_style_root in text_style_cascades {
                let mut buffer = VecDeque::new();
                buffer.push_front(&text_style_root);

                while let Some(node_id) = buffer.pop_front() {
                    let element = self.elements.get(node_id).unwrap();
                    if let Some(parent_node_id) = self.parents.get(node_id) {
                        let entries = self
                            .text_style_state
                            .get_disjoint_entries([node_id, parent_node_id], |_id| {
                                TextStyleState::default()
                            });
                        if let Some([text_style_state, parent_text_style_state]) = entries {
                            text_style_state.update(
                                *node_id,
                                parent_text_style_state,
                                element,
                                &mut self.layout,
                            );
                        }
                    } else {
                        assert_eq!(*node_id, NodeId::ROOT);
                        self.text_style_state
                            .insert(NodeId::ROOT, TextStyleState::default());
                    }
                    if let Some(children) = self.children.get(node_id) {
                        buffer.extend(children);
                    }
                }
            }

            #[cfg(all(debug_assertions, feature = "debug-integrity"))]
            self.verify_tree_integrity();
        });

        MutationsApplyResult { needs_render }
    }

    #[cfg(debug_assertions)]
    pub fn print_metrics(&self) {
        println!("children: {}", self.children.capacity());
        println!("parents: {}", self.parents.capacity());
        println!("elements: {}", self.elements.capacity());
        println!("heights: {}", self.heights.capacity());
        println!("listeners: {}", self.listeners.capacity());
        println!("layer_state: {}", self.layer_state.capacity());
        println!("layout: {}", self.layout.size());
        println!("layers: {}", self.layers.capacity());
        println!("effect_state: {}", self.effect_state.capacity());
        println!(
            "accessibility_state: {}",
            self.accessibility_state.capacity()
        );
        println!("text_style_state: {}", self.text_style_state.capacity());
        self.text_cache.print_metrics();
    }

    /// Walk to the ancestor of `base` with the same height of `target`
    fn balance_heights(&self, base: &NodeId, target: &NodeId) -> Option<NodeId> {
        let target_height = self.heights.get(target)?;
        let mut current = base;
        loop {
            if self.heights.get(current)? == target_height {
                break;
            }

            let parent_current = self.parents.get(current);
            if let Some(parent_current) = parent_current {
                current = parent_current;
            }
        }
        Some(*current)
    }

    pub fn measure_layout(
        &mut self,
        size: Size2D,
        font_collection: &FontCollection,
        font_manager: &FontMgr,
        events_sender: &UnboundedSender<EventsChunk>,
        scale_factor: f64,
        fallback_fonts: &[Cow<'static, str>],
    ) {
        let mut tree_adapter = TreeAdapterFreya {
            elements: &self.elements,
            parents: &self.parents,
            children: &self.children,
            heights: &self.heights,
            scale_factor,
        };

        let mut events = Vec::new();

        let layout_adapter = LayoutMeasurerAdapter {
            elements: &self.elements,
            text_style_state: &self.text_style_state,
            font_collection,
            font_manager,
            events: &mut events,
            scale_factor,
            fallback_fonts,
            text_cache: &mut self.text_cache,
        };

        self.layout.find_best_root(&mut tree_adapter);
        self.layout.measure(
            NodeId::ROOT,
            Area::from_size(size),
            &mut Some(layout_adapter),
            &mut tree_adapter,
        );
        events_sender
            .unbounded_send(EventsChunk::Batch(events))
            .unwrap();
    }

    pub fn print_ascii(&self, node_id: NodeId, prefix: String, last: bool) {
        let height = self.heights.get(&node_id).unwrap();
        let layer = self.layer_state.get(&node_id).unwrap();

        // Print current node
        println!(
            "{}{}{:?} [{}] ({})",
            prefix,
            if last { "└── " } else { "├── " },
            node_id,
            height,
            layer.layer
        );

        // Get children
        if let Some(children) = self.children.get(&node_id) {
            let len = children.len();
            for (i, child) in children.iter().enumerate() {
                let is_last = i == len - 1;
                // Extend prefix
                let new_prefix = format!("{}{}", prefix, if last { "    " } else { "│   " });
                self.print_ascii(*child, new_prefix, is_last);
            }
        }
    }

    #[cfg(all(debug_assertions, feature = "debug-integrity"))]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn verify_tree_integrity(&self) {
        let mut visited = FxHashSet::default();
        let size = self.elements.len();
        let mut buffer = vec![NodeId::ROOT];
        while let Some(node_id) = buffer.pop() {
            if visited.contains(&node_id) {
                continue;
            }
            visited.insert(node_id);
            if let Some(parent) = self.parents.get(&node_id) {
                buffer.push(*parent);
            }
            if let Some(children) = self.children.get(&node_id) {
                buffer.extend(children);
            }
        }
        assert_eq!(size, visited.len())
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DiffModifies: u32 {
        const LAYOUT = 1;
        const STYLE = 2;
        const ACCESSIBILITY = 3;
        const EVENT_HANDLERS = 4;
        const LAYER = 5;
        const TEXT_STYLE = 6;
        const EFFECT = 7;
        const INNER_LAYOUT = 8;
        const REORDER_LAYOUT = 9;
    }
}

pub struct MutationsApplyResult {
    pub needs_render: bool,
}

pub struct LayoutMeasurerAdapter<'a> {
    pub font_collection: &'a FontCollection,
    pub font_manager: &'a FontMgr,
    elements: &'a FxHashMap<NodeId, Rc<dyn ElementExt>>,
    text_style_state: &'a FxHashMap<NodeId, TextStyleState>,
    events: &'a mut Vec<EmmitableEvent>,
    scale_factor: f64,
    fallback_fonts: &'a [Cow<'static, str>],
    text_cache: &'a mut TextCache,
}

impl LayoutMeasurer<NodeId> for LayoutMeasurerAdapter<'_> {
    fn measure(
        &mut self,
        node_id: NodeId,
        torin_node: &torin::node::Node,
        area_size: &Size2D,
    ) -> Option<(Size2D, Rc<dyn Any>)> {
        self.elements.get(&node_id)?.measure(LayoutContext {
            node_id,
            torin_node,
            area_size,
            font_collection: self.font_collection,
            font_manager: self.font_manager,
            text_style_state: self.text_style_state.get(&node_id).unwrap(),
            scale_factor: self.scale_factor,
            fallback_fonts: self.fallback_fonts,
            text_cache: self.text_cache,
        })
    }

    fn should_hook_measurement(&mut self, node_id: NodeId) -> bool {
        if let Some(element) = self.elements.get(&node_id) {
            element.should_hook_measurement()
        } else {
            false
        }
    }

    fn should_measure_inner_children(&mut self, node_id: NodeId) -> bool {
        if let Some(element) = self.elements.get(&node_id) {
            element.should_measure_inner_children()
        } else {
            false
        }
    }

    fn notify_layout_references(
        &mut self,
        node_id: NodeId,
        area: Area,
        visible_area: Area,
        inner_sizes: Size2D,
    ) {
        let mut data = SizedEventData::new(area, visible_area, inner_sizes);
        data.div(self.scale_factor as f32);
        self.events.push(EmmitableEvent {
            node_id,
            name: EventName::Sized,
            data: EventType::Sized(data),
            bubbles: false,
            source_event: EventName::Sized,
        });
    }
}
