use dioxus::prelude::*;
use dioxus_native_core::node::NodeType;
use dioxus_native_core::prelude::{ElementNode, TextNode};
use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::tree::TreeRef;
use dioxus_native_core::NodeId;
use dioxus_router::prelude::*;
use freya_components::*;
use freya_core::node::{get_node_state, NodeState};
use freya_dom::prelude::SafeDOM;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{use_init_accessibility, use_init_focus, use_init_theme, use_theme, DARK_THEME};

use freya_renderer::HoveredNode;
use std::sync::Arc;
use tokio::sync::Notify;
use torin::prelude::NodeAreas;

mod hooks;
mod node;
mod property;
mod tab;
mod tabs;

use tab::*;
use tabs::{layout::*, style::*, tree::*};

/// Run the [`VirtualDom`](dioxus_core::VirtualDom) with a sidepanel where the devtools are located.
pub fn with_devtools(
    rdom: SafeDOM,
    root: fn(cx: Scope) -> Element,
    mutations_notifier: Arc<Notify>,
    hovered_node: HoveredNode,
) -> VirtualDom {
    VirtualDom::new_with_props(
        AppWithDevtools,
        AppWithDevtoolsProps {
            root,
            rdom,
            mutations_notifier,
            hovered_node,
        },
    )
}

struct AppWithDevtoolsProps {
    root: fn(cx: Scope) -> Element,
    rdom: SafeDOM,
    mutations_notifier: Arc<Notify>,
    hovered_node: HoveredNode,
}

