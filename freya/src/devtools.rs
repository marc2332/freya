use dioxus::core::ElementId;
use dioxus::prelude::*;
use dioxus_core::Scope;
use dioxus_native_core::tree::TreeView;
use dioxus_native_core::{node::NodeType, real_dom::RealDom};
use dioxus_router::*;
use freya_components::*;
use freya_elements as dioxus_elements;
use freya_hooks::use_theme;
use freya_node_state::{AttributeType, CustomAttributeValues, NodeState, ShadowSettings};
use skia_safe::Color;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

/// Launch a component with the devtools panel enabled.
pub fn with_devtools(
    rdom: Arc<Mutex<RealDom<NodeState, CustomAttributeValues>>>,
    root: fn(cx: Scope) -> Element,
) -> VirtualDom {
    fn app(cx: Scope<DomProps>) -> Element {
        #[allow(non_snake_case)]
        let Root = cx.props.root;

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
                            rdom: cx.props.rdom.clone()
                        }
                    }
                }
            }
        )
    }

    struct DomProps {
        root: fn(cx: Scope) -> Element,
        rdom: Arc<Mutex<RealDom<NodeState, CustomAttributeValues>>>,
    }

    VirtualDom::new_with_props(app, DomProps { root, rdom })
}

#[derive(Clone)]
struct TreeNode {
    tag: String,
    id: ElementId,
    height: u16,
    #[allow(dead_code)]
    text: Option<String>,
    state: NodeState,
}

#[derive(Props)]
pub struct DevToolsProps {
    rdom: Arc<Mutex<RealDom<NodeState, CustomAttributeValues>>>,
}

