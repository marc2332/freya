use std::sync::{
    Arc,
    Mutex,
};

use accesskit::{
    Action,
    DefaultActionVerb,
    Node,
    NodeBuilder,
    NodeId as AccessibilityId,
    Rect,
    Role,
    Tree,
    TreeUpdate,
};
use freya_common::DirtyAccessibilityTree;
use freya_native_core::{
    prelude::NodeImmutable,
    tags::TagName,
    NodeId,
};
use freya_node_state::AccessibilityNodeState;
use rustc_hash::{
    FxHashMap,
    FxHashSet,
};
use torin::{
    prelude::LayoutNode,
    torin::Torin,
};

use super::{
    AccessibilityFocusStrategy,
    NodeAccessibility,
};
use crate::dom::{
    DioxusDOM,
    DioxusNode,
};

pub const ACCESSIBILITY_ROOT_ID: AccessibilityId = AccessibilityId(0);

pub type SharedAccessibilityTree = Arc<Mutex<AccessibilityTree>>;

pub struct AccessibilityTree {
    pub map: FxHashMap<AccessibilityId, NodeId>,
    // Current focused Accessibility Node.
    pub focused_id: AccessibilityId,
}

impl AccessibilityTree {
    pub fn new(focused_id: AccessibilityId) -> Self {
        Self {
            focused_id,
            map: FxHashMap::default(),
        }
    }

    /// Initialize the Accessibility Tree
    pub fn init(
        &self,
        rdom: &DioxusDOM,
        layout: &Torin<NodeId>,
        dirty: &mut DirtyAccessibilityTree,
    ) -> TreeUpdate {
        dirty.clear();

        let mut nodes = vec![];

        rdom.traverse_depth_first_advanced(|node_ref| {
            if !node_ref.node_type().is_element() {
                return false;
            }

            let accessibility_id = node_ref.get_accessibility_id();
            let layout_node = layout.get(node_ref.id());

            // Layout nodes might not exist yet when the app is lauched
            if let Some((accessibility_id, layout_node)) = accessibility_id.zip(layout_node) {
                let node_accessibility_state = node_ref.get::<AccessibilityNodeState>().unwrap();
                let accessibility_node =
                    Self::create_node(&node_ref, layout_node, &node_accessibility_state);
                nodes.push((accessibility_id, accessibility_node));
            }

            if let Some(tag) = node_ref.node_type().tag() {
                if *tag == TagName::Paragraph || *tag == TagName::Label {
                    return false;
                }
            }

            true
        });

        TreeUpdate {
            nodes,
            tree: Some(Tree::new(ACCESSIBILITY_ROOT_ID)),
            focus: ACCESSIBILITY_ROOT_ID,
        }
    }

    /// Process any pending Accessibility Tree update
    pub fn process_updates(
        &mut self,
        rdom: &DioxusDOM,
        layout: &Torin<NodeId>,
        dirty: &mut DirtyAccessibilityTree,
    ) -> TreeUpdate {
        let removed_ids = dirty.removed.drain().collect::<FxHashMap<_, _>>();
        let mut added_or_updated_ids = dirty.added_or_updated.drain().collect::<FxHashSet<_>>();

        // Mark the ancestors as modified
        for node_id in added_or_updated_ids.clone() {
            let node_ref = rdom.get(node_id).unwrap();
            let node_accessibility_state = node_ref.get::<AccessibilityNodeState>().unwrap();
            added_or_updated_ids.insert(
                node_accessibility_state
                    .closest_accessibility_node_id
                    .unwrap_or(rdom.root_id()),
            );
            self.map
                .insert(node_ref.get_accessibility_id().unwrap(), node_id);
        }

        // Mark the still existing ancenstors as modified
        for (node_id, ancestor_node_id) in removed_ids {
            added_or_updated_ids.insert(ancestor_node_id);
            self.map.retain(|_, id| *id != node_id);
        }

        // Create the updated nodes
        let mut nodes = Vec::new();
        for node_id in added_or_updated_ids {
            let node_ref = rdom.get(node_id).unwrap();
            let layout_node = layout.get(node_id).unwrap();
            let node_accessibility_state = node_ref.get::<AccessibilityNodeState>().unwrap();
            let accessibility_node =
                Self::create_node(&node_ref, layout_node, &node_accessibility_state);

            let accessibility_id = node_ref.get_accessibility_id().unwrap();

            nodes.push((accessibility_id, accessibility_node));
        }

        if !self.map.contains_key(&self.focused_id) {
            self.focused_id = ACCESSIBILITY_ROOT_ID;
        }

        TreeUpdate {
            nodes,
            tree: Some(Tree::new(ACCESSIBILITY_ROOT_ID)),
            focus: self.focused_id,
        }
    }

