use std::{
    collections::{
        HashMap,
        HashSet,
    },
    sync::Arc,
    time::Duration,
};

use dioxus_radio::prelude::*;
use freya::prelude::*;
use freya_core::animation_clock::AnimationClock;
use freya_devtools::{
    IncomingMessage,
    IncomingMessageAction,
    OutgoingMessage,
    OutgoingMessageAction,
};
use freya_native_core::NodeId;
use freya_router::prelude::*;
use futures_util::{
    SinkExt,
    StreamExt,
};
use state::{
    DevtoolsChannel,
    DevtoolsState,
};

mod components;
mod hooks;
mod node;
mod property;
mod state;
mod tabs;

use tabs::{
    computed_layout::*,
    font_style::*,
    layout::*,
    misc::*,
    style::*,
    svg::*,
    tree::*,
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        self,
        Message,
    },
};

fn main() {
    launch_with_params(app, "Freya Devtools", (700., 500.))
}

pub fn app() -> Element {
    use_init_theme(|| DARK_THEME);
    use_init_radio_station::<DevtoolsState, DevtoolsChannel>(|| DevtoolsState {
        nodes: HashMap::new(),
        expanded_nodes: HashSet::default(),
        client: Arc::default(),
        animation_speed: AnimationClock::DEFAULT_SPEED / AnimationClock::MAX_SPEED * 100.,
    });
    let radio = use_radio(DevtoolsChannel::Global);

    use_future(move || async move {
        async fn connect(
            mut radio: dioxus_radio::prelude::Radio<DevtoolsState, DevtoolsChannel>,
        ) -> Result<(), tungstenite::Error> {
            let (ws_stream, _) = connect_async("ws://[::1]:7354").await?;

            let (write, read) = ws_stream.split();

            radio.write_silently().client.lock().await.replace(write);

            read.for_each(move |message| async move {
                if let Ok(message) = message {
                    if let Ok(text) = message.into_text() {
                        if let Ok(outgoing) = serde_json::from_str::<OutgoingMessage>(&text) {
                            match outgoing.action {
                                OutgoingMessageAction::Update { window_id, nodes } => {
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
            Router::<Route> {
                config: ||RouterConfig::<Route>::default().with_initial_path(Route::DOMInspector { })
             }
        }
    )
}

#[component]
pub fn NavBar() -> Element {
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
            Link {
                to: Route::Misc { },
                ActivableRoute {
                    route: Route::Misc { },
                    Tab {
                        label {
                            "Misc"
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
    #[layout(NavBar)]
        #[route("/misc")]
        Misc {},
        #[layout(LayoutForDOMInspector)]
            #[nest("/inspector")]
                #[route("/")]
                DOMInspector {},
                #[nest("/node/:node_id/:window_id")]
                    #[layout(LayoutForNodeInspector)]
                        #[route("/style")]
                        NodeInspectorStyle { node_id: NodeId, window_id: u64 },
                        #[route("/layout")]
                        NodeInspectorLayout { node_id: NodeId, window_id: u64 },
                        #[route("/computed-layout")]
                        NodeInspectorComputedLayout { node_id: NodeId, window_id: u64 },
                        #[route("/font-style")]
                        NodeInspectorFontStyle { node_id: NodeId, window_id: u64 },
                        #[route("/svg")]
                        NodeInspectorSvg { node_id: NodeId, window_id: u64 },
}

impl Route {
    pub fn node_id(&self) -> Option<NodeId> {
        match self {
            Self::NodeInspectorStyle { node_id, .. }
            | Self::NodeInspectorLayout { node_id, .. }
            | Self::NodeInspectorComputedLayout { node_id, .. } => Some(*node_id),
            Self::NodeInspectorFontStyle { node_id, .. } => Some(*node_id),
            Self::NodeInspectorSvg { node_id, .. } => Some(*node_id),
            _ => None,
        }
    }

    pub fn window_id(&self) -> Option<u64> {
        match self {
            Self::NodeInspectorStyle { window_id, .. }
            | Self::NodeInspectorLayout { window_id, .. }
            | Self::NodeInspectorComputedLayout { window_id, .. } => Some(*window_id),
            Self::NodeInspectorFontStyle { window_id, .. } => Some(*window_id),
            Self::NodeInspectorSvg { window_id, .. } => Some(*window_id),
            _ => None,
        }
    }
}

#[component]
fn LayoutForNodeInspector(node_id: NodeId, window_id: u64) -> Element {
    rsx!(
        rect {
            overflow: "clip",
            width: "fill",
            height: "fill",
            background: "rgb(30, 30, 30)",
            margin: "0 10 10 10",
            corner_radius: "16",
            padding: "6",
            spacing: "6",
            ScrollView {
                height: "auto",
                direction: "horizontal",
                Link {
                    to: Route::NodeInspectorLayout { node_id, window_id },
                    ActivableRoute {
                        route: Route::NodeInspectorLayout { node_id, window_id },
                        BottomTab {
                            label {
                                max_lines: "1",
                                "Layout"
                            }
                        }
                    }
                }
                Link {
                    to: Route::NodeInspectorStyle { node_id, window_id },
                    ActivableRoute {
                        route: Route::NodeInspectorStyle { node_id, window_id },
                        BottomTab {
                            label {
                                max_lines: "1",
                                "Style"
                            }
                        }
                    }
                }
                Link {
                    to: Route::NodeInspectorComputedLayout { node_id, window_id },
                    ActivableRoute {
                        route: Route::NodeInspectorComputedLayout { node_id, window_id },
                        BottomTab {
                            label {
                                max_lines: "1",
                                "Computed Layout"
                            }
                        }
                    }
                }
                Link {
                    to: Route::NodeInspectorFontStyle { node_id, window_id },
                    ActivableRoute {
                        route: Route::NodeInspectorFontStyle { node_id, window_id },
                        BottomTab {
                            label {
                                max_lines: "1",
                                "Font Style"
                            }
                        }
                    }
                }
                Link {
                    to: Route::NodeInspectorSvg { node_id, window_id },
                    ActivableRoute {
                        route: Route::NodeInspectorSvg { node_id, window_id },
                        BottomTab {
                            label {
                                max_lines: "1",
                                "SVG"
                            }
                        }
                    }
                }
            }
            Outlet::<Route> {}
        }
    )
}

#[component]
fn LayoutForDOMInspector() -> Element {
    let route = use_route::<Route>();
    let radio = use_radio(DevtoolsChannel::Global);

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
                        onselected: move |(window_id, node_id): (u64, NodeId)| {
                            let message = Message::Text(
                                serde_json::to_string(&IncomingMessage {
                                    action: IncomingMessageAction::HighlightNode { window_id, node_id },
                                }).unwrap()
                                .into()
                            );
                            let client = radio.read().client.clone();
                            spawn(async move {
                                client.lock().await.as_mut().unwrap().send(message).await.ok();
                            });
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

#[component]
fn DOMInspector() -> Element {
    Ok(VNode::placeholder())
}
