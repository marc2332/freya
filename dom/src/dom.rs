use std::sync::{Arc, Mutex};

use dioxus_core::Mutations;
use dioxus_native_core::{
    prelude::{DioxusState, NodeType, State},
    real_dom::{NodeImmutable, NodeRef, RealDom},
    tree::TreeRef,
    NodeId, SendAnyMap,
};
use freya_node_state::{
    CursorSettings, CustomAttributeValues, FontStyle, References, SizeState, Style, Transform,
};
use std::sync::MutexGuard;
use torin::prelude::*;

pub type DioxusDOM = RealDom<CustomAttributeValues>;
pub type DioxusNode<'a> = NodeRef<'a, CustomAttributeValues>;

/// Tiny wrapper over [FreyaDOM] to make it thread-safe if desired.
/// This is primarily used by the Devtools and Testing renderer.
pub struct SafeDOM {
    #[cfg(not(feature = "shared"))]
    pub fdom: FreyaDOM,

    #[cfg(feature = "shared")]
    pub fdom: Arc<Mutex<FreyaDOM>>,
}

#[cfg(feature = "shared")]
impl Clone for SafeDOM {
    fn clone(&self) -> Self {
        Self {
            fdom: self.fdom.clone(),
        }
    }
}

impl SafeDOM {
    #[cfg(not(feature = "shared"))]
    pub fn new(fdom: FreyaDOM) -> Self {
        Self { fdom }
    }

    #[cfg(feature = "shared")]
    pub fn new(fdom: FreyaDOM) -> Self {
        Self {
            fdom: Arc::new(Mutex::new(fdom)),
        }
    }

    /// Get a reference to the DOM.
    #[cfg(not(feature = "shared"))]
    pub fn get(&self) -> &FreyaDOM {
        &self.fdom
    }

    /// Get a mutable reference to the DOM.
    #[cfg(not(feature = "shared"))]
    pub fn get_mut(&mut self) -> &mut FreyaDOM {
        &mut self.fdom
    }

    /// Get a reference to the DOM.
    #[cfg(feature = "shared")]
    pub fn get(&self) -> MutexGuard<FreyaDOM> {
        return self.fdom.lock().unwrap();
    }

    /// Get a mutable reference to the dom.
    #[cfg(feature = "shared")]
    pub fn get_mut(&self) -> MutexGuard<FreyaDOM> {
        return self.fdom.lock().unwrap();
    }
}

/// Manages the application DOM.
pub struct FreyaDOM {
    rdom: DioxusDOM,
    dioxus_integration_state: DioxusState,
    torin: Arc<Mutex<Torin<NodeId>>>,
}

impl Default for FreyaDOM {
    fn default() -> Self {
        let mut rdom = RealDom::<CustomAttributeValues>::new([
            CursorSettings::to_type_erased(),
            FontStyle::to_type_erased(),
            References::to_type_erased(),
            SizeState::to_type_erased(),
            Style::to_type_erased(),
            Transform::to_type_erased(),
        ]);
        let dioxus_integration_state = DioxusState::create(&mut rdom);
        Self {
            rdom,
            dioxus_integration_state,
            torin: Arc::new(Mutex::new(Torin::new())),
        }
    }
}

impl FreyaDOM {
    pub fn new(rdom: DioxusDOM, dioxus_integration_state: DioxusState) -> Self {
        Self {
            rdom,
            dioxus_integration_state,
            torin: Arc::new(Mutex::new(Torin::new())),
        }
    }

    pub fn layout(&self) -> MutexGuard<Torin<NodeId>> {
        self.torin.lock().unwrap()
    }

    /// Create the initial DOM from the given Mutations
    pub fn init_dom(&mut self, mutations: Mutations, scale_factor: f32) {
        self.dioxus_integration_state
            .apply_mutations(&mut self.rdom, mutations);

        let mut ctx = SendAnyMap::new();
        ctx.insert(scale_factor);
        ctx.insert(self.torin.clone());

        self.rdom.update_state(ctx);
    }

