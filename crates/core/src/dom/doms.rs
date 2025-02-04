use std::sync::{
    Arc,
    Mutex,
    MutexGuard,
};
#[cfg(feature = "rc-dom")]
use std::{
    cell::{
        Ref,
        RefCell,
        RefMut,
    },
    rc::Rc,
};

use dioxus_core::VirtualDom;
use freya_native_core::{
    prelude::{
        DioxusState,
        State,
    },
    real_dom::{
        NodeRef,
        RealDom,
    },
    NodeId,
    SendAnyMap,
};
use torin::prelude::*;

use super::{
    mutations_writer::MutationsWriter,
    CompositorDirtyNodes,
    ImagesCache,
    ParagraphElements,
};
use crate::{
    accessibility::{
        AccessibilityDirtyNodes,
        AccessibilityGenerator,
    },
    custom_attributes::CustomAttributeValues,
    elements::ParagraphElement,
    event_loop_messages::TextGroupMeasurement,
    layers::Layers,
    render::{
        CompositorCache,
        CompositorDirtyArea,
    },
    states::{
        AccessibilityNodeState,
        CursorState,
        FontStyleState,
        LayerState,
        LayoutState,
        ReferencesState,
        StyleState,
        TransformState,
        ViewportState,
    },
};

pub type DioxusDOM = RealDom<CustomAttributeValues>;
pub type DioxusNode<'a> = NodeRef<'a, CustomAttributeValues>;

/// Tiny wrapper over [FreyaDOM] to make it thread-safe if desired.
/// This is primarily used by the Devtools and Testing renderer.
pub struct SafeDOM {
    #[cfg(not(feature = "rc-dom"))]
    pub fdom: FreyaDOM,

    #[cfg(feature = "rc-dom")]
    pub fdom: Rc<RefCell<FreyaDOM>>,
}

#[cfg(feature = "rc-dom")]
impl Clone for SafeDOM {
    fn clone(&self) -> Self {
        Self {
            fdom: self.fdom.clone(),
        }
    }
}

impl SafeDOM {
    #[cfg(not(feature = "rc-dom"))]
    pub fn new(fdom: FreyaDOM) -> Self {
        Self { fdom }
    }

    #[cfg(feature = "rc-dom")]
    pub fn new(fdom: FreyaDOM) -> Self {
        Self {
            fdom: Rc::new(RefCell::new(fdom)),
        }
    }

    /// Get a reference to the DOM.
    #[cfg(not(feature = "rc-dom"))]
    pub fn get(&self) -> &FreyaDOM {
        &self.fdom
    }

    /// Get a reference to the DOM.
    #[cfg(not(feature = "rc-dom"))]
    pub fn try_get(&self) -> Option<&FreyaDOM> {
        Some(&self.fdom)
    }

    /// Get a mutable reference to the DOM.
    #[cfg(not(feature = "rc-dom"))]
    pub fn get_mut(&mut self) -> &mut FreyaDOM {
        &mut self.fdom
    }

    /// Get a reference to the DOM.
    #[cfg(feature = "rc-dom")]
    pub fn get(&self) -> Ref<FreyaDOM> {
        return self.fdom.borrow();
    }

    /// Get a mutable reference to the dom.
    #[cfg(feature = "rc-dom")]
    pub fn get_mut(&self) -> RefMut<FreyaDOM> {
        return self.fdom.borrow_mut();
    }
}

/// Manages the application DOM.
pub struct FreyaDOM {
    rdom: DioxusDOM,
    dioxus_integration_state: DioxusState,
    torin: Arc<Mutex<Torin<NodeId>>>,
    paragraphs: Arc<Mutex<ParagraphElements>>,
    layers: Arc<Mutex<Layers>>,
    compositor_dirty_nodes: Arc<Mutex<CompositorDirtyNodes>>,
    compositor_dirty_area: Arc<Mutex<CompositorDirtyArea>>,
    compositor_cache: Arc<Mutex<CompositorCache>>,
    accessibility_dirty_nodes: Arc<Mutex<AccessibilityDirtyNodes>>,
    accessibility_generator: Arc<AccessibilityGenerator>,
    images_cache: Arc<Mutex<ImagesCache>>,
}

impl Default for FreyaDOM {
    fn default() -> Self {
        let mut rdom = RealDom::<CustomAttributeValues>::new([
            CursorState::to_type_erased(),
            FontStyleState::to_type_erased(),
            ReferencesState::to_type_erased(),
            LayoutState::to_type_erased(),
            StyleState::to_type_erased(),
            TransformState::to_type_erased(),
            AccessibilityNodeState::to_type_erased(),
            ViewportState::to_type_erased(),
            LayerState::to_type_erased(),
        ]);
        let dioxus_integration_state = DioxusState::create(&mut rdom);
        Self {
            rdom,
            dioxus_integration_state,
            torin: Arc::new(Mutex::new(Torin::new())),
            paragraphs: Arc::default(),
            layers: Arc::default(),
            compositor_dirty_nodes: Arc::default(),
            compositor_dirty_area: Arc::default(),
            compositor_cache: Arc::default(),
            accessibility_dirty_nodes: Arc::default(),
            accessibility_generator: Arc::default(),
            images_cache: Arc::default(),
        }
    }
}

impl FreyaDOM {
    pub fn layout(&self) -> MutexGuard<Torin<NodeId>> {
        self.torin.lock().unwrap()
    }

    pub fn layers(&self) -> MutexGuard<Layers> {
        self.layers.lock().unwrap()
    }

    pub fn paragraphs(&self) -> MutexGuard<ParagraphElements> {
        self.paragraphs.lock().unwrap()
    }

