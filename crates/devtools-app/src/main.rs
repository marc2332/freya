use std::{
    collections::{
        HashMap,
        HashSet,
    },
    time::Duration,
};

use dioxus_radio::prelude::*;
use freya::prelude::*;
use freya_devtools::{
    Outgoing,
    OutgoingNotification,
};
use freya_native_core::NodeId;
use freya_router::prelude::*;
use futures_util::StreamExt;
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
use tokio_tungstenite::{
    connect_async,
    tungstenite,
};

fn main() {
    launch_with_params(app, "Freya Devtools", (700., 500.))
}

pub fn app() -> Element {
    use_init_theme(|| DARK_THEME);
    use_init_radio_station::<DevtoolsState, DevtoolsChannel>(|| DevtoolsState {
        nodes: HashMap::new(),
        expanded_nodes: HashSet::default(),
    });
    let radio = use_radio(DevtoolsChannel::Global);

    use_future(move || async move {
        async fn connect(
            mut radio: dioxus_radio::prelude::Radio<DevtoolsState, DevtoolsChannel>,
        ) -> Result<(), tungstenite::Error> {
            let (ws_stream, _) = connect_async("ws://[::1]:3000").await?;

            let (_write, read) = ws_stream.split();

            read.for_each(move |message| async move {
                if let Ok(message) = message {
                    if let Ok(text) = message.into_text() {
                        if let Ok(outgoing) = serde_json::from_str::<Outgoing>(&text) {
                            match outgoing.notification {
                                OutgoingNotification::Update { window_id, nodes } => {
                                    radio
                                        .write_channel(DevtoolsChannel::UpdatedDOM)
                                        .nodes
                                        .insert(window_id, nodes);
                                }
                            }
                        }
                    }
                }
            })
            .await;

            Ok(())
        }

        loop {
            println!("Connecting to server...");
            connect(radio).await.ok();
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });

    rsx!(
        Body {
            Router::<Route> { }
        }
    )
}

#[component]
pub fn DevtoolsBar() -> Element {
    rsx!(
        Tabsbar {
            Link {
                to: Route::DOMInspector { },
                ActivableRoute {
                    route: Route::DOMInspector { },
                    Tab {
                        label {
                            "Tree Inspector"
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
        #[nest("/node/:node_id/:window_id")]
            #[layout(LayoutForNodeInspector)]
                #[route("/style")]
                NodeInspectorStyle { node_id: NodeId, window_id: u64 },
                #[route("/layout")]
                NodeInspectorLayout { node_id: NodeId, window_id: u64 },
}

impl Route {
    pub fn node_id(&self) -> Option<NodeId> {
        match self {
            Self::NodeInspectorStyle { node_id, .. }
            | Self::NodeInspectorLayout { node_id, .. } => Some(*node_id),
            _ => None,
        }
    }

    pub fn window_id(&self) -> Option<u64> {
        match self {
            Self::NodeInspectorStyle { window_id, .. }
            | Self::NodeInspectorLayout { window_id, .. } => Some(*window_id),
            _ => None,
        }
    }
}

#[allow(non_snake_case)]
#[component]
fn LayoutForNodeInspector(node_id: NodeId, window_id: u64) -> Element {
    let navigator = use_navigator();

    rsx!(
        rect {
            overflow: "clip",
            width: "fill",
            height: "fill",
            background: "rgb(30, 30, 30)",
            margin: "0 10 10 10",
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
                        to: Route::NodeInspectorStyle { node_id, window_id },
                        ActivableRoute {
                            route: Route::NodeInspectorStyle { node_id, window_id },
                            BottomTab {
                                label {
                                    "Style"
                                }
                            }
                        }
                    }
                    Link {
                        to: Route::NodeInspectorLayout { node_id, window_id },
                        ActivableRoute {
                            route: Route::NodeInspectorLayout { node_id, window_id },
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

    let selected_node_id = route.node_id();
    let selected_window_id = route.window_id();

    let is_expanded_vertical = selected_node_id.is_some();

    rsx!(
        ResizableContainer {
            direction: "horizontal",
            ResizablePanel {
                initial_size: 40.,
                rect {
                    padding: "10",
                    NodesTree {
                        selected_node_id,
                        selected_window_id,
                        onselected: move |_node_id: NodeId| {
                            // platform.send(EventLoopMessage::RequestFullRerender).ok();
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
    )
}

#[allow(non_snake_case)]
#[component]
fn DOMInspector() -> Element {
    Ok(VNode::placeholder())
}
