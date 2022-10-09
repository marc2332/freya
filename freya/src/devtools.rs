use dioxus::core::ElementId;
use dioxus::prelude::*;
use dioxus_core::Scope;
use dioxus_native_core::real_dom::{NodeType, RealDom};
use dioxus_router::*;
use fermi::use_atom_ref;
use freya_components::*;
use freya_elements as dioxus_elements;
use freya_node_state::node::NodeState;
use skia_safe::Color;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::sleep;

#[derive(Clone)]
struct TreeNode {
    tag: String,
    id: ElementId,
    height: u16,
    text: Option<String>,
    state: NodeState,
}

#[derive(Props)]
pub struct DevToolsProps {
    rdom: Arc<Mutex<RealDom<NodeState>>>,
}

// Hacky stuff over here
impl PartialEq for DevToolsProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[allow(non_snake_case)]
pub fn DevTools(cx: Scope<DevToolsProps>) -> Element {
    let children = use_state(&cx, || Vec::<TreeNode>::new());
    let setter = children.setter();

    use_effect(&cx, (), move |_| {
        let rdom = cx.props.rdom.clone();
        async move {
            loop {
                sleep(Duration::from_millis(25)).await;

                let rdom = rdom.lock().unwrap();
                let mut children = Vec::new();

                let mut root_found = false;
                let mut devtools_found = false;

                rdom.traverse_depth_first(|n| {
                    if n.height == 2 {
                        if root_found == false {
                            root_found = true;
                        } else {
                            devtools_found = true;
                        }
                    }

                    if !devtools_found {
                        let mut maybe_text = None;
                        let tag = match &n.node_type {
                            NodeType::Text { text, .. } => {
                                maybe_text = Some(text.clone());
                                "text"
                            }
                            NodeType::Element { tag, .. } => tag,
                            NodeType::Placeholder => "placeholder",
                        }
                        .to_string();

                        children.push(TreeNode {
                            height: n.height,
                            id: n.id,
                            tag,
                            text: maybe_text,
                            state: n.state.clone(),
                        });
                    }
                });
                setter(children);
            }
        }
    });

    let selected_node_id = use_state::<Option<ElementId>>(&cx, || None);

    let selected_node = children.iter().find(|c| {
        if let Some(n_id) = selected_node_id.get() {
            n_id == &c.id
        } else {
            false
        }
    });

    render!(
        Router {
            initial_url: "bla://bla/elements".to_string(),
            container {
                width: "100%",
                direction: "horizontal",
                height: "35",
                TabButton {
                    to: "/elements",
                    label: "Elements"
                }
                TabButton {
                    to: "/settings",
                    label: "Settings"
                }
            }
            Route {
                to: "/elements",
                NodesTree {
                    nodes: children,
                    height: "calc(100% - 35)",
                    onselected: |node: &TreeNode| {
                        selected_node_id.set(Some(node.id));
                    }
                }
            }
            Route {
                to: "/elements/style",
                NodesTree {
                    nodes: children,
                    height: "calc(60% - 35)",
                    onselected: |node: &TreeNode| {
                        selected_node_id.set(Some(node.id));
                    }
                }
                selected_node.and_then(|selected_node| {
                    render!(
                        NodeInspectorStyle {
                            node: selected_node
                        }
                    )
                })
            }
            Route {
                to: "/elements/listeners",
                NodesTree {
                    nodes: children,
                    height: "calc(60% - 35)",
                    onselected: |node: &TreeNode| {
                        selected_node_id.set(Some(node.id));
                    }
                }
                selected_node.and_then(|selected_node| {
                    render!(
                        NodeInspectorListeners {
                            node: selected_node
                        }
                    )
                })
            }
            Route {
                to: "/settings",
                label {
                    "Settings would be here."
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn NodesTree<'a>(
    cx: Scope<'a>,
    nodes: &'a Vec<TreeNode>,
    height: &'a str,
    onselected: EventHandler<'a, &'a TreeNode>,
) -> Element<'a> {
    let router = use_router(&cx);

    let nodes = nodes.iter().map(|node| {
        rsx! {
            NodeElement {
                key: "{node.id}",
                onselected: |node: &TreeNode| {
                    onselected.call(node);
                    router.push_route("/elements/style", None, None)
                }
                node: node
            }
        }
    });

    render!(ScrollView {
        width: "100%",
        height: "{height}",
        padding: "30",
        show_scrollbar: true,
        nodes
    })
}

#[derive(Props)]
struct TabButtonProps<'a> {
    pub to: &'a str,
    pub label: &'a str,
}

#[allow(non_snake_case)]
fn TabButton<'a>(cx: Scope<'a, TabButtonProps<'a>>) -> Element<'a> {
    let theme = use_atom_ref(&cx, THEME);
    let button_theme = &theme.read().button;

    let background = use_state(&cx, || button_theme.background.clone());
    let set_background = background.setter();

    use_effect(&cx, &button_theme.clone(), move |button_theme| async move {
        set_background(button_theme.background);
    });

    let content = cx.props.label;
    render!(
        container {
            background: "{background}",
            onmouseover: move |_| {
                    background.set(theme.read().button.hover_background);
            },
            onmouseleave: move |_| {
                background.set(theme.read().button.background);
            },
            width: "150",
            height: "100%",
            color: "{button_theme.font_theme.color}",
            RouterLink {
                to: cx.props.to,
                container {
                    width: "100%",
                    height: "100%",
                    padding: "15",
                    label {
                        height: "100%",
                        content
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
fn NodeInspectorBar<'a>(cx: Scope<'a>) -> Element<'a> {
    render!(
        container {
            width: "100%",
            direction: "horizontal",
            height: "35",
            TabButton {
                to: "/elements/style",
                label: "Style"
            }
            TabButton {
                to: "/elements/listeners",
                label: "Event Listeners"
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn NodeInspectorStyle<'a>(cx: Scope<'a>, node: &'a TreeNode) -> Element<'a> {
    let background = &node.state.style.background;
    let color = &node.state.font_style.color;
    let height = node.state.size.height.to_string();
    let width = node.state.size.width.to_string();
    render!(
        container {
            width: "100%",
            height: "40%",
            NodeInspectorBar { }
            ScrollView {
                show_scrollbar: true,
                height: "calc(100% - 35)",
                ColorfulProperty {
                    name: "Background",
                    color: background
                }
                ColorfulProperty {
                    name: "Color",
                    color: color
                }
                Property {
                    name: "Width",
                    value: width
                }
                Property {
                    name: "Height",
                    value: height
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn Property<'a>(cx: Scope<'a>, name: &'a str, value: String) -> Element<'a> {
    render!(
        container {
            height: "30",
            width: "100%",
            direction: "horizontal",
            padding: "20",
            label {
                font_size: "15",
                width: "90",
                "{name}: {value}"
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn ColorfulProperty<'a>(cx: Scope<'a>, name: &'a str, color: &'a Color) -> Element<'a> {
    let color = color.to_rgb();
    render!(
        container {
            height: "30",
            width: "100%",
            direction: "horizontal",
            padding: "20",
            label {
                font_size: "15",
                width: "90",
                "{name}: "
            }
            rect {
                width: "17",
                height: "17",
                radius: "5",
                background: "white",
                padding: "5",
                rect {
                    radius: "3",
                    width: "100%",
                    height: "100%",
                    background: "rgb({color.r}, {color.g}, {color.b})",
                }
            }
            rect { // hacky spacer
                width: "5"
            }
            label {
                font_size: "15",
                "rgb({color.r}, {color.g}, {color.b})"
            }
        }
    )
}

#[allow(unused_variables)]
#[allow(non_snake_case)]
#[inline_props]
fn NodeInspectorListeners<'a>(cx: Scope<'a>, node: &'a TreeNode) -> Element<'a> {
    render!(
        container {
            width: "100%",
            height: "40%",
            NodeInspectorBar { }
            container {
                height: "calc(100% - 35)",
                width: "100%",
                direction: "horizontal",
                padding: "30",
                label {
                    "Listeners would be here."
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn NodeElement<'a>(
    cx: Scope<'a>,
    node: &'a TreeNode,
    onselected: EventHandler<'a, &'a TreeNode>,
) -> Element<'a> {
    let text = node
        .text
        .as_ref()
        .map(|v| format!("({v})"))
        .unwrap_or_default();

    let text_color = use_state(&cx, || "white");

    render!(
        rect {
            width: "100%",
            height: "25",
            scroll_x: "{node.height * 10}",
            onclick: |_| onselected.call(node),
            onmouseover: move |_| {
                text_color.set("rgb(150, 150, 150)");
            },
            onmouseleave: move |_| {
                text_color.set("white");
            },
            label {
                font_size: "14",
                color: "{text_color}",
                "{node.tag} #{node.id} {text}"
            }
        }
    )
}
