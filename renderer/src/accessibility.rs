use accesskit::{
    Action, DefaultActionVerb, Node, NodeBuilder, NodeClassSet, NodeId as NodeIdKit, Rect, Role,
    Tree, TreeUpdate,
};
use accesskit_winit::Adapter;
use freya_layout::{DioxusDOM, RenderData};
use std::{
    num::NonZeroU128,
    sync::{Arc, Mutex},
};
use tokio::sync::watch;

pub type SharedAccessibilityState = Arc<Mutex<AccessibilityState>>;

const WINDOW_ID: NodeIdKit = NodeIdKit(unsafe { NonZeroU128::new_unchecked(1) });

pub struct AccessibilityState {
    pub nodes: Vec<(NodeIdKit, Node)>,
    pub node_classes: NodeClassSet,
    pub focus: Option<NodeIdKit>,
}

#[derive(PartialEq)]
pub enum FocusDirection {
    Forward,
    Backward,
}

impl AccessibilityState {
    pub fn wrap(self) -> SharedAccessibilityState {
        Arc::new(Mutex::new(self))
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn add_element(
        &mut self,
        dioxus_node: &RenderData,
        accessibility_id: NodeIdKit,
        children: Option<Vec<NodeIdKit>>,
        rdom: &DioxusDOM,
    ) {
        let mut builder = NodeBuilder::new(Role::Unknown);

        if let Some(children) = children {
            builder.set_children(children);
        }

        if let Some(value) = dioxus_node.get_text(rdom) {
            builder.set_value(value);
        }

        if let Some(role) = dioxus_node.get_node(rdom).state.accessibility.role {
            builder.set_role(role);
        }

        builder.set_bounds(Rect {
            x0: dioxus_node.node_area.x as f64,
            x1: (dioxus_node.node_area.x + dioxus_node.node_area.width) as f64,
            y0: dioxus_node.node_area.y as f64,
            y1: (dioxus_node.node_area.y + dioxus_node.node_area.height) as f64,
        });
        builder.add_action(Action::Default);
        builder.set_default_action_verb(DefaultActionVerb::Click);

        let node = builder.build(&mut self.node_classes);
        self.nodes.push((accessibility_id, node));
    }

    pub fn build_root(&mut self) -> Node {
        let mut builder = NodeBuilder::new(Role::Window);
        builder.set_children(
            self.nodes
                .iter()
                .map(|(id, _)| *id)
                .collect::<Vec<NodeIdKit>>(),
        );
        builder.set_name("window");

        builder.build(&mut self.node_classes)
    }

    pub fn process(&mut self) -> TreeUpdate {
        let root = self.build_root();
        let mut nodes = vec![(WINDOW_ID, root)];
        nodes.extend(self.nodes.clone());

        TreeUpdate {
            nodes,
            tree: Some(Tree::new(WINDOW_ID)),
            focus: self.focus,
        }
    }

    pub fn set_focus(
        &mut self,
        adapter: &Adapter,
        id: NodeIdKit,
        focus_sender: &watch::Sender<Option<NodeIdKit>>,
    ) {
        self.focus = Some(id);
        adapter.update(TreeUpdate {
            nodes: Vec::new(),
            tree: None,
            focus: self.focus,
        });

        focus_sender.send(self.focus).ok();
    }

    pub fn set_focus_on_next_node(
        &mut self,
        adapter: &Adapter,
        direction: FocusDirection,
        focus_sender: &watch::Sender<Option<NodeIdKit>>,
    ) {
        if let Some(focused_node_id) = self.focus {
            let current_node = self
                .nodes
                .iter()
                .enumerate()
                .find(|(_, node)| node.0 == focused_node_id);

            if let Some((node_index, _)) = current_node {
                let target_node = if direction == FocusDirection::Forward {
                    self.nodes
                        .iter()
                        .enumerate()
                        .find(|(i, _)| *i == node_index + 1)
                        .map(|(_, node)| node)
                } else {
                    self.nodes
                        .iter()
                        .enumerate()
                        .find(|(i, _)| i + 1 == node_index)
                        .map(|(_, node)| node)
                };

                if let Some((next_node_id, _)) = target_node {
                    self.focus = Some(*next_node_id);
                } else if direction == FocusDirection::Forward {
                    self.focus = self.nodes.first().map(|(id, _)| *id)
                } else if direction == FocusDirection::Backward {
                    self.focus = self.nodes.last().map(|(id, _)| *id)
                }
            } else {
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
