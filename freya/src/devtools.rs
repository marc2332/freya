use dioxus::prelude::*;
use dioxus::{core::ElementId, events::MouseData};
use dioxus_core::{Scope, UiEvent};
use dioxus_native_core::real_dom::{NodeType, RealDom};
use dioxus_router::*;
use fermi::use_atom_ref;
use freya_components::*;
use freya_elements as dioxus_elements;
use freya_node_state::node::NodeState;
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

    let children = children.get().iter().map(|node| {
        rsx! {
            NodeElement {
                node: node
            }
        }
    });

    cx.render(rsx! {
        Router {
             container {
                width: "100%",
                direction: "horizontal",
                height: "35",
                TabButton {
                    to: "/",
                    label: "Elements"
                }
                TabButton {
                    to: "/settings",
                    label: "Settings"
                }
             }
            Route {
                to: "/",
                ScrollView {
                    width: "100%",
                    height: "calc(100% - 50)",
                    padding: "30",
                    show_scrollbar: true,
                    children
                }
            }
            Route {
                to: "/settings",
                label {
                    "Settings would be here."
                }
            }
        }
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
            width: "100",
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
#[inline_props]
fn NodePopup<'a>(cx: Scope<'a>, node: &'a TreeNode) -> Element<'a> {
    let background = node.state.style.background.to_rgb();
    render!(
        rect {
            width: "0",
            height: "0",
            layer: "-10",
            rect {
                width: "200",
                height: "auto",
                radius: "10",
                background: "rgb(70, 70, 70)",
                shadow: "0 0 100 15 black",
                padding: "25",
                container {
                    height: "30",
                    width: "100%",
                    direction: "horizontal",
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
                            background: "rgb({background.r}, {background.g}, {background.b})",
                        }
                    }
                    label {
                        "rgb({background.r}, {background.g}, {background.b})"
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn NodeElement<'a>(cx: Scope<'a>, node: &'a TreeNode) -> Element<'a> {
    let text = node
        .text
        .as_ref()
        .map(|v| format!("({v})"))
        .unwrap_or_default();
    let show_popup = use_state(&cx, || false);

    let mouseover = |_: UiEvent<MouseData>| {
        show_popup.set(true);
    };

    let mouseleave = |_: UiEvent<MouseData>| {
        show_popup.set(false);
    };

    render!(
        rect {
            width: "100%",
            height: "25",
            scroll_x: "{node.height * 10}",
            onmouseover: mouseover,
            onmouseleave: mouseleave,
            label {
                "{node.tag} #{node.id} {text}"
            }
            show_popup.get().then(|| {
                cx.render(rsx! {
                    NodePopup {
                        node: node
                    }
                })
            })
        }
    )
}