    /// Process the given mutations from the [`VirtualDOM`](dioxus_core::VirtualDom).
    pub fn apply_mutations(&mut self, mutations: Mutations, scale_factor: f32) -> (bool, bool) {
        let node_resolver = DioxusNodeResolver::new(self.rdom());
        // Apply the mutations to the layout
        self.layout()
            .apply_mutations(&mutations, &self.dioxus_integration_state, &node_resolver);

        // Apply the mutations the integration state
        self.dioxus_integration_state
            .apply_mutations(&mut self.rdom, mutations);

        // Update the Nodes states
        let mut ctx = SendAnyMap::new();
        ctx.insert(scale_factor);
        ctx.insert(self.torin.clone());

        // Update the Node's states
        let (_, diff) = self.rdom.update_state(ctx);

        let must_repaint = !diff.is_empty();
        let must_relayout = !self.layout().get_dirty_nodes().is_empty();

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
}

fn balance_heights(rdom: &DioxusDOM, base: NodeId, target: NodeId) -> Option<NodeId> {
    let tree = rdom.tree_ref();
    let target_height = tree.height(target)?;
    let mut current = base;
    loop {
        if tree.height(current)? == target_height {
            break;
        }

        let parent_current = tree.parent_id(current);
        if let Some(parent_current) = parent_current {
            current = parent_current;
        }
    }
    Some(current)
}

fn find_common_parent(rdom: &DioxusDOM, node_a: NodeId, node_b: NodeId) -> Option<NodeId> {
    let tree = rdom.tree_ref();
    let height_a = tree.height(node_a)?;
    let height_b = tree.height(node_b)?;

    let (node_a, node_b) = match height_a.cmp(&height_b) {
        std::cmp::Ordering::Less => (
            node_a,
            balance_heights(rdom, node_b, node_a).unwrap_or(node_b),
        ),
        std::cmp::Ordering::Equal => (node_a, node_b),
        std::cmp::Ordering::Greater => (
            balance_heights(rdom, node_a, node_b).unwrap_or(node_a),
            node_b,
        ),
    };

    let mut currents = (node_a, node_b);

    loop {
        // Common parent of node_a and node_b
        if currents.0 == currents.1 {
            return Some(currents.0);
        }

        let parent_a = tree.parent_id(currents.0);
        if let Some(parent_a) = parent_a {
            currents.0 = parent_a;
        } else if rdom.root_id() != currents.0 {
            // Skip unconected nodes
            break;
        }

        let parent_b = tree.parent_id(currents.1);
        if let Some(parent_b) = parent_b {
            currents.1 = parent_b;
        } else if rdom.root_id() != currents.1 {
            // Skip unconected nodes
            break;
        }
    }

    None
}

pub struct DioxusNodeResolver<'a> {
    pub rdom: &'a DioxusDOM,
}

impl<'a> DioxusNodeResolver<'a> {
    pub fn new(rdom: &'a DioxusDOM) -> Self {
        Self { rdom }
    }
}

impl DOMAdapter<NodeId> for DioxusNodeResolver<'_> {
    fn closest_common_parent(&self, node_id_a: &NodeId, node_id_b: &NodeId) -> Option<NodeId> {
        find_common_parent(self.rdom, *node_id_a, *node_id_b)
    }

    fn get_node(&self, node_id: &NodeId) -> Option<Node> {
        let node = self.rdom.get(*node_id)?;
        let mut size = node.get::<SizeState>().unwrap().clone();

        // The root node expands by default
        if *node_id == self.rdom.root_id() {
            size.width = Size::Percentage(Length::new(100.0));
            size.height = Size::Percentage(Length::new(100.0));
        }

        Some(Node {
            width: size.width,
            height: size.height,
            minimum_width: size.minimum_width,
            minimum_height: size.minimum_height,
            maximum_width: size.maximum_width,
            maximum_height: size.maximum_height,
            direction: size.direction,
            padding: size.padding,
            display: size.display,
            scroll_x: Length::new(size.scroll_x),
            scroll_y: Length::new(size.scroll_y),
            has_layout_references: size.node_ref.is_some(),
        })
    }

    fn height(&self, node_id: &NodeId) -> Option<u16> {
        self.rdom.tree_ref().height(*node_id)
    }

    fn parent_of(&self, node_id: &NodeId) -> Option<NodeId> {
        self.rdom.tree_ref().parent_id(*node_id)
    }

    fn children_of(&self, node_id: &NodeId) -> Vec<NodeId> {
        self.rdom.tree_ref().children_ids(*node_id)
    }

    fn is_node_valid(&self, node_id: &NodeId) -> bool {
        let node = self.rdom.get(*node_id);

        if let Some(node) = node {
            let is_placeholder = matches!(*node.node_type(), NodeType::Placeholder);
            let tries_to_be_root = node.parent_id().is_none() && *node_id != self.rdom.root_id();

            !(is_placeholder || tries_to_be_root)
        } else {
            false
        }
    }
}