#[allow(non_snake_case)]
fn AppWithDevtools(cx: Scope<AppWithDevtoolsProps>) -> Element {
    use_init_focus(cx);
    use_init_accessibility(cx);

    #[allow(non_snake_case)]
    let Root = cx.props.root;
    let mutations_notifier = cx.props.mutations_notifier.clone();
    let hovered_node = cx.props.hovered_node.clone();

    render!(
        rect {
            width: "100%",
            height: "100%",
            direction: "horizontal",
            rect {
                overflow: "clip",
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
                        mutations_notifier: mutations_notifier,
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
    areas: NodeAreas,
}

#[derive(Props)]
pub struct DevToolsProps {
    rdom: SafeDOM,
    mutations_notifier: Arc<Notify>,
    hovered_node: HoveredNode,
}

impl PartialEq for DevToolsProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[allow(non_snake_case)]
pub fn DevTools(cx: Scope<DevToolsProps>) -> Element {
    use_shared_state_provider(cx, Vec::<TreeNode>::new);
    use_shared_state_provider::<HoveredNode>(cx, || cx.props.hovered_node.clone());
    use_init_theme(cx, DARK_THEME);
    let children = use_shared_state::<Vec<TreeNode>>(cx).unwrap();
    let theme = use_theme(cx);
    let theme = theme.read();

    #[allow(clippy::await_holding_lock)]
    use_effect(cx, (), move |_| {
        let rdom = cx.props.rdom.clone();
        let mutations_notifier = cx.props.mutations_notifier.clone();
        let children = children.clone();
        async move {
            loop {
                mutations_notifier.notified().await;

                let dom = rdom.get();
                let rdom = dom.rdom();
                let layout = dom.layout();

                let mut new_children = Vec::new();

                let mut root_found = false;
                let mut devtools_found = false;

                rdom.traverse_depth_first(|node| {
                    let height = rdom.tree_ref().height(node.id()).unwrap();
                    if height == 2 {
                        if !root_found {
                            root_found = true;
                        } else {
                            devtools_found = true;
                        }
                    }

                    if !devtools_found && root_found {
                        let areas = layout.get(node.id());
                        if let Some(areas) = areas {
                            let (text, tag) = match &*node.node_type() {
                                NodeType::Text(TextNode { text, .. }) => {
                                    (Some(text.to_string()), "text".to_string())
                                }
                                NodeType::Element(ElementNode { tag, .. }) => {
                                    (None, tag.to_string())
                                }
                                NodeType::Placeholder => (None, "placeholder".to_string()),
                            };

                            let state = get_node_state(&node);

                            new_children.push(TreeNode {
                                height,
                                id: node.id(),
                                tag,
                                text,
                                state,
                                areas: areas.clone(),
                            });
                        }
                    }
                });
                *children.write() = new_children;
            }
        }
    });

    render!(
        rect {
            width: "100%",
            height: "100%",
            color: theme.body.color,
            Router::<Route> { }
        }
    )
}

#[inline_props]
#[allow(non_snake_case)]
pub fn DevtoolsBar(cx: Scope) -> Element {
    render!(
        TabsBar {
            TabButton {
                to: Route::TreeElementsTab { },
                label: "Elements"
            }
        }
        Outlet::<Route> {}
    )
}

#[allow(non_snake_case)]
#[inline_props]
pub fn NodeInspectorBar(cx: Scope, node_id: NodeId) -> Element {
    render!(
        TabsBar {
            TabButton {
                to: Route::TreeStyleTab { node_id: node_id.serialize() },
                label: "Style"
            }
            TabButton {
                to: Route::TreeLayoutTab { node_id: node_id.serialize() },
                label: "Layout"
            }
        }
    )
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(DevtoolsBar)]
        #[route("/")]
        TreeElementsTab  {},

        #[route("/elements/:node_id/style")]
        TreeStyleTab { node_id: String },

        #[route("/elements/:node_id/layout")]
        TreeLayoutTab { node_id: String },
    #[end_layout]
    #[route("/..route")]
    PageNotFound { },
}

#[allow(non_snake_case)]
#[inline_props]
fn PageNotFound(cx: Scope) -> Element {
    render!(
        label {
            "Page not found."
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn TreeElementsTab(cx: Scope) -> Element {
    let hovered_node = use_shared_state::<HoveredNode>(cx).unwrap();

    render!(NodesTree {
        height: "calc(100% - 35)",
        onselected: |node: &TreeNode| {
            if let Some(hovered_node) = &hovered_node.read().as_ref() {
                hovered_node.lock().unwrap().replace(node.id);
            }
        }
    })
}

#[derive(Props, PartialEq)]
struct TreeTabProps {
    node_id: String,
}

#[allow(non_snake_case)]
fn TreeStyleTab(cx: Scope<TreeTabProps>) -> Element {
    let hovered_node = use_shared_state::<HoveredNode>(cx).unwrap();
    let node_id = NodeId::deserialize(&cx.props.node_id);

    render!(
        NodesTree {
            height: "calc(50% - 35)",
            selected_node_id: node_id,
            onselected: |node: &TreeNode| {
                if let Some(hovered_node) = &hovered_node.read().as_ref() {
                    hovered_node.lock().unwrap().replace(node.id);
                }
            }
        }
        NodeInspectorStyle {
            node_id: node_id
        }
    )
}

#[allow(non_snake_case)]
fn TreeLayoutTab(cx: Scope<TreeTabProps>) -> Element {
    let hovered_node = use_shared_state::<HoveredNode>(cx).unwrap();
    let node_id = NodeId::deserialize(&cx.props.node_id);

    render!(
        NodesTree {
            height: "calc(50% - 35)",
            selected_node_id: node_id,
            onselected: |node: &TreeNode| {
                if let Some(hovered_node) = &hovered_node.read().as_ref() {
                    hovered_node.lock().unwrap().replace(node.id);
                }
            }
        }
        NodeInspectorLayout {
            node_id: node_id
        }
    )
}

pub trait NodeIdSerializer {
    fn serialize(&self) -> String;

    fn deserialize(node_id: &str) -> Self;
}

impl NodeIdSerializer for NodeId {
    fn serialize(&self) -> String {
        format!("{}-{}", self.index(), self.gen())
    }

    fn deserialize(node_id: &str) -> Self {
        let (index, gen) = node_id.split_once('-').unwrap();
        NodeId::new_from_index_and_gen(index.parse().unwrap(), gen.parse().unwrap())
    }
}