impl PartialEq for DevToolsProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[allow(non_snake_case)]
pub fn DevTools(cx: Scope<DevToolsProps>) -> Element {
    let children = use_state(cx, Vec::<TreeNode>::new);

    use_effect(cx, (), move |_| {
        let rdom = cx.props.rdom.clone();
        let children = children.clone();
        async move {
            loop {
                // TODO I hate the idea of manually checking every 100ms, it would be better to create a tokio channel to
                // to notify a listener in here if there has been any mutation at all.
                sleep(Duration::from_millis(100)).await;

                let rdom = rdom.lock().unwrap();
                let mut new_children = Vec::new();

                let mut root_found = false;
                let mut devtools_found = false;

                rdom.traverse_depth_first(|n| {
                    let height = rdom.tree.height(n.node_data.node_id).unwrap();
                    if height == 2 {
                        if !root_found {
                            root_found = true;
                        } else {
                            devtools_found = true;
                        }
                    }

                    if !devtools_found {
                        let mut maybe_text = None;
                        let tag = match &n.node_data.node_type {
                            NodeType::Text { text, .. } => {
                                maybe_text = Some(text.clone());
                                "text"
                            }
                            NodeType::Element { tag, .. } => tag,
                            NodeType::Placeholder => "placeholder",
                        }
                        .to_string();

                        if let Some(id) = n.node_data.element_id {
                            new_children.push(TreeNode {
                                height,
                                id,
                                tag,
                                text: maybe_text,
                                // TODO Improve this, I don't like this.
                                state: n.state.clone(),
                            });
                        }
                    }
                });
                children.set(new_children);
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
            initial_url: "freya://freya/elements".to_string(),
            TabsBar {
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
                    selected_node_id: &None,
                    onselected: |node: &TreeNode| {
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
                        selected_node_id.set(Some(node.id));
                    }
                }
                selected_node.and_then(|selected_node| {
                    Some(rsx!(
                        NodeInspectorStyle {
                            node: selected_node
                        }
                    ))
                })
            }
            Route {
                to: "/elements/listeners",
                NodesTree {
                    nodes: children,
                    height: "calc(50% - 35)",
                    selected_node_id: selected_node_id.get(),
                    onselected: |node: &TreeNode| {
                        selected_node_id.set(Some(node.id));
                    }
                }
                selected_node.and_then(|selected_node| {
                    Some(rsx!(
                        NodeInspectorListeners {
                            node: selected_node
                        }
                    ))
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
    selected_node_id: &'a Option<ElementId>,
    onselected: EventHandler<'a, &'a TreeNode>,
) -> Element<'a> {
    let router = use_router(&cx);

    let nodes = nodes.iter().map(|node| {
        rsx! {
            NodeElement {
                key: "{node.id:?}",
                is_selected: Some(node.id) == **selected_node_id,
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
struct TabsBarProps<'a> {
    pub children: Element<'a>,
}

#[allow(non_snake_case)]
fn TabsBar<'a>(cx: Scope<'a, TabsBarProps<'a>>) -> Element<'a> {
    let theme = use_theme(&cx);
    let button_theme = &theme.read().button;
    render!(
        container {
            background: "{button_theme.background}",
            direction: "horizontal",
            height: "35",
            width: "100%",
            color: "{button_theme.font_theme.color}",
            &cx.props.children
        }
    )
}

#[derive(Props)]
struct TabButtonProps<'a> {
    pub to: &'a str,
    pub label: &'a str,
}

#[allow(non_snake_case)]
fn TabButton<'a>(cx: Scope<'a, TabButtonProps<'a>>) -> Element<'a> {
    let theme = use_theme(&cx);
    let button_theme = &theme.read().button;

    let background = use_state(cx, || <&str>::clone(&button_theme.background));
    let set_background = background.setter();

    use_effect(cx, &button_theme.clone(), move |button_theme| async move {
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
            width: "125",
            radius: "7",
            height: "100%",
            color: "{button_theme.font_theme.color}",
            RouterLink {
                to: cx.props.to,
                container {
                    width: "100%",
                    height: "100%",
                    padding: "15",
                    label {
                        align: "center",
                        height: "100%",
                        width: "100%",
                        content
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
fn NodeInspectorBar(cx: Scope) -> Element {
    render!(
        TabsBar {
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
    render!(
        container {
            width: "100%",
            height: "50%",
            NodeInspectorBar { }
            ScrollView {
                show_scrollbar: true,
                height: "calc(100% - 35)",
                width: "100%",
                node.state.iter().enumerate().map(|(i, (name, attr))| {
                    match attr {
                        AttributeType::Measure(measure) => {
                            rsx!{
                                Property {
                                    key: "{i}",
                                    name: "{name}",
                                    value: measure.to_string()
                                }
                            }
                        }
                        AttributeType::Measures((a, b, c, d)) => {
                            rsx!{
                                Property {
                                    key: "{i}",
                                    name: "{name}",
                                    value: format!("({a}, {b}, {c}, {d})")
                                }
                            }
                        }
                        AttributeType::Size(size) => {
                            rsx!{
                                Property {
                                    key: "{i}",
                                    name: "{name}",
                                    value: size.to_string()
                                }
                            }
                        }
                        AttributeType::Color(color) => {
                            rsx!{
                                ColorfulProperty {
                                    key: "{i}",
                                    name: "{name}",
                                    color: color
                                }
                            }
                        }
                        AttributeType::Text(text) => {
                            rsx!{
                                Property {
                                    key: "{i}",
                                    name: "{name}",
                                    value: text.to_string()
                                }
                            }
                        }
                        AttributeType::Direction(direction) => {
                            rsx!{
                                Property {
                                    key: "{i}",
                                    name: "{name}",
                                    value: direction.to_string()
                                }
                            }
                        }
                        AttributeType::Display(display) => {
                            rsx!{
                                Property {
                                    key: "{i}",
                                    name: "{name}",
                                    value: display.to_string()
                                }
                            }
                        }
                        AttributeType::Shadow(shadow_settings) => {
                            rsx!{
                                ShadowProperty {
                                    key: "{i}",
                                    name: "{name}",
                                    shadow_settings: shadow_settings
                                }
                            }
                        }
                    }
                })
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
            paragraph {
                width: "100%",
                text {
                    font_size: "15",
                    color: "rgb(71, 180, 240)",
                    "{name}"
                }
                text {
                    font_size: "15",
                    color: "rgb(215, 215, 215)",
                    ": "
                }
                text {
                    font_size: "15",
                    color: "rgb(252,181,172)",
                    "{value}"
                }
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
                color: "rgb(71, 180, 240)",
                "{name}"
            }
            label {
                font_size: "15",
                color: "rgb(215, 215, 215)",
                ": "
            }
            rect {
                width: "5"
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
            rect {
                width: "5"
            }
            label {
                font_size: "15",
                color: "rgb(252,181,172)",
                "rgb({color.r}, {color.g}, {color.b})"
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn ShadowProperty<'a>(
    cx: Scope<'a>,
    name: &'a str,
    shadow_settings: &'a ShadowSettings,
) -> Element<'a> {
    let color = shadow_settings.color.to_rgb();
    render!(
        container {
            height: "30",
            width: "100%",
            direction: "horizontal",
            padding: "20",
            paragraph {
                text {
                    font_size: "15",
                    color: "rgb(71, 180, 240)",
                    "{name}"
                }
                text {
                    font_size: "15",
                    color: "rgb(215, 215, 215)",
                    ": "
                }
                text {
                    font_size: "15",
                    color: "rgb(252,181,172)",
                    "{shadow_settings.x} {shadow_settings.y} {shadow_settings.intensity} {shadow_settings.size}"
                }
            }
            rect {
                width: "5"
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
            rect {
                width: "5"
            }
            label {
                font_size: "15",
                color: "rgb(252,181,172)",
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
    is_selected: bool,
    onselected: EventHandler<'a, &'a TreeNode>,
) -> Element<'a> {
    let text_color = use_state(cx, || "white");

    let mut margin_left = (node.height * 10) as f32 + 16.5;
    let mut text = format!("{} #{}", node.tag, node.id.0);

    if *is_selected {
        margin_left -= 16.5;
        text = format!("-> {text}");
    };

    render!(
        rect {
            width: "100%",
            height: "25",
            scroll_x: "{margin_left}",
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
                "{text}"
            }
        }
    )
}
