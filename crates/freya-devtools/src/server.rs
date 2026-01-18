use std::{
    collections::HashMap,
    sync::{
        Arc,
        Mutex,
        atomic::{
            AtomicU32,
            Ordering,
        },
    },
};

use anyhow::bail;
use async_tungstenite::accept_async;
use freya_core::integration::{
    NodeId,
    UserEvent,
};
use freya_winit::{
    plugins::PluginHandle,
    renderer::{
        NativeEvent,
        NativeWindowEvent,
        NativeWindowEventAction,
    },
};
use futures_util::stream::StreamExt;
use smol::net::TcpListener;
use tungstenite::protocol::Message;

use crate::{
    IncomingMessage,
    OutgoingMessage,
    OutgoingMessageAction,
    SharedWebsockets,
    WindowState,
    incoming::IncomingMessageAction,
};

static WEBSOCKET_ID: AtomicU32 = AtomicU32::new(0);

async fn handle_connection(
    id: u32,
    stream: smol::net::TcpStream,
    windows: Arc<Mutex<HashMap<u64, WindowState>>>,
    websockets: SharedWebsockets,
    highlighted_node: Arc<Mutex<Option<NodeId>>>,
    hovered_node: Arc<Mutex<Option<NodeId>>>,
    plugin_handle: PluginHandle,
) -> anyhow::Result<()> {
    let ws_stream = accept_async(stream).await.unwrap();
    let (mut write, mut read) = ws_stream.split();

    let windows_snapshot = windows.lock().unwrap().clone();
    for (window_id, WindowState { nodes, .. }) in windows_snapshot {
        let message = Message::Text(
            serde_json::to_string(&OutgoingMessage {
                action: OutgoingMessageAction::Update { window_id, nodes },
            })?
            .into(),
        );

        // Send nodes snapshot
        write.send(message).await?;
    }

    websockets.lock().await.insert(id, write);

    while let Some(Ok(msg)) = read.next().await {
        match msg {
            Message::Text(msg) => {
                let incoming = serde_json::from_str::<IncomingMessage>(msg.as_str());

                if let Ok(incoming) = incoming {
                    match incoming.action {
                        IncomingMessageAction::HighlightNode { window_id, node_id } => {
                            highlighted_node.lock().unwrap().replace(node_id);
                            plugin_handle.send_event_loop_event(NativeEvent::Window(
                                NativeWindowEvent {
                                    window_id: window_id.into(),
                                    action: NativeWindowEventAction::User(UserEvent::RequestRedraw),
                                },
                            ));
                        }
                        IncomingMessageAction::HoverNode { window_id, node_id } => {
                            *hovered_node.lock().unwrap() = node_id;
                            plugin_handle.send_event_loop_event(NativeEvent::Window(
                                NativeWindowEvent {
                                    window_id: window_id.into(),
                                    action: NativeWindowEventAction::User(UserEvent::RequestRedraw),
                                },
                            ));
                        }
                        IncomingMessageAction::SetSpeedTo { speed } => {
                            for WindowState {
                                animation_clock, ..
                            } in windows.lock().unwrap().values()
                            {
                                animation_clock.set_speed(speed);
                            }
                        }
                    }
                } else {
                    bail!("Failed to parse.");
                }
            }
            Message::Close(_) => {
                bail!("Closed.");
            }
            _ => {}
        }
    }
    Ok(())
}

pub async fn run_server(
    windows: Arc<Mutex<HashMap<u64, WindowState>>>,
    websockets: SharedWebsockets,
    highlighted_node: Arc<Mutex<Option<NodeId>>>,
    hovered_node: Arc<Mutex<Option<NodeId>>>,
    plugin_handle: PluginHandle,
) -> anyhow::Result<()> {
    println!("Running the Devtools Server in [::1]:7354");

    let listener = TcpListener::bind("[::1]:7354").await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let windows = windows.clone();
        let websockets = websockets.clone();
        let highlighted_node = highlighted_node.clone();
        let hovered_node = hovered_node.clone();
        let plugin_handle = plugin_handle.clone();
        smol::spawn(async move {
            let id = WEBSOCKET_ID.fetch_add(1, Ordering::Relaxed);
            if let Err(err) = handle_connection(
                id,
                stream,
                windows,
                websockets.clone(),
                highlighted_node,
                hovered_node,
                plugin_handle,
            )
            .await
            {
                println!("Disconnected: {err:?}");
            }
            websockets.lock().await.remove(&id);
        })
        .detach();
    }
}
