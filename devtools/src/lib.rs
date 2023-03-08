use dioxus::prelude::*;
use dioxus_native_core::node::NodeType;
use dioxus_native_core::tree::TreeView;
use dioxus_native_core::NodeId;
use dioxus_router::*;
use freya_components::*;
use freya_core::SharedRealDOM;
use freya_elements as dioxus_elements;
use freya_hooks::use_theme;
use freya_node_state::NodeState;
use freya_renderer::HoveredNode;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::time::sleep;

mod node;
mod property;
mod tab;
mod tabs;

use tab::*;
use tabs::{style::*, tree::*};

/// Run the [VirtualDom] with a sidepanel where the devtools are located.
pub fn with_devtools(
    rdom: SharedRealDOM,
    root: fn(cx: Scope) -> Element,
    mutations_receiver: UnboundedReceiver<()>,
    hovered_node: HoveredNode,
) -> VirtualDom {
    let mutations_receiver = Arc::new(Mutex::new(mutations_receiver));

    VirtualDom::new_with_props(
        AppWithDevtools,
        AppWithDevtoolsProps {
            root,
            rdom,
            mutations_receiver,
            hovered_node,
        },
    )
}

struct AppWithDevtoolsProps {
    root: fn(cx: Scope) -> Element,
    rdom: SharedRealDOM,
    mutations_receiver: Arc<Mutex<UnboundedReceiver<()>>>,
    hovered_node: HoveredNode,
}

#[allow(non_snake_case)]
fn AppWithDevtools(cx: Scope<AppWithDevtoolsProps>) -> Element {
    #[allow(non_snake_case)]
    let Root = cx.props.root;
    let mutations_receiver = cx.props.mutations_receiver.clone();
    let hovered_node = cx.props.hovered_node.clone();

    render!(
        rect {
            width: "100%",
            height: "100%",
            direction: "horizontal",
            container {
                height: "100%",
                width: "calc(100% - 350)",
                Root { },
            }
            rect {
                background: "rgb(40, 40, 40)",
                height: "100%",
                width: "350",
                ThemeProvider {
                    DevTools {
                        rdom: cx.props.rdom.clone(),
                        mutations_receiver: mutations_receiver
                        hovered_node: hovered_node
                    }
                }
            }
        }
    )
}

#[derive(Clone)]
pub struct TreeNode {
    tag: String,
    id: NodeId,
    height: u16,
    #[allow(dead_code)]
    text: Option<String>,
    state: NodeState,
}

#[derive(Props)]
pub struct DevToolsProps {
    rdom: SharedRealDOM,
    mutations_receiver: Arc<Mutex<UnboundedReceiver<()>>>,
    hovered_node: HoveredNode,
}

impl PartialEq for DevToolsProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[allow(non_snake_case)]
pub fn DevTools(cx: Scope<DevToolsProps>) -> Element {
    let children = use_state(cx, Vec::<TreeNode>::new);
    let theme = use_theme(cx);
    let theme = theme.read();

    #[allow(clippy::await_holding_lock)]
    use_effect(cx, (), move |_| {
        let rdom = cx.props.rdom.clone();
        let mutations_receiver = cx.props.mutations_receiver.clone();
        let children = children.clone();
        async move {
            
            let mut mutations_receiver = mutations_receiver.lock().unwrap();
            loop {
                if mutations_receiver.recv().await.is_some() {
                    sleep(Duration::from_millis(10)).await;

                    let rdom = rdom.lock().unwrap();
                    let mut new_children = Vec::new();

                    let mut root_found = false;
                    let mut devtools_found = false;

                    rdom.traverse_depth_first(|node| {
                        let height = rdom.tree.height(node.node_data.node_id).unwrap();
                        if height == 2 {
                            if !root_found {
                                root_found = true;
                            } else {
                                devtools_found = true;
                            }
                        }

                        if !devtools_found {
                            let mut maybe_text = None;
                            let tag = match &node.node_data.node_type {
                                NodeType::Text { text, .. } => {
                                    maybe_text = Some(text.clone());
                                    "text"
                                }
                                NodeType::Element { tag, .. } => tag,
                                NodeType::Placeholder => "placeholder",
                            }
                            .to_string();

                            new_children.push(TreeNode {
                                height,
                                id: node.node_data.node_id,
                                tag,
                                text: maybe_text,
                                state: node.state.clone(),
                            });
                        }
                    });
                    children.set(new_children);
                }
            }
        }
    });

    let selected_node_id = use_state::<Option<NodeId>>(cx, || None);

    let selected_node = children.iter().find(|c| {
        if let Some(n_id) = selected_node_id.get() {
            n_id == &c.id
        } else {
            false
        }
    });

    render!(
        rect {
            width: "100%",
            height: "100%",
            color: theme.body.color,
            Router {
                initial_url: "freya://freya/elements".to_string(),
                DevtoolsBar {}
                Route {
                    to: "/elements",
                    NodesTree {
                        nodes: children,
                        height: "calc(100% - 35)",
                        selected_node_id: &None,
                        onselected: |node: &TreeNode| {
                            if let Some(hovered_node) = &cx.props.hovered_node {
                                hovered_node.lock().unwrap().replace(node.id);
                            }
                            selected_node_id.set(Some(node.id));
                        }
                    }
                }
                Route {
                    to: "/elements/style",
                    NodesTree {
                        nodes: children,
                        height: "calc(50% - 35)",
                        selected_node_id: selected_node_id.get(),
                        onselected: |node: &TreeNode| {
                            if let Some(hovered_node) = &cx.props.hovered_node {
                                hovered_node.lock().unwrap().replace(node.id);
                            }
                            selected_node_id.set(Some(node.id));
                        }
                    }
                    selected_node.map(|selected_node| {
                        rsx!(
                            NodeInspectorStyle {
                                node: selected_node
                            }
                        )
                    })
                }
            }
        }
    )
}

#[allow(non_snake_case)]
pub fn DevtoolsBar(cx: Scope) -> Element {
    render!(
        TabsBar {
            TabButton {
                to: "/elements",
                label: "Elements"
            }
        }
    )
}

#[allow(non_snake_case)]
pub fn NodeInspectorBar(cx: Scope) -> Element {
    render!(
        TabsBar {
            TabButton {
                to: "/elements/style",
                label: "Style"
            }
        }
    )
}
