use accesskit::{
    Action, DefaultActionVerb, Node, NodeBuilder, NodeClassSet, NodeId as AccessibilityId, Rect,
    Role, Tree, TreeUpdate,
};
use accesskit_winit::Adapter;
use freya_dom::FreyaDOM;
use freya_layout::RenderData;
use freya_node_state::AccessibilitySettings;
use std::{
    num::NonZeroU128,
    sync::{Arc, Mutex},
};
use tokio::sync::watch;

pub type SharedAccessibilityState = Arc<Mutex<AccessibilityState>>;

const WINDOW_ID: AccessibilityId = AccessibilityId(unsafe { NonZeroU128::new_unchecked(1) });

/// Manages the Accessibility integration.
#[derive(Default)]
pub struct AccessibilityState {
    /// Accessibility Nodes
    pub nodes: Vec<(AccessibilityId, Node)>,

    /// Accessibility tree
    pub node_classes: NodeClassSet,

    /// Current focused Accessibility Node.
    pub focus: Option<AccessibilityId>,
}

/// Direction for the next Accessibility Node to be focused.
#[derive(PartialEq)]
pub enum AccessibilityFocusDirection {
    Forward,
    Backward,
}

impl AccessibilityState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Wrap it in a Arc<Mutex<T>>.
    pub fn wrap(self) -> SharedAccessibilityState {
        Arc::new(Mutex::new(self))
    }

    /// Clear the Accessibility Nodes.
    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    /// Add an Accessibility Node to the Tree.
    pub fn add_element(
        &mut self,
        render_node: &RenderData,
        accessibility_id: AccessibilityId,
        node_accessibility: &AccessibilitySettings,
        dom: &FreyaDOM,
    ) {
        let mut builder = NodeBuilder::new(Role::Unknown);

        // Set children
        let children = render_node.get_accessibility_children(dom);
        if !children.is_empty() {
            builder.set_children(children);
        }

        // Set text value
        if let Some(alt) = &node_accessibility.alt {
            builder.set_value(alt.to_owned());
        } else if let Some(value) = render_node.get_text(dom) {
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
        let area = render_node.node_area.to_f64();
        builder.set_bounds(Rect {
            x0: area.min_x(),
            x1: area.max_x(),
            y0: area.min_y(),
            y1: area.max_y(),
        });

        // Set the action
        builder.add_action(Action::Default);
        builder.set_default_action_verb(DefaultActionVerb::Click);

        // Insert the node into the Tree
        let node = builder.build(&mut self.node_classes);
        self.nodes.push((accessibility_id, node));
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

    /// Get a list of all the nodes
    pub fn get_nodes(&self) -> Vec<(AccessibilityId, Node)> {
        self.nodes
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<(AccessibilityId, Node)>>()
    }

    /// Process the Nodes accessibility Tree
    pub fn process(&mut self, root_name: &str) -> TreeUpdate {
        let root = self.build_root(root_name);
        let mut nodes = vec![(WINDOW_ID, root)];
        nodes.extend(self.get_nodes());

        TreeUpdate {
            nodes,
            tree: Some(Tree::new(WINDOW_ID)),
            focus: self.focus,
        }
    }

    /// Focus a Node given it's `AccessibilityId`
    pub fn set_focus(&mut self, adapter: &Adapter, id: AccessibilityId) {
        self.focus = Some(id);
        adapter.update(TreeUpdate {
            nodes: Vec::new(),
            tree: None,
            focus: self.focus,
        });
    }

    /// Focus the next/previous Node starting from the currently focused Node.
    pub fn set_focus_on_next_node(
        &mut self,
        adapter: &Adapter,
        direction: AccessibilityFocusDirection,
        focus_sender: &watch::Sender<Option<AccessibilityId>>,
    ) {
        if let Some(focused_node_id) = self.focus {
            let current_node = self
                .nodes
                .iter()
                .enumerate()
                .find(|(_, node)| node.0 == focused_node_id);

            if let Some((node_index, _)) = current_node {
                let target_node = if direction == AccessibilityFocusDirection::Forward {
                    // Find the next Node
                    self.nodes
                        .iter()
                        .enumerate()
                        .find(|(i, _)| i + 1 == node_index)
                        .map(|(_, node)| node)
                } else {
                    // Find the previous Node
                    self.nodes
                        .iter()
                        .enumerate()
                        .find(|(i, _)| *i == node_index + 1)
                        .map(|(_, node)| node)
                };

                if let Some((next_node_id, _)) = target_node {
                    self.focus = Some(*next_node_id);
                } else if direction == AccessibilityFocusDirection::Forward {
                    // Select the last Node
                    self.focus = self.nodes.last().map(|(id, _)| *id)
                } else if direction == AccessibilityFocusDirection::Backward {
                    // Select the first Node
                    self.focus = self.nodes.first().map(|(id, _)| *id)
                }
            } else {
                // Select the first Node
                self.focus = self.nodes.first().map(|(id, _)| *id)
            }

            adapter.update(TreeUpdate {
                nodes: Vec::new(),
                tree: None,
                focus: self.focus,
            });

            focus_sender.send(self.focus).ok();
        }
    }
}