    pub fn compositor_dirty_nodes(&self) -> MutexGuard<CompositorDirtyNodes> {
        self.compositor_dirty_nodes.lock().unwrap()
    }

    pub fn compositor_dirty_area(&self) -> MutexGuard<CompositorDirtyArea> {
        self.compositor_dirty_area.lock().unwrap()
    }

    pub fn compositor_cache(&self) -> MutexGuard<CompositorCache> {
        self.compositor_cache.lock().unwrap()
    }

    pub fn accessibility_dirty_nodes(&self) -> MutexGuard<AccessibilityDirtyNodes> {
        self.accessibility_dirty_nodes.lock().unwrap()
    }

    pub fn accessibility_generator(&self) -> &Arc<AccessibilityGenerator> {
        &self.accessibility_generator
    }

    pub fn images_cache(&self) -> MutexGuard<ImagesCache> {
        self.images_cache.lock().unwrap()
    }

    /// Create the initial DOM from the given Mutations
    pub fn init_dom(&mut self, vdom: &mut VirtualDom, scale_factor: f32) {
        // Build the RealDOM
        vdom.rebuild(&mut MutationsWriter {
            native_writer: self
                .dioxus_integration_state
                .create_mutation_writer(&mut self.rdom),
            layout: &mut self.torin.lock().unwrap(),
            layers: &mut self.layers.lock().unwrap(),
            paragraphs: &mut self.paragraphs.lock().unwrap(),
            scale_factor,
            compositor_dirty_nodes: &mut self.compositor_dirty_nodes.lock().unwrap(),
            compositor_dirty_area: &mut self.compositor_dirty_area.lock().unwrap(),
            compositor_cache: &mut self.compositor_cache.lock().unwrap(),
            accessibility_dirty_nodes: &mut self.accessibility_dirty_nodes.lock().unwrap(),
            images_cache: &mut self.images_cache.lock().unwrap(),
        });

        let mut ctx = SendAnyMap::new();
        ctx.insert(self.torin.clone());
        ctx.insert(self.layers.clone());
        ctx.insert(self.paragraphs.clone());
        ctx.insert(self.compositor_dirty_nodes.clone());
        ctx.insert(self.accessibility_dirty_nodes.clone());
        ctx.insert(self.rdom.root_id());
        ctx.insert(self.accessibility_generator.clone());
        ctx.insert(self.images_cache.clone());

        self.rdom.update_state(ctx);
    }

    /// Process the given mutations from the [`VirtualDOM`](dioxus_core::VirtualDom).
    pub fn render_mutations(&mut self, vdom: &mut VirtualDom, scale_factor: f32) -> (bool, bool) {
        // Update the RealDOM
        vdom.render_immediate(&mut MutationsWriter {
            native_writer: self
                .dioxus_integration_state
                .create_mutation_writer(&mut self.rdom),
            layout: &mut self.torin.lock().unwrap(),
            layers: &mut self.layers.lock().unwrap(),
            paragraphs: &mut self.paragraphs.lock().unwrap(),
            scale_factor,
            compositor_dirty_nodes: &mut self.compositor_dirty_nodes.lock().unwrap(),
            compositor_dirty_area: &mut self.compositor_dirty_area.lock().unwrap(),
            compositor_cache: &mut self.compositor_cache.lock().unwrap(),
            accessibility_dirty_nodes: &mut self.accessibility_dirty_nodes.lock().unwrap(),
            images_cache: &mut self.images_cache.lock().unwrap(),
        });

        // Update the Nodes states
        let mut ctx = SendAnyMap::new();
        ctx.insert(self.torin.clone());
        ctx.insert(self.layers.clone());
        ctx.insert(self.paragraphs.clone());
        ctx.insert(self.compositor_dirty_nodes.clone());
        ctx.insert(self.accessibility_dirty_nodes.clone());
        ctx.insert(self.rdom.root_id());
        ctx.insert(self.accessibility_generator.clone());
        ctx.insert(self.images_cache.clone());

        // Update the Node's states
        let diff = self.rdom.update_state(ctx);

        let must_repaint = !diff.is_empty();
        let must_relayout = !self.layout().get_dirty_nodes().is_empty();

        #[cfg(debug_assertions)]
        if !diff.is_empty() {
            tracing::info!(
                "Updated {} nodes in RealDOM, now of size {}",
                diff.len(),
                self.rdom().tree_ref().len()
            );
        }

        (must_repaint, must_relayout)
    }

    /// Get a reference to the [`DioxusDOM`].
    pub fn rdom(&self) -> &DioxusDOM {
        &self.rdom
    }

    /// Get a mutable reference to the [`DioxusDOM`].
    pub fn rdom_mut(&mut self) -> &mut DioxusDOM {
        &mut self.rdom
    }

    pub fn state_mut(&mut self) -> &mut DioxusState {
        &mut self.dioxus_integration_state
    }

    /// Measure all the paragraphs registered under the given TextId
    pub fn measure_paragraphs(&self, text_measurement: TextGroupMeasurement, scale_factor: f64) {
        let paragraphs = self.paragraphs.lock().unwrap();
        let group = paragraphs.get(&text_measurement.text_id);
        let layout = self.layout();
        if let Some(group) = group {
            for node_id in group {
                let node = self.rdom().get(*node_id);
                let layout_node = layout.get(*node_id);

                if let Some((node, layout_node)) = node.zip(layout_node) {
                    ParagraphElement::measure_paragraph(
                        &node,
                        layout_node,
                        &text_measurement,
                        scale_factor,
                    );
                }
            }
        }
    }
}