    /// Update the focused Node ID and generate a TreeUpdate if necessary.
    pub fn set_focus_with_update(
        &mut self,
        new_focus_id: AccessibilityId,
    ) -> Option<(TreeUpdate, NodeId)> {
        self.focused_id = new_focus_id;

        // Only focus the element if it exists
        if let Some(node_id) = self.map.get(&new_focus_id).copied() {
            Some((
                TreeUpdate {
                    nodes: Vec::new(),
                    tree: Some(Tree::new(ACCESSIBILITY_ROOT_ID)),
                    focus: self.focused_id,
                },
                node_id,
            ))
        } else {
            None
        }
    }

    /// Focus a Node given the strategy.
    pub fn set_focus_on_next_node(
        &mut self,
        stragegy: AccessibilityFocusStrategy,
        rdom: &DioxusDOM,
    ) -> (TreeUpdate, NodeId) {
        let mut nodes = Vec::new();

        rdom.traverse_depth_first_advanced(|node_ref| {
            if !node_ref.node_type().is_element() {
                return false;
            }

            let accessibility_id = node_ref.get_accessibility_id();

            if let Some(accessibility_id) = accessibility_id {
                nodes.push((accessibility_id, node_ref.id()))
            }

            if let Some(tag) = node_ref.node_type().tag() {
                if *tag == TagName::Paragraph || *tag == TagName::Label {
                    return false;
                }
            }

            true
        });

        let node_index = nodes
            .iter()
            .enumerate()
            .find(|(_, (accessibility_id, _))| *accessibility_id == self.focused_id)
            .map(|(i, _)| i);

        let target_node = if stragegy == AccessibilityFocusStrategy::Forward {
            // Find the next Node
            if let Some(node_index) = node_index {
                if node_index == nodes.len() - 1 {
                    nodes.first()
                } else {
                    nodes.get(node_index + 1)
                }
            } else {
                nodes.first()
            }
        } else {
            // Find the previous Node
            if let Some(node_index) = node_index {
                if node_index == 0 {
                    nodes.last()
                } else {
                    nodes.get(node_index - 1)
                }
            } else {
                nodes.last()
            }
        };

        let (accessibility_id, node_id) = target_node
            .copied()
            .unwrap_or((ACCESSIBILITY_ROOT_ID, rdom.root_id()));

        self.focused_id = accessibility_id;

        (
            TreeUpdate {
                nodes: Vec::new(),
                tree: Some(Tree::new(ACCESSIBILITY_ROOT_ID)),
                focus: self.focused_id,
            },
            node_id,
        )
    }

    /// Create an accessibility node
    pub fn create_node(
        node_ref: &DioxusNode,
        layout_node: &LayoutNode,
        node_accessibility: &AccessibilityNodeState,
    ) -> Node {
        let mut builder = NodeBuilder::new(Role::Unknown);

        // Set children
        let children = node_ref.get_accessibility_children();
        builder.set_children(children);

        // Set text value
        if let Some(alt) = &node_accessibility.alt {
            builder.set_value(alt.to_owned());
        } else if let Some(value) = node_ref.get_inner_texts() {
            builder.set_value(value);
            builder.set_role(Role::Label);
        }

        // Set name
        if let Some(name) = &node_accessibility.name {
            builder.set_name(name.to_owned());
        }

        // Set role
        if let Some(role) = node_accessibility.role {
            builder.set_role(role);
        }
        // Set root role
        if node_ref.id() == node_ref.real_dom().root_id() {
            builder.set_role(Role::Window);
        }

        // Set the area
        let area = layout_node.area.to_f64();
        builder.set_bounds(Rect {
            x0: area.min_x(),
            x1: area.max_x(),
            y0: area.min_y(),
            y1: area.max_y(),
        });

        // Set focusable action
        if node_accessibility.focusable {
            builder.add_action(Action::Focus);
        } else {
            builder.add_action(Action::Default);
            builder.set_default_action_verb(DefaultActionVerb::Focus);
        }

        builder.build()
    }
}
