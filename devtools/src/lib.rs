use dioxus::prelude::*;
use dioxus_native_core::NodeId;
use dioxus_router::*;
use freya_components::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::use_theme;

use freya_renderer::{HoveredNode, NodeMutation};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedReceiver;

mod node;
mod property;
mod tab;
mod tabs;

use tab::*;
use tabs::{computed::*, style::*, tree::*};

/// Run the [`VirtualDom`](dioxus_core::VirtualDom) with a sidepanel where the devtools are located.
pub fn with_devtools(
    root: fn(cx: Scope) -> Element,
    mutations_receiver: UnboundedReceiver<Vec<NodeMutation>>,
    hovered_node: HoveredNode,
) -> VirtualDom {
    let mutations_receiver = Arc::new(Mutex::new(mutations_receiver));

    VirtualDom::new_with_props(
        AppWithDevtools,
        AppWithDevtoolsProps {
            root,
            mutations_receiver,
            hovered_node,
        },
    )
}

struct AppWithDevtoolsProps {
    root: fn(cx: Scope) -> Element,
    mutations_receiver: Arc<Mutex<UnboundedReceiver<Vec<NodeMutation>>>>,
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
                Root {}
            }
            rect {
                background: "rgb(40, 40, 40)",
                height: "100%",
                width: "350",
                ThemeProvider {
                    DevTools {
                        mutations_receiver: mutations_receiver
                        hovered_node: hovered_node
                    }
                }
            }
        }
    )
}

#[derive(Props)]
pub struct DevToolsProps {
    mutations_receiver: Arc<Mutex<UnboundedReceiver<Vec<NodeMutation>>>>,
    hovered_node: HoveredNode,
}

impl PartialEq for DevToolsProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[allow(non_snake_case)]
pub fn DevTools(cx: Scope<DevToolsProps>) -> Element {
    let children = use_state(cx, Vec::<NodeMutation>::new);
    let theme = use_theme(cx);
    let theme = theme.read();

    use_effect(cx, (), move |_| {
        let mutations_receiver = cx.props.mutations_receiver.clone();
        let children = children.clone();
        async move {
            let mut mutations_receiver = mutations_receiver.lock().unwrap();
            loop {
                if let Some(new_children) = mutations_receiver.recv().await {
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
                        onselected: |node: &NodeMutation| {
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
                        onselected: |node: &NodeMutation| {
                            if let Some(hovered_node) = &cx.props.hovered_node {
                                hovered_node.lock().unwrap().replace(node.id);
                            }
                            selected_node_id.set(Some(node.id));
                        }
                    }
                    if let Some(selected_node) = selected_node.clone() {
                        rsx!(
                            NodeInspectorStyle {
                                node: selected_node
                            }
                        )
                    }
                }
                Route {
                    to: "/elements/computed",
                    NodesTree {
                        nodes: children,
                        height: "calc(50% - 35)",
                        selected_node_id: selected_node_id.get(),
                        onselected: |node: &NodeMutation| {
                            if let Some(hovered_node) = &cx.props.hovered_node {
                                hovered_node.lock().unwrap().replace(node.id);
                            }
                            selected_node_id.set(Some(node.id));
                        }
                    }
                    if let Some(selected_node) = selected_node.clone() {
                        rsx!(
                            NodeInspectorComputed {
                                node: selected_node
                            }
                        )
                    }
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
            TabButton {
                to: "/elements/computed",
                label: "Computed"
            }
        }
    )
}
