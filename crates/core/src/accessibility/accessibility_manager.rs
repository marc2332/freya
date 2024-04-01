use crate::{accessibility::*, dom::DioxusNode};
use accesskit::{
    Action, DefaultActionVerb, Node, NodeBuilder, NodeClassSet, Rect, Role, Tree, TreeUpdate,
};

use freya_node_state::AccessibilityNodeState;
use std::sync::{Arc, Mutex};
use torin::prelude::LayoutNode;

pub type SharedAccessibilityManager = Arc<Mutex<AccessibilityManager>>;

pub const ACCESSIBILITY_ROOT_ID: AccessibilityId = AccessibilityId(0);

/// Manages the Accessibility integration.
pub struct AccessibilityManager {
    /// Accessibility Nodes
    pub nodes: Vec<(AccessibilityId, Node)>,
    /// Accessibility tree
    pub node_classes: NodeClassSet,
    /// Current focused Accessibility Node.
    pub focused_id: AccessibilityId,
}

impl AccessibilityManager {
    pub fn new(focused_id: AccessibilityId) -> Self {
        Self {
            focused_id,
            node_classes: NodeClassSet::default(),
            nodes: Vec::default(),
        }
    }

    /// Wrap it in a `Arc<Mutex<T>>`.
    pub fn wrap(self) -> SharedAccessibilityManager {
        Arc::new(Mutex::new(self))
    }

    /// Clear the Accessibility Nodes.
    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn push_node(&mut self, id: AccessibilityId, node: Node) {
        self.nodes.push((id, node))
    }

    /// Add a Node to the Accessibility Tree.
    pub fn add_node(
        &mut self,
        dioxus_node: &DioxusNode,
        layout_node: &LayoutNode,
        accessibility_id: AccessibilityId,
        node_accessibility: &AccessibilityNodeState,
    ) {
        let mut builder = NodeBuilder::new(Role::Unknown);

        // Set children
        let children = dioxus_node.get_accessibility_children();
        if !children.is_empty() {
            builder.set_children(children);
        }

        // Set text value
        if let Some(alt) = &node_accessibility.alt {
            builder.set_value(alt.to_owned());
        } else if let Some(value) = dioxus_node.get_inner_texts() {
            builder.set_value(value);
        }

        // Set name
        if let Some(name) = &node_accessibility.name {
            builder.set_name(name.to_owned());
        }

        // Set role
        if let Some(role) = node_accessibility.role {
            builder.set_role(role);
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

        // Insert the node into the Tree
        let node = builder.build(&mut self.node_classes);
        self.push_node(accessibility_id, node);
    }

    /// Update the focused Node ID and generate a TreeUpdate if necessary.
    pub fn set_focus_with_update(&mut self, new_focus_id: AccessibilityId) -> Option<TreeUpdate> {
        self.focused_id = new_focus_id;

        // Only focus the element if it exists
        let node_focused_exists = self.nodes.iter().any(|node| node.0 == new_focus_id);
        if node_focused_exists {
            Some(TreeUpdate {
                nodes: Vec::new(),
                tree: None,
                focus: self.focused_id,
            })
        } else {
            None
        }
    }

    /// Create the root Accessibility Node.
    pub fn build_root(&mut self, root_name: &str) -> Node {
        let mut builder = NodeBuilder::new(Role::Window);
        builder.set_name(root_name.to_string());
        builder.set_children(
            self.nodes
                .iter()
                .map(|(id, _)| *id)
                .collect::<Vec<AccessibilityId>>(),
        );

        builder.build(&mut self.node_classes)
    }

    /// Process the Nodes accessibility Tree
    pub fn process(&mut self, root_id: AccessibilityId, root_name: &str) -> TreeUpdate {
        let root = self.build_root(root_name);
        let mut nodes = vec![(root_id, root)];
        nodes.extend(self.nodes.clone());
        nodes.reverse();

        let focus = self
            .nodes
            .iter()
            .find_map(|node| {
                if node.0 == self.focused_id {
                    Some(node.0)
                } else {
                    None
                }
            })
            .unwrap_or(ACCESSIBILITY_ROOT_ID);

        TreeUpdate {
            nodes,
            tree: Some(Tree::new(root_id)),
            focus,
        }
    }

    /// Focus the next/previous Node starting from the currently focused Node.
    pub fn set_focus_on_next_node(&mut self, direction: AccessibilityFocusDirection) -> TreeUpdate {
        let node_index = self
            .nodes
            .iter()
            .enumerate()
            .find(|(_, node)| node.0 == self.focused_id)
            .map(|(i, _)| i);

        let target_node = if direction == AccessibilityFocusDirection::Forward {
            // Find the next Node
            if let Some(node_index) = node_index {
                if node_index == self.nodes.len() - 1 {
                    self.nodes.first()
                } else {
                    self.nodes.get(node_index + 1)
                }
            } else {
                self.nodes.first()
            }
        } else {
            // Find the previous Node
            if let Some(node_index) = node_index {
                if node_index == 0 {
                    self.nodes.last()
                } else {
                    self.nodes.get(node_index - 1)
                }
            } else {
                self.nodes.last()
            }
        };

        self.focused_id = target_node
            .map(|(id, _)| *id)
            .unwrap_or(ACCESSIBILITY_ROOT_ID);

        TreeUpdate {
            nodes: Vec::new(),
            tree: None,
            focus: self.focused_id,
        }
    }
}
