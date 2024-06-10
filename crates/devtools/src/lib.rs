use std::collections::HashSet;

use dioxus::prelude::*;
use dioxus_radio::prelude::*;
use dioxus_router::prelude::{use_route, Outlet, Routable, Router};
use freya_components::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{use_init_theme, use_platform, DARK_THEME};
use freya_native_core::NodeId;

use freya_renderer::{devtools::DevtoolsReceiver, HoveredNode};
use state::{DevtoolsChannel, DevtoolsState};

mod hooks;
mod node;
mod property;
mod state;
mod tabs;

use tabs::{layout::*, style::*, tree::*};

/// Run the [`VirtualDom`] with a sidepanel where the devtools are located.
pub fn with_devtools(
    root: fn() -> Element,
    devtools_receiver: DevtoolsReceiver,
    hovered_node: HoveredNode,
) -> VirtualDom {
    VirtualDom::new_with_props(
        AppWithDevtools,
        AppWithDevtoolsProps {
            root,
            devtools_receiver,
            hovered_node,
        },
    )
}

#[derive(Props, Clone)]
struct AppWithDevtoolsProps {
    root: fn() -> Element,
    devtools_receiver: DevtoolsReceiver,
    hovered_node: HoveredNode,
}

impl PartialEq for AppWithDevtoolsProps {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[allow(non_snake_case)]
fn AppWithDevtools(props: AppWithDevtoolsProps) -> Element {
    #[allow(non_snake_case)]
    let Root = props.root;
    let devtools_receiver = props.devtools_receiver;
    let hovered_node = props.hovered_node;

    rsx!(
        NativeContainer {
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
                            devtools_receiver,
                            hovered_node
                        }
                    }
                }
            }
        }
    )
}

#[derive(Props, Clone)]
pub struct DevToolsProps {
    devtools_receiver: DevtoolsReceiver,
    hovered_node: HoveredNode,
}

impl PartialEq for DevToolsProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[allow(non_snake_case)]
pub fn DevTools(props: DevToolsProps) -> Element {
    let theme = use_init_theme(|| DARK_THEME);
    use_init_radio_station::<DevtoolsState, DevtoolsChannel>(|| DevtoolsState {
        hovered_node: props.hovered_node.clone(),
        devtools_receiver: props.devtools_receiver.clone(),
        devtools_tree: HashSet::default(),
    });

    let theme = theme.read();
    let color = &theme.body.color;

    rsx!(
        rect {
            width: "fill",
            height: "fill",
            color: "{color}",
            Router::<Route> { }
        }
    )
}

#[component]
#[allow(non_snake_case)]
pub fn DevtoolsBar() -> Element {
    rsx!(
        TabsBar {
            Link {
                to: Route::DOMInspector { },
                ActivableRoute {
                    route: Route::DOMInspector { },
                    Tab {
                        label {
                            "Elements"
                        }
                    }
                }
            }
        }
        Outlet::<Route> {}
    )
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(DevtoolsBar)]
        #[nest("/")]
            #[layout(LayoutForDOMInspector)]
                #[route("/")]
                DOMInspector  {},
                #[nest("/node/:node_id")]
                    #[layout(LayoutForNodeInspector)]
                        #[route("/style")]
                        NodeInspectorStyle { node_id: String },
                        #[route("/layout")]
                        NodeInspectorLayout { node_id: String },
                    #[end_layout]
                #[end_nest]
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[route("/..route")]
    PageNotFound { },
}

impl Route {
    pub fn get_node_id(&self) -> Option<NodeId> {
        match self {
            Self::NodeInspectorStyle { node_id } | Self::NodeInspectorLayout { node_id } => {
                Some(NodeId::deserialize(node_id))
            }
            _ => None,
        }
    }
}

#[allow(non_snake_case)]
#[component]
fn PageNotFound() -> Element {
    rsx!(
        label {
            "Page not found."
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn LayoutForNodeInspector(node_id: String) -> Element {
    rsx!(
        rect {
            overflow: "clip",
            width: "100%",
            height: "50%",
            TabsBar {
                Link {
                    to: Route::NodeInspectorStyle { node_id: node_id.clone() },
                    ActivableRoute {
                        route: Route::NodeInspectorStyle { node_id: node_id.clone() },
                        Tab {
                            label {
                                "Style"
                            }
                        }
                    }
                }
                Link {
                    to: Route::NodeInspectorLayout { node_id: node_id.clone() },
                    ActivableRoute {
                        route: Route::NodeInspectorLayout { node_id },
                        Tab {
                            label {
                                "Layout"
                            }
                        }
                    }
                }
            }
            Outlet::<Route> {}
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn LayoutForDOMInspector() -> Element {
    let route = use_route::<Route>();
    let platform = use_platform();
    let mut radio = use_radio(DevtoolsChannel::Global);
    use_hook(move || {
        spawn(async move {
            let mut devtools_receiver = radio.read().devtools_receiver.clone();
            loop {
                devtools_receiver
                    .changed()
                    .await
                    .expect("Failed while waiting for DOM changes.");

                radio.write_channel(DevtoolsChannel::UpdatedDOM);
            }
        });
    });

    let selected_node_id = route.get_node_id();

    let is_expanded_vertical = selected_node_id.is_some();

    let height = if is_expanded_vertical {
        "calc(50% - 35)"
    } else {
        "fill"
    };

    rsx!(
        NodesTree {
            height,
            selected_node_id,
            onselected: move |node_id: NodeId| {
                if let Some(hovered_node) = &radio.read().hovered_node.as_ref() {
                    hovered_node.lock().unwrap().replace(node_id);
                    platform.request_animation_frame();
                }
            }
        }
        Outlet::<Route> {}
    )
}

#[allow(non_snake_case)]
#[component]
fn DOMInspector() -> Element {
    None
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
