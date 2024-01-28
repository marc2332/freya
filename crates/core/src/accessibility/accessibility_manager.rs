use crate::accessibility::*;
use accesskit::{
    Action, DefaultActionVerb, Node, NodeBuilder, NodeClassSet, NodeId as AccessibilityId, Rect,
    Role, Tree, TreeUpdate,
};
use freya_dom::dom::DioxusNode;
use freya_node_state::AccessibilityState;
use std::{
    num::NonZeroU128,
    sync::{Arc, Mutex},
};
use tokio::sync::watch;
use torin::dom_adapter::NodeAreas;

pub type SharedAccessibilityManager = Arc<Mutex<AccessibilityManager>>;

pub const ROOT_ID: AccessibilityId = AccessibilityId(unsafe { NonZeroU128::new_unchecked(1) });

/// Manages the Accessibility integration.
#[derive(Default)]
pub struct AccessibilityManager {
    /// Accessibility Nodes
    pub nodes: Vec<(AccessibilityId, Node)>,
    /// Accessibility tree
    pub node_classes: NodeClassSet,
    /// Current focused Accessibility Node.
    pub focus: Option<AccessibilityId>,
}

impl AccessibilityManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Wrap it in a `Arc<Mutex<T>>`.
    pub fn wrap(self) -> SharedAccessibilityManager {
        Arc::new(Mutex::new(self))
    }

    /// Clear the Accessibility Nodes.
    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn focus_id(&self) -> Option<AccessibilityId> {
        self.focus
    }

    pub fn set_focus(&mut self, new_focus_id: Option<AccessibilityId>) {
        self.focus = new_focus_id;
    }

    pub fn push_node(&mut self, id: AccessibilityId, node: Node) {
        self.nodes.push((id, node))
    }

    /// Add a Node to the Accessibility Tree.
    pub fn add_node(
        &mut self,
        dioxus_node: &DioxusNode,
        node_areas: &NodeAreas,
        accessibility_id: AccessibilityId,
        node_accessibility: &AccessibilityState,
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
        let area = node_areas.area.to_f64();
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
    pub fn set_focus_with_update(
        &mut self,
        new_focus_id: Option<AccessibilityId>,
    ) -> Option<TreeUpdate> {
        self.set_focus(new_focus_id);

        // Only focus the element if it exists
        let node_focused_exists = self.nodes.iter().any(|node| Some(node.0) == new_focus_id);
        if node_focused_exists {
            Some(TreeUpdate {
                nodes: Vec::new(),
                tree: None,
                focus: self.focus_id(),
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

        let focus = self.nodes.iter().find_map(|node| {
            if Some(node.0) == self.focus_id() {
                Some(node.0)
            } else {
                None
            }
        });

        TreeUpdate {
            nodes,
            tree: Some(Tree::new(root_id)),
            focus,
        }
    }

    /// Focus the next/previous Node starting from the currently focused Node.
    pub fn set_focus_on_next_node(
        &mut self,
        direction: AccessibilityFocusDirection,
        focus_sender: &watch::Sender<Option<AccessibilityId>>,
    ) -> Option<TreeUpdate> {
        // Start from the focused node or from the first registered node
        let focused_node_id = self.focus_id().or(self.nodes.first().map(|node| node.0));
        if let Some(focused_node_id) = focused_node_id {
            let current_node = self
                .nodes
                .iter()
                .enumerate()
                .find(|(_, node)| node.0 == focused_node_id)
                .map(|(i, _)| i);

            if let Some(node_index) = current_node {
                let target_node_index = if direction == AccessibilityFocusDirection::Forward {
                    // Find the next Node
                    if node_index == self.nodes.len() - 1 {
                        0
                    } else {
                        node_index + 1
                    }
                } else {
                    // Find the previous Node
                    if node_index == 0 {
                        self.nodes.len() - 1
                    } else {
                        node_index - 1
                    }
                };

                let target_node = self
                    .nodes
                    .iter()
                    .enumerate()
                    .find(|(i, _)| *i == target_node_index)
                    .map(|(_, node)| node.0);

                self.set_focus(target_node);
            } else {
                // Select the first Node
                self.set_focus(self.nodes.first().map(|(id, _)| *id))
            }

            focus_sender.send(self.focus_id()).ok();

            Some(TreeUpdate {
                nodes: Vec::new(),
                tree: None,
                focus: self.focus_id(),
            })
        } else {
            None
        }
    }
}
