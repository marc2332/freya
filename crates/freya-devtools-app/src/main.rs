use std::{
    collections::{
        HashMap,
        HashSet,
    },
    sync::Arc,
    time::Duration,
};

use freya::prelude::*;
use freya_core::integration::NodeId;
use freya_devtools::{
    IncomingMessage,
    IncomingMessageAction,
    OutgoingMessage,
    OutgoingMessageAction,
};
use freya_radio::prelude::*;
use freya_router::prelude::*;
use futures_util::StreamExt;
use smol::{
    Timer,
    net::TcpStream,
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

use async_tungstenite::tungstenite::protocol::Message;
use tabs::{
    computed_layout::*,
    layout::*,
    misc::*,
    style::*,
    text_style::*,
    tree::*,
};

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_title("Freya Devtools")
                .with_size(1200., 700.),
        ),
    )
}

pub fn app() -> Element {
    use_init_theme(|| DARK_THEME);
    use_init_radio_station::<DevtoolsState, DevtoolsChannel>(|| DevtoolsState {
        nodes: HashMap::new(),
        expanded_nodes: HashSet::default(),
        client: Arc::default(),
        animation_speed: AnimationClock::DEFAULT_SPEED / AnimationClock::MAX_SPEED * 100.,
    });
    let mut radio = use_radio(DevtoolsChannel::Global);

    use_hook(move || {
        spawn(async move {
            async fn connect(
                mut radio: Radio<DevtoolsState, DevtoolsChannel>,
            ) -> Result<(), tungstenite::Error> {
                let tcp_stream = TcpStream::connect("[::1]:7354").await?;
                let (ws_stream, _response) =
                    async_tungstenite::client_async("ws://[::1]:7354", tcp_stream).await?;

                let (write, read) = ws_stream.split();

                radio.write_silently().client.lock().await.replace(write);

                read.for_each(move |message| async move {
                    if let Ok(message) = message
                        && let Ok(text) = message.into_text()
                        && let Ok(outgoing) = serde_json::from_str::<OutgoingMessage>(&text)
                    {
                        match outgoing.action {
                            OutgoingMessageAction::Update { window_id, nodes } => {
                                radio
                                    .write_channel(DevtoolsChannel::UpdatedTree)
                                    .nodes
                                    .insert(window_id, nodes);
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
                radio
                    .write_channel(DevtoolsChannel::UpdatedTree)
                    .nodes
                    .clear();
                Timer::after(Duration::from_secs(2)).await;
            }
        })
    });

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .color(Color::WHITE)
        .background((15, 15, 15))
        .child(router(|| {
            RouterConfig::<Route>::default().with_initial_path(Route::TreeInspector {})
        }))
        .into()
}

#[derive(PartialEq)]
struct NavBar;
impl Render for NavBar {
    fn render(&self) -> Element {
        SideBar::new()
            .width(Size::px(100.))
            .bar(
                rect()
                    .child(ActivableRoute::new(
                        Route::TreeInspector {},
                        Link::new(Route::TreeInspector {}).child(SideBarItem::new().child("Tree")),
                    ))
                    .child(ActivableRoute::new(
                        Route::Misc {},
                        Link::new(Route::Misc {}).child(SideBarItem::new().child("Misc")),
                    )),
            )
            .content(rect().padding(Gaps::new_all(8.)).child(outlet::<Route>()))
            .into()
    }
}
#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
        #[route("/misc")]
        Misc {},
        #[layout(LayoutForTreeInspector)]
            #[nest("/inspector")]
                #[route("/")]
                TreeInspector {},
                #[nest("/node/:node_id/:window_id")]
                    #[layout(LayoutForNodeInspector)]
                        #[route("/style")]
                        NodeInspectorStyle { node_id: NodeId, window_id: u64 },
                        #[route("/layout")]
                        NodeInspectorLayout { node_id: NodeId, window_id: u64 },
                        #[route("/computed-layout")]
                        NodeInspectorComputedLayout { node_id: NodeId, window_id: u64 },
                        #[route("/text-style")]
                        NodeInspectorTextStyle { node_id: NodeId, window_id: u64 },
}

impl Route {
    pub fn node_id(&self) -> Option<NodeId> {
        match self {
            Self::NodeInspectorStyle { node_id, .. }
            | Self::NodeInspectorComputedLayout { node_id, .. }
            | Self::NodeInspectorLayout { node_id, .. }
            | Self::NodeInspectorTextStyle { node_id, .. } => Some(*node_id),
            _ => None,
        }
    }

    pub fn window_id(&self) -> Option<u64> {
        match self {
            Self::NodeInspectorStyle { window_id, .. }
            | Self::NodeInspectorComputedLayout { window_id, .. }
            | Self::NodeInspectorLayout { window_id, .. }
            | Self::NodeInspectorTextStyle { window_id, .. } => Some(*window_id),
            _ => None,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
struct LayoutForNodeInspector {
    window_id: u64,
    node_id: NodeId,
}

impl Render for LayoutForNodeInspector {
    fn render(&self) -> Element {
        let LayoutForNodeInspector { window_id, node_id } = *self;

        rect()
            .expanded()
            .child(
                ScrollView::new()
                    .direction(Direction::Horizontal)
                    .height(Size::auto())
                    .child(ActivableRoute::new(
                        Route::NodeInspectorStyle { node_id, window_id },
                        Link::new(Route::NodeInspectorStyle { node_id, window_id })
                            .child(FloatingTab::new().child("Style")),
                    ))
                    .child(ActivableRoute::new(
                        Route::NodeInspectorLayout { node_id, window_id },
                        Link::new(Route::NodeInspectorLayout { node_id, window_id })
                            .child(FloatingTab::new().child("Layout")),
                    ))
                    .child(ActivableRoute::new(
                        Route::NodeInspectorTextStyle { node_id, window_id },
                        Link::new(Route::NodeInspectorTextStyle { node_id, window_id })
                            .child(FloatingTab::new().child("Text Style")),
                    ))
                    .child(ActivableRoute::new(
                        Route::NodeInspectorComputedLayout { node_id, window_id },
                        Link::new(Route::NodeInspectorComputedLayout { node_id, window_id })
                            .child(FloatingTab::new().child("Computed Layout")),
                    )),
            )
            .child(rect().padding((6., 0.)).child(outlet::<Route>()))
            .into()
    }
}

#[derive(PartialEq)]
struct LayoutForTreeInspector;

impl Render for LayoutForTreeInspector {
    fn render(&self) -> Element {
        let route = use_route::<Route>();
        let radio = use_radio(DevtoolsChannel::Global);

        let selected_node_id = route.node_id();
        let selected_window_id = route.window_id();

        let is_expanded_vertical = selected_node_id.is_some();

        ResizableContainer::new()
            .direction(Direction::Horizontal)
            .panel(
                ResizablePanel::new(60.).child(rect().padding(10.).child(NodesTree {
                    selected_node_id,
                    selected_window_id,
                    on_selected: EventHandler::new(move |(window_id, node_id)| {
                        let message = Message::Text(
                            serde_json::to_string(&IncomingMessage {
                                action: IncomingMessageAction::HighlightNode { window_id, node_id },
                            })
                            .unwrap()
                            .into(),
                        );
                        let client = radio.read().client.clone();
                        spawn(async move {
                            client
                                .lock()
                                .await
                                .as_mut()
                                .unwrap()
                                .send(message)
                                .await
                                .ok();
                        });
                    }),
                })),
            )
            .panel(is_expanded_vertical.then(|| ResizablePanel::new(40.).child(outlet::<Route>())))
            .into()
    }
}

#[derive(PartialEq)]
struct TreeInspector;

impl Render for TreeInspector {
    fn render(&self) -> Element {
        rect().into()
    }
}
