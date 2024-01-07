use accesskit::{
    Action, DefaultActionVerb, Node, NodeBuilder, NodeClassSet, NodeId as AccessibilityId, Rect,
    Role, Tree, TreeUpdate,
};
use dioxus_native_core::{
    prelude::{NodeType, TextNode},
    real_dom::NodeImmutable,
    NodeId,
};
use freya_dom::prelude::{DioxusDOM, DioxusNode};
use freya_node_state::AccessibilityNodeState;
use std::slice::Iter;
use tokio::sync::watch;
use torin::{prelude::NodeAreas, torin::Torin};

use crate::layout::*;

use super::accessibility_state::ACCESSIBILITY_ROOT_ID;

/// Direction for the next Accessibility Node to be focused.
#[derive(PartialEq)]
pub enum AccessibilityFocusDirection {
    Forward,
    Backward,
}

pub trait AccessibilityProvider {
    /// Add a Node to the Accessibility Tree.
    fn add_node(
        &mut self,
        dioxus_node: &DioxusNode,
        node_areas: &NodeAreas,
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
        let node = builder.build(self.node_classes());
        self.push_node(accessibility_id, node);
    }

    /// Push a Node into the Accesibility Tree.
    fn push_node(&mut self, id: AccessibilityId, node: Node);

    /// Mutable reference to the NodeClassSet.
    fn node_classes(&mut self) -> &mut NodeClassSet;

    /// Iterator over the Accessibility Tree of Nodes.
    fn nodes(&self) -> Iter<(AccessibilityId, Node)>;

    /// Get the currently focused Node's ID.
    fn focus_id(&self) -> AccessibilityId;

    /// Update the focused Node ID.
    fn set_focus(&mut self, new_focus_id: AccessibilityId);

    /// Update the focused Node ID and generate a TreeUpdate if necessary.
    fn set_focus_with_update(&mut self, new_focus_id: AccessibilityId) -> Option<TreeUpdate> {
        self.set_focus(new_focus_id);

        // Only focus the element if it exists
        let node_focused_exists = self.nodes().any(|node| node.0 == new_focus_id);
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
    fn build_root(&mut self, root_name: &str) -> Node {
        let mut builder = NodeBuilder::new(Role::Window);
        builder.set_name(root_name.to_string());
        builder.set_children(
            self.nodes()
                .map(|(id, _)| *id)
                .collect::<Vec<AccessibilityId>>(),
        );

        builder.build(self.node_classes())
    }

    /// Process the Nodes accessibility Tree
    fn process(&mut self, root_id: AccessibilityId, root_name: &str) -> TreeUpdate {
        let root = self.build_root(root_name);
        let mut nodes = vec![(root_id, root)];
        nodes.extend(self.nodes().cloned());
        nodes.reverse();

        let focus = self
            .nodes()
            .find_map(|node| {
                if node.0 == self.focus_id() {
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
    fn set_focus_on_next_node(
        &mut self,
        direction: AccessibilityFocusDirection,
        focus_sender: &watch::Sender<AccessibilityId>,
    ) -> Option<TreeUpdate> {
        let current_node = self
            .nodes()
            .enumerate()
            .find(|(_, node)| node.0 == self.focus_id())
            .map(|(i, _)| i);

        if let Some(node_index) = current_node {
            let target_node_index = if direction == AccessibilityFocusDirection::Forward {
                // Find the next Node
                if node_index == self.nodes().len() - 1 {
                    0
                } else {
                    node_index + 1
                }
            } else {
                // Find the previous Node
                if node_index == 0 {
                    self.nodes().len() - 1
                } else {
                    node_index - 1
                }
            };

            let target_node = self
                .nodes()
                .enumerate()
                .find(|(i, _)| *i == target_node_index)
                .map(|(_, node)| node.0)
                .unwrap_or(ACCESSIBILITY_ROOT_ID);

            self.set_focus(target_node);
        } else {
            // Select the first Node
            self.set_focus(
                self.nodes()
                    .next()
                    .map(|(id, _)| *id)
                    .unwrap_or(ACCESSIBILITY_ROOT_ID),
            );
        }

        focus_sender.send(self.focus_id()).ok();

        Some(TreeUpdate {
            nodes: Vec::new(),
            tree: None,
            focus: self.focus_id(),
        })
    }
}

/// Shortcut functions to retrieve Acessibility info from a Dioxus Node
trait NodeAccessibility {
    /// Return the first TextNode from this Node
    fn get_inner_texts(&self) -> Option<String>;

    /// Collect all the AccessibilityIDs from a Node's children
    fn get_accessibility_children(&self) -> Vec<AccessibilityId>;
}

impl NodeAccessibility for DioxusNode<'_> {
    /// Return the first TextNode from this Node
    fn get_inner_texts(&self) -> Option<String> {
        let children = self.children();
        let first_child = children.first()?;
        let node_type = first_child.node_type();
        if let NodeType::Text(TextNode { text, .. }) = &*node_type {
            Some(text.to_owned())
        } else {
            None
        }
    }

    /// Collect all the AccessibilityIDs from a Node's AccessibilityNodeState
    fn get_accessibility_children(&self) -> Vec<AccessibilityId> {
        self.children()
            .iter()
            .filter_map(|child| {
                let node_accessibility = &*child.get::<AccessibilityNodeState>().unwrap();
                node_accessibility.accessibility_id
            })
            .collect::<Vec<AccessibilityId>>()
    }
}

pub fn process_accessibility(
    layers: &Layers,
    layout: &Torin<NodeId>,
    rdom: &DioxusDOM,
    access_provider: &mut impl AccessibilityProvider,
) {
    for layer in layers.layers.values() {
        for node_id in layer {
            let node_areas = layout.get(*node_id).unwrap();
            let dioxus_node = rdom.get(*node_id);
            if let Some(dioxus_node) = dioxus_node {
                let node_accessibility = &*dioxus_node.get::<AccessibilityNodeState>().unwrap();
                if let Some(accessibility_id) = node_accessibility.accessibility_id {
                    access_provider.add_node(
                        &dioxus_node,
                        node_areas,
                        accessibility_id,
                        node_accessibility,
                    );
                }
            }
        }
    }
}
