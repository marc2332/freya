use std::collections::HashSet;

use dioxus::prelude::*;
use dioxus_radio::prelude::*;
use freya_components::*;
use freya_core::event_loop_messages::EventLoopMessage;
use freya_elements as dioxus_elements;
use freya_hooks::{
    use_applied_theme,
    use_init_theme,
    use_platform,
    DARK_THEME,
};
use freya_native_core::NodeId;
use freya_router::prelude::*;
use freya_winit::devtools::{
    DevtoolsReceiver,
    HighlightedNode,
};
use state::{
    DevtoolsChannel,
    DevtoolsState,
};

mod hooks;
mod node;
mod property;
mod state;
mod tabs;

use tabs::{
    layout::*,
    style::*,
    tree::*,
};

#[derive(Props, Clone)]
pub struct DevtoolsProps {
    children: Element,
    devtools_receiver: DevtoolsReceiver,
    highlighted_node: HighlightedNode,
}

impl PartialEq for DevtoolsProps {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[allow(non_snake_case)]
pub fn DevtoolsView(
    DevtoolsProps {
        children,
        devtools_receiver,
        highlighted_node: hovered_node,
    }: DevtoolsProps,
) -> Element {
    rsx!(
        ResizableContainer {
            direction: "horizontal",
            ResizablePanel {
                initial_size: 75.,
                {children}
            }
            ResizablePanel {
                initial_size: 25.,
                min_size: 10.,
                rect {
                    background: "rgb(40, 40, 40)",
                    height: "fill",
                    width: "fill",
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
    hovered_node: HighlightedNode,
}

impl PartialEq for DevToolsProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[allow(non_snake_case)]
pub fn DevTools(props: DevToolsProps) -> Element {
    use_init_theme(|| DARK_THEME);
    use_init_radio_station::<DevtoolsState, DevtoolsChannel>(|| DevtoolsState {
        hovered_node: props.hovered_node.clone(),
        devtools_receiver: props.devtools_receiver.clone(),
        devtools_tree: HashSet::default(),
    });

    let theme = use_applied_theme!(None, body);
    let color = &theme.color;

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
        Tabsbar {
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

        NativeRouter {
            Outlet::<Route> {}
        }
    )
}

#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(DevtoolsBar)]
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
    let navigator = use_navigator();

    rsx!(
        rect {
            overflow: "clip",
            width: "fill",
            height: "fill",
            background: "rgb(30, 30, 30)",
            margin: "10",
            corner_radius: "16",
            cross_align: "center",
            padding: "6",
            spacing: "6",
            rect {
                direction: "horizontal",
                width: "fill",
                main_align: "space-between",
                rect {
                    direction: "horizontal",
                    Link {
                        to: Route::NodeInspectorStyle { node_id: node_id.clone() },
                        ActivableRoute {
                            route: Route::NodeInspectorStyle { node_id: node_id.clone() },
                            BottomTab {
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
                            BottomTab {
                                label {
                                    "Layout"
                                }
                            }
                        }
                    }
                }
                BottomTab {
                    onpress: move |_| {navigator.replace(Route::DOMInspector {});},
                    label {
                        "Close"
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

    rsx!(
        rect {
            height: "fill",
            ResizableContainer {
                direction: "vertical",
                ResizablePanel {
                    initial_size: 40.,
                    rect {
                        padding: "15",
                        NodesTree {
                            selected_node_id,
                            onselected: move |node_id: NodeId| {
                                radio.read().hovered_node.lock().unwrap().replace(node_id);
                                platform.send(EventLoopMessage::RequestFullRerender).ok();
                            }
                        }
                    }
                }
                if is_expanded_vertical {
                    ResizablePanel {
                        initial_size: 60.,
                        Outlet::<Route> {}
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn DOMInspector() -> Element {
    Ok(VNode::placeholder())
}
